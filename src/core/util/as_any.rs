use crate::core::env::Env;
use crate::core::widget::Widget;
use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl AsAny for dyn Any {
    fn as_any(&self) -> &dyn Any {
        &(*self)
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut (*self)
    }
}

impl<E> AsAny for dyn Widget<E> where E: Env {
    fn as_any(&self) -> &dyn Any {
        self.as_any()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_any_mut()
    }
}