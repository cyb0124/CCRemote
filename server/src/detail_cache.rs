use super::item::{Detail, Item};
use super::util::{make_local_one_shot, spawn, LocalReceiver, LocalSender};
use abort_on_drop::ChildTask;
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
                "data race detected on inventory".to_owned()
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

impl DetailCache {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|weak| RefCell::new(Self { state: FnvHashMap::default(), weak: weak.clone() }))
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
