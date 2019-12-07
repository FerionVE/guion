use super::*;


impl<T,E> ITemplate<E> for AsTemplate<T,E> where T: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        ITemplate::id(self)
    }
    
    #[inline]
    fn invalid(&self) -> bool {
        ITemplate::invalid(self)
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        ITemplate::set_invalid(self,v)
    }
    
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        ITemplate::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        ITemplate::set_parent(self,v)
    }
    #[inline]
    fn style(&self) -> E::Style {
        ITemplate::style(self)
    }
}

impl<T,E> Widget<E> for AsTemplate<T,E> where T: ITemplate<E>, E: Context + 'static {
    crate::impl_template_inner!(AsTemplate<T,E>,E);
}