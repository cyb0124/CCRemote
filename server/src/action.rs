use super::lua_value::{vec_to_table, Table, Value};
use std::{
    cell::RefCell,
    convert::TryInto,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

pub trait Action: 'static {
    type Output;
    fn build_request(self) -> Value;
    fn parse_response(response: Value) -> Result<Self::Output, String>;
}

struct ActionState<T: Action> {
    result: Option<Result<T::Output, String>>,
    waker: Option<Waker>,
    action: Option<T>,
}

pub trait ActionRequest {
    fn build_request(&mut self) -> Value;
    fn on_fail(&mut self, reason: String);
    fn on_response(&mut self, result: Value) -> Result<(), String>;
}

impl<T: Action> ActionRequest for ActionState<T> {
    fn build_request(&mut self) -> Value { self.action.take().unwrap().build_request() }

    fn on_fail(&mut self, reason: String) {
        self.result = Some(Err(reason));
        if let Some(waker) = self.waker.take() {
            waker.wake()
        }
    }

    fn on_response(&mut self, result: Value) -> Result<(), String> {
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
    type Output = Result<T::Output, String>;
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
    pub text: String,
    pub color: u8,
}

impl Action for Log {
    type Output = ();

    fn build_request(self) -> Value {
        let mut result = Table::new();
        result.insert("op".into(), "log".into());
        result.insert("color".into(), self.color.into());
        result.insert("text".into(), self.text.into());
        result.into()
    }

    fn parse_response(_response: Value) -> Result<(), String> { Ok(()) }
}

pub struct Call {
    pub addr: &'static str,
    pub args: Vec<Value>,
}

impl Action for Call {
    type Output = Value;

    fn build_request(self) -> Value {
        let mut result = Table::new();
        result.insert("op".into(), "call".into());
        result.insert("p".into(), self.addr.into());
        result.insert("v".into(), vec_to_table(self.args).into());
        result.into()
    }

    fn parse_response(response: Value) -> Result<Value, String> { Ok(response) }
}

pub struct RedstoneInput {
    pub side: &'static str,
}

impl Action for RedstoneInput {
    type Output = u8;

    fn build_request(self) -> Value {
        let mut result = Table::new();
        result.insert("op".into(), "ri".into());
        result.insert("s".into(), self.side.into());
        result.into()
    }

    fn parse_response(response: Value) -> Result<u8, String> { response.try_into() }
}

pub struct RedstoneOutput {
    pub side: &'static str,
    pub value: u8,
}

impl Action for RedstoneOutput {
    type Output = ();

    fn build_request(self) -> Value {
        let mut result = Table::new();
        result.insert("op".into(), "ro".into());
        result.insert("s".into(), self.side.into());
        result.insert("v".into(), self.value.into());
        result.into()
    }

    fn parse_response(_response: Value) -> Result<(), String> { Ok(()) }
}
