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