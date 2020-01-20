use super::*;
use std::any::Any;

pub mod link;
pub mod dyn_widget;
pub mod fns;
//pub mod imp;

pub trait Widget<E>: WidgetAsAny<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    fn render(&self, l: Link<E>, r: (&mut ERenderer<E>,&Bounds));
    fn event(&self, l: Link<E>, r: (EEvent<E>,&Bounds));
    fn size(&self, l: Link<E>) -> Size;

    /// returns if the widget should be rendered
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);

    fn has_childs(&self) -> bool;

    fn for_childs(&self, f: &mut dyn FnOnce(&E::DynWidget,usize));
    fn for_childs_mut(&mut self, f: &mut dyn FnOnce(&mut E::DynWidget,usize)->E::ValidState) -> E::ValidState;

    fn child_paths(&self, own_path: WPSlice<E>) -> Vec<E::WidgetPath>;/* {
        self.childs().iter()
            .map(|p| p.path(own_path) )
            .collect()
    }*/
    #[inline]
    fn resolve_mut(&mut self, i: &EWPSub<E>, f: &mut dyn FnOnce(&mut E::DynWidget)->E::ValidState ) -> E::ValidState {
        /*for c in self.childs_mut() {
            if let Some(w) = c.widget_if_id_eq_mut(i) {
                return Some(w);
            }
        }
        None*/
        unimplemented!()
    }
    #[inline]
    fn resolve(&self, i: &EWPSub<E>, f: &mut dyn FnOnce(&E::DynWidget) ) {
        /*for c in self.childs() {
            if let Some(w) = c.widget_if_id_eq(i) {
                return Some(w);
            }
        }
        None*/
        unimplemented!();
    }
    #[inline]
    fn self_in_parent(&self, parent: WPSlice<E>) -> E::WidgetPath {
        parent.unslice().attached(SubPath::from_id(self.id()))
    }
    #[inline]
    fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        p.eq_id(self.id())
    }

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