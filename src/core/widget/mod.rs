use super::*;
use std::any::Any;

pub mod link;
pub mod dyn_widget;
pub mod fns;
//pub mod imp;

pub trait Widget<E>: WidgetAsAny<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID;
    #[doc(hidden)]
    fn _fns(&self) -> WidgetFns<E>;

    /// returns if the widget should be rendered
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);

    fn has_childs(&self) -> bool;
    /// iterator over widget's child widgets
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a>;
    /// id of child widgets as vec
    fn childs_vec<'a>(&'a self) -> Vec<E::WidgetID>;
    /// should the widget be focusable, regularly true for interactive widgets, false for layouts
    fn selectable(&self) -> bool;

    #[inline]
    fn style(&self) -> &EStyle<E> {
        e_default_style::<E>()
    }
    #[inline]
    fn border(&self) -> &Border {
        e_default_border::<E>()
    }
    /// returns this widget as Any
    #[inline]
    fn as_any(&self) -> &dyn Any {
        WidgetAsAny::_as_any(self)
    }
    /// returns this widget as Any
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        WidgetAsAny::_as_any_mut(self)
    }
    /// returns a erased reference to a underlying struct for a wrapper, else to this widget
    #[inline]
    fn as_any_inner(&self) -> &dyn Any {
        WidgetAsAny::_as_any(self)
    }
    /// returns a erased reference to a underlying struct for a wrapper, else to this widget
    #[inline]
    fn as_any_inner_mut(&mut self) -> &mut dyn Any {
        WidgetAsAny::_as_any_mut(self)
    }
}