use std::future::Future;
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

macro_rules! upgrade_mut {
    ($e:expr, $v:ident) => {
        let $v = $e.upgrade().unwrap();
        let mut $v = $v.borrow_mut();
        let $v = &mut *$v;
    };
}
