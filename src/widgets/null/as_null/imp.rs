use std::any::Any;
use super::*;

impl<T,E,C> INull<E> for AsNull<T,E,C> where C: Borrow<T> + BorrowMut<T> + 'static, T: INull<E>, E: Env + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        <T as INull<E>>::id(self)
    }
    
    #[inline]
    fn invalid(&self) -> bool {
        <T as INull<E>>::invalid(self)
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        <T as INull<E>>::set_invalid(self,v)
    }
    
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        <T as INull<E>>::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        <T as INull<E>>::set_parent(self,v)
    }
    #[inline]
    fn style(&self, s: &mut EStyle<E>) {
        <T as INull<E>>::style(self,s)
    }
}
#[doc(hidden)]
impl<T,E,C> Widget<E> for AsNull<T,E,C> where C: Borrow<T> + BorrowMut<T> + 'static, T: INull<E>, E: Env + 'static {
    crate::impl_null_inner!(AsNull<T,E,C>,E);
    #[inline]
    fn as_any_inner(&self) -> &dyn Any {
        &self.inner
    }
    #[inline]
    fn as_any_inner_mut(&mut self) -> &mut dyn Any {
        &mut self.inner
    }
}