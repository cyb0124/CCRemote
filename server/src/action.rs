use super::lua_value::{vec_to_table, Table, Value};
use flexstr::LocalStr;
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

pub trait Action: 'static {
    type Output;
    fn build_request(self, table: &mut Table);
    fn parse_response(response: Value) -> Result<Self::Output, LocalStr>;
}

struct ActionState<T: Action> {
    result: Option<Result<T::Output, LocalStr>>,
    waker: Option<Waker>,
    action: Option<T>,
}

pub trait ActionRequest {
    fn build_request(&mut self, table: &mut Table);
    fn on_fail(&mut self, reason: LocalStr);
    fn on_response(&mut self, result: Value) -> Result<(), LocalStr>;
}

impl<T: Action> ActionRequest for ActionState<T> {
    fn build_request(&mut self, table: &mut Table) { self.action.take().unwrap().build_request(table) }

    fn on_fail(&mut self, reason: LocalStr) {
        self.result = Some(Err(reason));
        if let Some(waker) = self.waker.take() {
            waker.wake()
        }
    }

    fn on_response(&mut self, result: Value) -> Result<(), LocalStr> {
        let result = T::parse_response(result);
        let ret = if let Err(ref e) = result { Err(e.clone()) } else { Ok(()) };
        self.result = Some(result);
        if let Some(waker) = self.waker.take() {
            waker.wake()
        }
        ret
    }
}

pub struct ActionFuture<T: Action>(Rc<RefCell<ActionState<T>>>);

impl<T: Action> Clone for ActionFuture<T> {
    fn clone(&self) -> Self { ActionFuture(self.0.clone()) }
}

impl<T: Action> Future for ActionFuture<T> {
    type Output = Result<T::Output, LocalStr>;
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

impl<T: Action> From<T> for ActionFuture<T> {
    fn from(action: T) -> Self {
        ActionFuture(Rc::new(RefCell::new(ActionState { result: None, waker: None, action: Some(action) })))
    }
}

impl<T: Action> From<ActionFuture<T>> for Rc<RefCell<dyn ActionRequest>> {
    fn from(future: ActionFuture<T>) -> Self { future.0 }
}

#[derive(Clone)]
pub struct Log {
    pub text: LocalStr,
    pub color: u8,
}

impl Action for Log {
    type Output = ();

    fn build_request(self, table: &mut Table) {
        table.insert("o".into(), "l".into());
        table.insert("c".into(), self.color.into());
        table.insert("t".into(), self.text.into());
    }

    fn parse_response(_: Value) -> Result<(), LocalStr> { Ok(()) }
}

pub struct Call {
    pub addr: LocalStr,
    pub args: Vec<Value>,
}

impl Action for Call {
    type Output = Value;

    fn build_request(self, table: &mut Table) {
        table.insert("o".into(), "c".into());
        table.insert("p".into(), self.addr.into());
        table.insert("v".into(), vec_to_table(self.args).into());
    }

    fn parse_response(response: Value) -> Result<Value, LocalStr> { Ok(response) }
}

pub struct RedstoneInput {
    pub side: LocalStr,
    pub addr: Option<LocalStr>,
    pub bit: Option<u8>,
}

impl Action for RedstoneInput {
    type Output = u8;

    fn build_request(self, table: &mut Table) {
        table.insert("o".into(), "i".into());
        table.insert("s".into(), self.side.into());
        if let Some(addr) = self.addr {
            table.insert("p".into(), addr.into());
        }
        if let Some(bit) = self.bit {
            table.insert("b".into(), (1 << bit).into());
        }
    }

    fn parse_response(response: Value) -> Result<u8, LocalStr> { response.try_into() }
}

pub struct RedstoneOutput {
    pub side: LocalStr,
    pub addr: Option<LocalStr>,
    pub bit: Option<u8>,
    pub value: u8,
}

impl Action for RedstoneOutput {
    type Output = ();

    fn build_request(self, table: &mut Table) {
        table.insert("o".into(), "o".into());
        table.insert("s".into(), self.side.into());
        table.insert("v".into(), self.value.into());
        if let Some(addr) = self.addr {
            table.insert("p".into(), addr.into());
        }
        if let Some(bit) = self.bit {
            table.insert("b".into(), (1 << bit).into());
        }
    }

    fn parse_response(_: Value) -> Result<(), LocalStr> { Ok(()) }
}

pub struct TurtleCall {
    pub func: LocalStr,
    pub args: Vec<Value>,
}

impl Action for TurtleCall {
    type Output = Value;

    fn build_request(self, table: &mut Table) {
        table.insert("o".into(), "t".into());
        table.insert("f".into(), self.func.into());
        table.insert("v".into(), vec_to_table(self.args).into());
    }

    fn parse_response(response: Value) -> Result<Value, LocalStr> { Ok(response) }
}
