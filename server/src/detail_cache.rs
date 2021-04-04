use super::item::{Item, ItemDetail};
use super::util::{make_local_one_shot, spawn, AbortOnDrop, LocalReceiver, LocalSender};
use fnv::FnvHashMap;
use std::{
    cell::RefCell,
    collections::hash_map::Entry,
    mem::take,
    rc::{Rc, Weak},
};

enum DetailState {
    Resolved(Rc<ItemDetail>),
    Resolving { _resolver: AbortOnDrop<()>, wait_queue: Vec<LocalSender<Rc<ItemDetail>>> },
}

pub enum DetailResult<'a> {
    Resolved(&'a Rc<ItemDetail>),
    Resolving(LocalReceiver<Rc<ItemDetail>>),
    Unresolved(LocalSender<Rc<ItemDetail>>),
}

pub struct DetailCache {
    state: FnvHashMap<Rc<Item>, DetailState>,
    weak: Weak<RefCell<DetailCache>>,
}

async fn resolver_main(weak: Weak<RefCell<DetailCache>>, item: Rc<Item>, receiver: LocalReceiver<Rc<ItemDetail>>) {
    let result = receiver.await;
    if let Some(this) = weak.upgrade() {
        let mut this = this.borrow_mut();
        if let Entry::Occupied(mut state) = this.state.entry(item) {
            if let DetailState::Resolving { wait_queue, .. } = state.get_mut() {
                for sender in take(wait_queue) {
                    sender.send(result.clone())
                }
            } else {
                unreachable!()
            }
            if let Ok(x) = result {
                state.insert(DetailState::Resolved(x));
            } else {
                state.remove();
            }
        } else {
            unreachable!()
        }
    }
}

impl DetailCache {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|weak| RefCell::new(Self { state: FnvHashMap::default(), weak: weak.clone() }))
    }

    pub fn get(&mut self, item: &Rc<Item>) -> DetailResult {
        match self.state.entry(item.clone()) {
            Entry::Vacant(state) => {
                let (sender, receiver) = make_local_one_shot();
                state.insert(DetailState::Resolving {
                    _resolver: spawn(resolver_main(self.weak.clone(), item.clone(), receiver)),
                    wait_queue: Vec::new(),
                });
                DetailResult::Unresolved(sender)
            }
            Entry::Occupied(state) => match state.into_mut() {
                DetailState::Resolved(x) => DetailResult::Resolved(x),
                DetailState::Resolving { wait_queue, .. } => {
                    let (sender, receiver) = make_local_one_shot();
                    wait_queue.push(sender);
                    DetailResult::Resolving(receiver)
                }
            },
        }
    }
}
