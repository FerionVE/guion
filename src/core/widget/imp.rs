use super::*;

impl<'a,E,T> Widget<'a,E> for &'a mut T where T: Widget<'a,E>, E: Context {
    #[inline]
    fn id(&'a self) -> E::WidgetID {
        Widget::id(self)
    }
    #[inline]
    fn _handler(&'a self) -> HandlerFns<E> {
        Widget::_handler(self)
    }
    #[inline]
    fn invalid(&'a self) -> bool {
        Widget::invalid(self)
    }
    #[inline]
    fn set_invalid(&'a mut self, v: bool) {
        Widget::set_invalid(self,v)
    }
    #[inline]
    fn parent(&'a self) -> Option<E::WidgetID> {
        Widget::parent(self)
    }
    #[inline]
    fn set_parent(&'a mut self, v: Option<E::WidgetID>) {
        Widget::set_parent(self,v)
    }
    #[inline]
    fn childs(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a> {
        Widget::childs(self)
    }
    #[inline]
    fn childs_vec(&'a self) -> Vec<E::WidgetID> {
        Widget::childs_vec(self)
    }
    #[inline]
    fn as_any(&'a self) -> &dyn Any {
        Widget::as_any(self)
    }
    #[inline]
    fn as_any_mut(&'a mut self) -> &mut dyn Any {
        Widget::as_any_mut(self)
    }
    #[inline]
    fn selectable(&'a self) -> bool {
        Widget::selectable(self)
    }
    #[inline]
    fn has_childs(&'a self) -> bool {
        Widget::has_childs(self)
    }
    #[inline]
    fn style(&'a self) -> E::Style {
        Widget::style(self)
    }
}

/*use crate::core::util::ScopedMut;
use super::*;

impl<E,S,T> Widget<E> for S where S: ScopedMut<T=T> + 'static, T: Widget<E>, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        Widget::access(self, #[inline] |w| w.id() )
    }
    #[inline]
    fn _handler(&self) -> HandlerFns<E> {
        Widget::access(self, #[inline] |w| w._handler() )
    }
    #[inline]
    fn invalid(&self) -> bool {
        Widget::access(self, #[inline] |w| w.invalid() )
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        Widget::access_mut(self, #[inline] |w| w.set_invalid(v) )
    }
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        Widget::access(self, #[inline] |w| w.parent() )
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        Widget::access_mut(self, #[inline] |w| w.set_parent(v) )
    }
    #[inline]
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a> {
        Widget::access(self, #[inline] |w| Box::new( w.childs_vec().into_iter() ) )
    }
    #[inline]
    fn childs_vec<'a>(&'a self) -> Vec<E::WidgetID> {
        Widget::access(self, #[inline] |w| w.childs_vec() )
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
        Widget::access(self, #[inline] |w| w.selectable() )
    }
    #[inline]
    fn has_childs(&self) -> bool {
        Widget::access(self, #[inline] |w| w.has_childs() )
    }
    #[inline]
    fn style(&self) -> E::Style {
        Widget::access(self, #[inline] |w| w.style() )
    }
}*/