use super::*;

pub trait SubPath<E>:
    Clone +
    PartialEq<Self> +
    Sized +
    From<EWPSub<E>> +
    Into<EWPSub<E>> +
where E: Env {
    fn from_id(id: E::WidgetID) -> Self;
    fn _eq_id(&self, id: E::WidgetID) -> bool;
    fn into_id(self) -> E::WidgetID;

    fn resolves_to_id(&self, id: E::WidgetID) -> bool;
    fn resolves_to_path(&self, p: E::WidgetPath) -> bool;

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