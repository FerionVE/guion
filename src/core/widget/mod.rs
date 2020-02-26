use super::*;
use std::any::Any;
use std::rc::Rc;

pub mod link;
pub mod dyn_widget;
pub mod fns;
pub mod immediate;
pub mod as_widget;
//mod imp;

pub trait Widget<E>: WidgetAsAny<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool;
    fn event(&self, l: Link<E>, e: EEvent<E>);
    fn size(&self, l: Link<E>) -> ESize<E>;

    /// returns if the widget should be rendered
    fn invalid(&self) -> bool {
        true
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
    }

    fn has_childs(&self) -> bool; //TODO eventually trash this

    fn childs<'a>(&'a self) -> Vec<Resolvable<'a,E>>;
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>>;

    fn child_paths(&self, own_path: WPSlice<E>) -> Vec<E::WidgetPath> {
        self.childs().into_iter()
            .map(|c| c.self_in_parent(own_path) )
            .collect::<Vec<_>>()
    }

    fn _child_bounds(&self, l: Link<E>, own: &Bounds, force: bool) -> Vec<Bounds>;

    fn erase(&self) -> &E::DynWidget {
        WidgetAsAny::_erase(self)
    }
    fn erase_mut(&mut self) -> &mut E::DynWidget {
        WidgetAsAny::_erase_mut(self)
    }

    fn as_immediate(&self) -> WidgetRef<E> {
        WidgetAsAny::_as_immediate(self)
    }
    fn as_immediate_mut(&mut self) -> WidgetRefMut<E> {
        WidgetAsAny::_as_immediate_mut(self)
    }

    #[inline]
    fn resolve_mut<'a>(&'a mut self, i: WPSlice<E>, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        if invalidate {self.set_invalid(true);}
        if i.slice.is_empty() {
            return Ok(ResolvableMut::Widget(self.as_immediate_mut()))
        }
        for c in self.childs_mut() {
            if c.is_subpath(i.index(0)) {
                return c.resolve_mut(i.slice(1..),invalidate);
            }
        }
        Err(())
    }
    #[inline]
    fn resolve<'a>(&'a self, i: WPSlice<E>) -> Result<Resolvable<'a,E>,()> {
        if i.slice.is_empty() {
            return Ok(Resolvable::Widget(Rc::new(self.as_immediate())))
        }
        for c in self.childs() {
            if c.is_subpath(i.index(0)) {
                return c.resolve(i.slice(1..));
            }
        }
        Err(())
    }
    fn trace_bounds(&self, l: Link<E>, i: WPSlice<E>, b: &Bounds, force: bool) -> Result<Bounds,()> {
        if i.slice.is_empty() {
            return Ok(*b)
        }
        let b = self._child_bounds(l,b,force);
        let mut i = 0;

        

        Err(())
    }
    #[inline]
    fn self_in_parent(&self, parent: WPSlice<E>) -> E::WidgetPath {
        parent.attached(SubPath::from_id(self.id()))
    }
    #[inline]
    fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        p.eq_id(self.id())
    }

    /// should the widget be focusable, regularly true for interactive widgets, false for layouts
    fn focusable(&self) -> bool;
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        self.focusable()
    }
    //if tab/shift-tab should tabulate away from this widget
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        true
    }

    /// attach widget's style
    #[inline]
    fn style(&self, s: &mut ESVariant<E>) {
        let _ = s;
    }
    #[inline]
    fn border(&self, b: &mut Border) {
        let _ = b;
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