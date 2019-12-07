use crate::core::util::ScopedMut;
use super::*;


impl<T,U,E> ITemplate<E> for AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        self.inner.access(#[inline] |s| Widget::id(s).clone() )
    }

    #[inline]
    fn invalid(&self) -> bool {
        self.inner.access(#[inline] |s| Widget::invalid(s) )
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        self.inner.access_mut(#[inline] |s| Widget::set_invalid(s,v) )
    }

    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        self.inner.access(#[inline] |s| Widget::parent(s) )
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.inner.access_mut(#[inline] |s| Widget::set_parent(s,v) )
    }
}

impl<T,U,E> Widget<E> for AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    crate::impl_template_inner!(AsTemplate<T,U,E>,E);
}