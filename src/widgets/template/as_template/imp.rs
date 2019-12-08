use super::*;


impl<T,E,C> ITemplate<E> for AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T> + 'static, T: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        <T as ITemplate<E>>::id(self)
    }
    
    #[inline]
    fn invalid(&self) -> bool {
        <T as ITemplate<E>>::invalid(self)
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        <T as ITemplate<E>>::set_invalid(self,v)
    }
    
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        <T as ITemplate<E>>::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        <T as ITemplate<E>>::set_parent(self,v)
    }
    #[inline]
    fn style(&self) -> &E::Style {
        <T as ITemplate<E>>::style(self)
    }
}

impl<T,E,C> Widget<E> for AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T> + 'static, T: ITemplate<E>, E: Context + 'static {
    crate::impl_template_inner!(AsTemplate<T,E,C>,E);
}