use std::any::Any;
use super::*;

impl<T,E,C> ITemplate<E> for AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T> + 'static, T: ITemplate<E>, E: Env + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        <T as ITemplate<E>>::id(self)
    }
    
    #[inline]
    fn invalid(&self) -> Option<u32> {
        <T as ITemplate<E>>::invalid(self)
    }
    #[inline]
    fn set_invalid(&mut self, v: Option<u32>) {
        <T as ITemplate<E>>::set_invalid(self,v)
    }
    
    #[inline]
    fn style(&self, s: &mut ESVariant<E>) {
        <T as ITemplate<E>>::style(self,s)
    }
}
#[doc(hidden)]
impl<T,E,C> Widget<E> for AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T> + 'static, T: ITemplate<E>, E: Env + 'static {
    crate::impl_template_inner!(AsTemplate<T,E,C>,E);
    #[inline]
    fn as_any_inner(&self) -> &dyn Any {
        &self.inner
    }
    #[inline]
    fn as_any_inner_mut(&mut self) -> &mut dyn Any {
        &mut self.inner
    }
}