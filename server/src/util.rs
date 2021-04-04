use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
    task::{Context, Poll, Waker},
};
use tokio::task::{spawn_local, JoinHandle};

pub struct AbortOnDrop<T>(Option<JoinHandle<T>>);

impl<T> Drop for AbortOnDrop<T> {
    fn drop(&mut self) {
        if let Some(ref x) = self.0 {
            x.abort()
        }
    }
}

impl<T> AbortOnDrop<T> {
    pub async fn into_future(mut self) -> T { self.0.take().unwrap().await.unwrap() }
}

pub fn spawn<T: 'static>(future: impl Future<Output = T> + 'static) -> AbortOnDrop<T> {
    AbortOnDrop(Some(spawn_local(future)))
}

struct LocalOneShotState<T> {
    result: Option<Result<T, String>>,
    waker: Option<Waker>,
}

fn send<T>(state: Weak<RefCell<LocalOneShotState<T>>>, result: Result<T, String>) {
    if let Some(state) = state.upgrade() {
        let mut state = state.borrow_mut();
        state.result = Some(result);
        if let Some(waker) = state.waker.take() {
            waker.wake()
        }
    }
}

pub struct LocalReceiver<T>(Rc<RefCell<LocalOneShotState<T>>>);

impl<T> Future for LocalReceiver<T> {
    type Output = Result<T, String>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut this = this.0.borrow_mut();
        if let Some(result) = this.result.take() {
            Poll::Ready(result)
        } else {
            this.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct LocalSender<T>(Option<Weak<RefCell<LocalOneShotState<T>>>>);

impl<T> Drop for LocalSender<T> {
    fn drop(&mut self) {
        if let Some(state) = self.0.take() {
            send(state, Err("sender died".to_owned()))
        }
    }
}

impl<T> LocalSender<T> {
    pub fn send(mut self, result: Result<T, String>) { send(self.0.take().unwrap(), result) }
}

pub fn make_local_one_shot<T>() -> (LocalSender<T>, LocalReceiver<T>) {
    let state = Rc::new(RefCell::new(LocalOneShotState { result: None, waker: None }));
    (LocalSender(Some(Rc::downgrade(&state))), LocalReceiver(state))
}

macro_rules! upgrade_mut {
    ($e:expr, $v:ident) => {
        let $v = $e.upgrade().unwrap();
        let mut $v = $v.borrow_mut();
        let $v = &mut *$v;
    };
}
