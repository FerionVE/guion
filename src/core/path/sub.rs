use super::*;

pub trait SubPath<E>: Clone + PartialEq<Self> + Sized where E: Env {
    fn from_id(id: E::WidgetID) -> Self;
    fn eq_id(&self, id: E::WidgetID) -> bool;
    fn into_id(self) -> E::WidgetID;

    fn is<T: Any>(&self) -> bool;
    fn downcast_ref<T: Any>(&self) -> Option<&T>;
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;
    fn downcast_into<T: Any>(self) -> Result<T,Self> where Self: Sized + 'static;
    #[inline]
    fn eq<I: SubPath<F> + 'static, F: Env>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, |r| self == r )
    }
}

/*impl SubPath for Box<dyn Any> {
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
}*/