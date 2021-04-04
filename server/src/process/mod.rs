use super::factory::Factory;
use super::util::AbortOnDrop;
use std::{cell::RefCell, rc::Rc};

pub trait Process: 'static {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>>;
}

pub trait IntoProcess {
    type Output: Process;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>>;
}
