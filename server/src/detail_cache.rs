use super::item::{Detail, Item};
use super::lua_value::{serialize, Parser, Table};
use super::util::{make_local_one_shot, spawn, LocalReceiver, LocalSender};
use abort_on_drop::ChildTask;
use flexstr::{local_fmt, local_str, LocalStr};
use fnv::FnvHashMap;
use std::{
    cell::RefCell,
    collections::hash_map::Entry,
    mem::take,
    rc::{Rc, Weak},
};

enum DetailState {
    Resolved(Rc<Detail>),
    Resolving { _resolver: ChildTask<()>, wait_queue: Vec<LocalSender<Rc<Detail>>> },
}

pub enum DetailResult<'a> {
    Resolved(&'a Rc<Detail>),
    Resolving { sender: Option<LocalSender<(Rc<Item>, Rc<Detail>)>>, receiver: LocalReceiver<Rc<Detail>> },
}

pub struct DetailCache {
    path: Option<LocalStr>,
    state: FnvHashMap<Rc<Item>, DetailState>,
    weak: Weak<RefCell<DetailCache>>,
}

async fn resolver_main(
    weak: Weak<RefCell<DetailCache>>,
    expected: Rc<Item>,
    receiver: LocalReceiver<(Rc<Item>, Rc<Detail>)>,
) {
    let result = receiver.await;
    if let Some(this) = weak.upgrade() {
        let mut this = this.borrow_mut();
        let e = match result {
            Ok((actual, detail)) => {
                let correct = actual == expected;
                this.insert(actual, detail);
                if correct {
                    return;
                }
                local_str!("data race detected on inventory")
            }
            Err(e) => e,
        };
        if let Entry::Occupied(mut state) = this.state.entry(expected) {
            if let DetailState::Resolving { wait_queue, .. } = state.get_mut() {
                for sender in take(wait_queue) {
                    sender.send(Err(e.clone()))
                }
                state.remove();
            }
        } else {
            unreachable!()
        }
    }
}

fn load(path: &str) -> Result<FnvHashMap<Rc<Item>, DetailState>, LocalStr> {
    let data = std::fs::read(path).map_err(|e| local_fmt!("{}", e))?;
    let mut result = FnvHashMap::default();
    Parser::new().shift(&data, &mut |value| {
        let mut table = Table::try_from(value)?;
        let item = Item::parse_part(&mut table)?;
        let detail = Detail::parse(table)?;
        result.insert(item, DetailState::Resolved(detail));
        Ok(())
    })?;
    Result::<FnvHashMap<Rc<Item>, DetailState>, LocalStr>::Ok(result)
}

impl DetailCache {
    pub fn new(path: Option<LocalStr>) -> Rc<RefCell<Self>> {
        let state = if let Some(path) = &path {
            match load(&**path) {
                Ok(state) => {
                    println!("detail_cache loaded with {} entries", state.len());
                    state
                }
                Err(e) => {
                    println!("detail_cache not loaded: {}", e);
                    FnvHashMap::default()
                }
            }
        } else {
            FnvHashMap::default()
        };
        Rc::new_cyclic(|weak| RefCell::new(Self { path, state, weak: weak.clone() }))
    }

    pub fn query(&mut self, item: &Rc<Item>) -> DetailResult {
        match self.state.entry(item.clone()) {
            Entry::Vacant(state) => {
                let (sender, producer) = make_local_one_shot();
                let (consumer, receiver) = make_local_one_shot();
                state.insert(DetailState::Resolving {
                    _resolver: spawn(resolver_main(self.weak.clone(), item.clone(), producer)),
                    wait_queue: vec![consumer],
                });
                DetailResult::Resolving { sender: Some(sender), receiver: receiver }
            }
            Entry::Occupied(state) => match state.into_mut() {
                DetailState::Resolved(x) => DetailResult::Resolved(x),
                DetailState::Resolving { wait_queue, .. } => {
                    let (sender, receiver) = make_local_one_shot();
                    wait_queue.push(sender);
                    DetailResult::Resolving { sender: None, receiver }
                }
            },
        }
    }

    fn insert(&mut self, item: Rc<Item>, detail: Rc<Detail>) {
        match self.state.entry(item) {
            Entry::Vacant(state) => {
                state.insert(DetailState::Resolved(detail));
            }
            Entry::Occupied(mut state) => {
                if let DetailState::Resolving { wait_queue, .. } = state.get_mut() {
                    for sender in take(wait_queue) {
                        sender.send(Ok(detail.clone()))
                    }
                    state.insert(DetailState::Resolved(detail));
                }
            }
        }
    }
}

impl Drop for DetailCache {
    fn drop(&mut self) {
        if let Some(path) = &self.path {
            let mut data = Vec::new();
            for (item, detail) in &self.state {
                if let DetailState::Resolved(detail) = detail {
                    let mut table = detail.encode();
                    item.encode(&mut table);
                    serialize(&table.into(), &mut data);
                }
            }
            if let Err(e) = std::fs::write(&**path, data) {
                println!("failed to save detail_cache: {}", e)
            } else {
                println!("saved detail_cache with {} entries", self.state.len());
            }
        }
    }
}
