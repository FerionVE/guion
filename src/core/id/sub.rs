use super::*;

pub trait SubWidgetID {
    fn is<T: Any>(&self) -> bool;
    fn downcast_ref<T: Any>(&self) -> Option<&T>;
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;
    fn downcast_into<T: Any>(self) -> Result<T,Self> where Self: Sized;
}

impl SubWidgetID for Box<dyn Any> {
    #[inline]
    fn is<T: Any>(&self) -> bool {
       (**self).is::<T>() 
    }
    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (**self).downcast_ref::<T>()
    }
    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (**self).downcast_mut::<T>()
    }
    #[inline]
    fn downcast_into<T: Any>(self) -> Result<T,Self> {
        self.downcast().map(|v| *v )
    }
}