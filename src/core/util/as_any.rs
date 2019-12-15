use crate::core::ctx::*;
use crate::core::widget::Widget;
use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl AsAny for dyn Any {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        &(*self)
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut (*self)
    }
}

impl<E> AsAny for dyn Widget<E> where E: Env {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self.as_any()
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_any_mut()
    }
}