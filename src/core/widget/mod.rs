//! Widgets are interfaced in two Traits for immutable and mutable operations
//! The Traits features interface for queuering e.g. id or style, and also accessing or resolving child widgets
//! Note that some functions in the traits are not meant to be called from externel, but over `Link`'s methods
use super::*;
use std::any::{TypeId, type_name};
use cast::Statize;

pub mod link;
pub mod as_widget;
pub mod cast;
pub mod ext;
#[doc(hidden)]
pub mod imp;

/// Core Trait of guion ™️
pub trait Widget<'w,E>: WBase<'w,E> + 'w where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    /// this method should not be called from external, rather `Link::render`
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool;
    /// this method should not be called from external, rather `Link::event`
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64));
    /// this method should not be called from external, rather `Link::size`
    fn size(&self, l: Link<E>) -> ESize<E>;

    /// returns if the widget should be rendered
    fn invalid(&self) -> bool {
        true
    }
    

    fn childs(&self) -> usize;
    //fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()>;
    //fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()>;

    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's;
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'w,E>>;
    
    #[deprecated]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        self.childs_ref().into_iter() //TODO optimize, use direct accessors
            .map(|c| c.self_in_parent(own_path.refc()) )
            .collect::<Vec<_>>()
    }
    
    /// resolve a deep child item by the given relative path
    /// an empty path will resolve to this widget
    #[inline]
    fn resolve<'s>(&'s self, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> where 'w: 's {
        if i.is_empty() {
            return Ok(Resolvable::Widget(self.box_ref()))
        }
        for c in self.childs_ref() {
            if c.is_subpath(i.index(0)) {
                return c.resolve(i.slice(1..));
            }
        }
        Err(())
    }
    /// resolve a deep child item by the given relative path
    /// an empty path will resolve to this widget
    #[inline]
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        if i.is_empty() {
            return Ok(Resolvable::Widget(self.box_box()))
        }
        for c in self.childs_box() {
            if c.is_subpath(i.index(0)) {
                return c.resolve(i.slice(1..));
            }
        }
        Err(())
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        for (i,c) in self.childs_ref().iter().enumerate() {
            if c.is_subpath(p) {
                return Ok(i);
            }
        }
        Err(())
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        if i.is_empty() {
            return Ok(*b)
        }
        self.resolve_child(i.index(0))
            .and_then(|i| self._trace_bounds(l,i,b,force) )
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()>;
    #[inline]
    fn self_in_parent(&self, parent: E::WidgetPath) -> E::WidgetPath {
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
    #[allow(unused)]
    #[inline]
    fn style(&self, s: &mut ESVariant<E>) {
        
    }
    #[allow(unused)]
    #[inline]
    fn border(&self, b: &mut Border) {
        
    }
    
    fn inner<'s>(&'s self) -> Option<&'s dyn Widget<'w,E>> {
        None
    }

    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
    }
}

pub trait WidgetMut<'w,E>: Widget<'w,E> + WBaseMut<'w,E> where E: Env + 'static {
    #[allow(unused)]
    fn set_invalid(&mut self, v: bool) {
        
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's;
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>>;

    /// resolve a deep child item by the given relative path
    /// an empty path will resolve to this widget
    #[inline]
    fn resolve_mut<'s>(&'s mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'s,E>,()> where 'w: 's { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        if invalidate {self.set_invalid(true);}
        if i.is_empty() {
            return Ok(ResolvableMut::Widget(self.box_mut()))
        }
        for c in self.childs_mut() {
            if c.is_subpath(i.index(0)) {
                return c.resolve_mut(i.slice(1..),invalidate);
            }
        }
        Err(())
    }

    /// resolve a deep child item by the given relative path
    /// an empty path will resolve to this widget
    fn resolve_box_mut(mut self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'w,E>,()> {
        if invalidate {self.set_invalid(true);}
        if i.is_empty() {
            return Ok(ResolvableMut::Widget(self.box_box_mut()))
        }
        for c in self.childs_box_mut() {
            if c.is_subpath(i.index(0)) {
                return c.resolve_mut(i.slice(1..),invalidate);
            }
        }
        Err(())
    }

    fn inner_mut<'s>(&'s mut self) -> Option<&'s mut dyn WidgetMut<'w,E>> {
        None
    }
}

/// this trait is blanket implemented for all widget and provides functions which require compile-time knowledge of types
#[doc(hidden)]
pub trait WBase<'w,E> where E: Env {
    fn typeid(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
    fn erase<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's;
    fn box_ref<'s>(&'s self) -> WidgetRef<'s,E> where 'w: 's;
    fn box_box(self: Box<Self>) -> WidgetRef<'w,E>;
    fn boxed_ref(self) -> WidgetRef<'w,E> where Self: Sized;
}
impl<'w,T,E> WBase<'w,E> for T where T: Widget<'w,E>+Statize<E>, E: Env {
    fn typeid(&self) -> TypeId {
        <Self as Statize<E>>::_typeid()
    }
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    fn erase<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's {
        self
    }
    fn box_ref<'s>(&'s self) -> WidgetRef<'s,E> where 'w: 's {
        Box::new(self.erase())
    }
    fn box_box(self: Box<Self>) -> WidgetRef<'w,E> {
        self
    }
    fn boxed_ref(self) -> WidgetRef<'w,E> where Self: Sized {
        Box::new(self)
    }
}

/// this trait is blanket implemented for all widget and provides functions which require compile-time knowledge of types
#[doc(hidden)]
pub trait WBaseMut<'w,E> where E: Env {
    fn base<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's;
    fn erase_mut<'s>(&'s mut self) -> &'s mut dyn WidgetMut<'w,E> where 'w: 's;
    fn box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> where 'w: 's;
    fn box_box_mut(self: Box<Self>) -> WidgetRefMut<'w,E>;
    fn boxed(self) -> WidgetRefMut<'w,E> where Self: Sized;
}
impl<'w,T,E> WBaseMut<'w,E> for T where T: WidgetMut<'w,E>+Statize<E>, E: Env {
    fn base<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's {
        self
    }
    fn erase_mut<'s>(&'s mut self) -> &'s mut dyn WidgetMut<'w,E> where 'w: 's {
        self
    }
    fn box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> where 'w: 's {
        Box::new(self.erase_mut())
    }
    fn box_box_mut(self: Box<Self>) -> WidgetRefMut<'w,E> {
        self
    }
    fn boxed(self) -> WidgetRefMut<'w,E> where Self: Sized {
        Box::new(self)
    }
}