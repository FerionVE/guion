use std::ops::Deref;
use super::*;
use std::any::Any;
use std::rc::Rc;

pub mod link;
pub mod dyn_widget;
pub mod fns;
pub mod immediate;
//mod imp;

pub trait Widget<E>: WidgetAsAny<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    fn render(&self, l: Link<E>, r: (&mut ERenderer<E>,&Bounds));
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds));
    fn size(&self, l: Link<E>) -> ESize<E>;

    /// returns if the widget should be rendered
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);

    fn has_childs(&self) -> bool;

    fn childs<'a>(&'a self) -> Vec<Resolvable<'a,E>>;
    fn _childs<'a>(&'a mut self) -> Vec<&'a dyn ResolveRaw<E>>;
    fn _childs_mut<'a>(&'a mut self) -> Vec<&'a mut dyn ResolveRaw<E>>;

    fn child_paths(&self, own_path: WPSlice<E>) -> Vec<E::WidgetPath>;/* {
        self.childs().iter()
            .map(|p| p.path(own_path) )
            .collect()
    }*/

    fn erase(&self) -> &E::DynWidget {
        WidgetAsAny::_erase(self)
    }
    fn erase_mut(&mut self) -> &mut E::DynWidget {
        WidgetAsAny::_erase_mut(self)
    }

    #[inline]
    fn resolve_mut<'a>(&'a mut self, i: EWPSlice<E>) -> Result<WidgetRefMut<'a,E>,()> {
        if i.is_empty() {
            return Ok(Box::new(self.erase_mut()))
        }
        for c in self._childs_mut() {
            if c.is_subpath(&i[0]) {
                return c.resolve_mut(&i[1..]);
            }
        }
        Err(())
    }
    #[inline]
    fn resolve<'a>(&'a self, i: EWPSlice<E>) -> Result<Resolvable<'a,E>,()> {
        if i.is_empty() {
            return Ok(Resolvable::Widget(Rc::new(self.erase())))
        }
        for c in self.childs() {
            if c.is_subpath(&i[0]) {
                return c.resolve(&i[1..]);
            }
        }
        Err(())
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