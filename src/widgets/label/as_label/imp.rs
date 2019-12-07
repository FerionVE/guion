use super::*;


impl<T,E> ILabel<E> for AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        ILabel::id(self)
    }
    
    #[inline]
    fn invalid(&self) -> bool {
        ILabel::invalid(self)
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        ILabel::set_invalid(self,v)
    }
    
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        ILabel::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        ILabel::set_parent(self,v)
    }
    #[inline]
    fn style(&self) -> E::Style {
        ILabel::style(self)
    }
}

impl<T,E> Widget<E> for AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    crate::impl_label_inner!(AsLabel<T,E>,E);
}