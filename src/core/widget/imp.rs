use crate::core::util::ScopedMut;
use super::*;

impl<E,S,T> Widget<E> for S where S: ScopedMut<T=T> + 'static, T: Widget<E>, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        <S as ScopedMut>::access(self, #[inline] |w| w.id() )
    }
    #[inline]
    fn _handler(&self) -> HandlerFns<E> {
        <S as ScopedMut>::access(self, #[inline] |w| w._handler() )
    }
    #[inline]
    fn invalid(&self) -> bool {
        <S as ScopedMut>::access(self, #[inline] |w| w.invalid() )
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        <S as ScopedMut>::access_mut(self, #[inline] |w| w.set_invalid(v) )
    }
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        <S as ScopedMut>::access(self, #[inline] |w| w.parent() )
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        <S as ScopedMut>::access_mut(self, #[inline] |w| w.set_parent(v) )
    }
    #[inline]
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a> {
        <S as ScopedMut>::access(self, #[inline] |w| Box::new( w.childs_vec().into_iter() ) )
    }
    #[inline]
    fn childs_vec<'a>(&'a self) -> Vec<E::WidgetID> {
        <S as ScopedMut>::access(self, #[inline] |w| w.childs_vec() )
    }
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline]
    fn selectable(&self) -> bool {
        <S as ScopedMut>::access(self, #[inline] |w| w.selectable() )
    }
    #[inline]
    fn has_childs(&self) -> bool {
        <S as ScopedMut>::access(self, #[inline] |w| w.has_childs() )
    }
}