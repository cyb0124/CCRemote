use super::super::factory::Factory;
use super::super::util::{spawn, AbortOnDrop};
use super::{IntoProcess, Process};
use std::{cell::RefCell, rc::Rc};

pub struct ConditionalConfig<T: IntoProcess> {
    pub condition: Box<dyn Fn(&Factory) -> bool>,
    pub child: T,
}

pub struct ConditionalProcess<T: Process> {
    condition: Box<dyn Fn(&Factory) -> bool>,
    child: Rc<RefCell<T>>,
}

impl<T: IntoProcess> IntoProcess for ConditionalConfig<T> {
    type Output = ConditionalProcess<T::Output>;
    fn into_process(self, factory: &Factory) -> Rc<RefCell<Self::Output>> {
        Rc::new(RefCell::new(Self::Output { condition: self.condition, child: self.child.into_process(factory) }))
    }
}

impl<T: Process> Process for ConditionalProcess<T> {
    fn run(&self, factory: &Factory) -> AbortOnDrop<Result<(), String>> {
        if (self.condition)(factory) {
            self.child.borrow().run(factory)
        } else {
            spawn(async { Ok(()) })
        }
    }
}
