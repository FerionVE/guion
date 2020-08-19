//! Widgets are interfaced in two Traits for immutable and mutable operations  
//! The Traits features interface for queuering e.g. id or style, and also accessing or resolving child widgets  
//! Note that some functions in the traits are not meant to be called from externel, but over `Link`'s methods  
use super::*;
use std::any::{TypeId, type_name};
use cast::Statize;
use traitcast::TraitObject;

pub mod link;
pub mod as_widget;
#[doc(hidden)]
pub mod cast;
pub mod ext;
#[doc(hidden)]
pub mod imp;
pub mod resolved;
pub mod resolvable;
pub mod root;
pub mod array;
pub mod ident;

/// Core Trait of guion ™️
pub trait Widget<'w,E>: WBase<'w,E> + 'w where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) this method should not be called from external, rather [`Link::render`](link/struct.Link.html#method.render)
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>);
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) this method should not be called from external, rather [`Link::event`](link/struct.Link.html#method.event)
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp;
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) this method should not be called from external, rather [`Link::size`](link/struct.Link.html#method.size)
    fn _size(&self, l: Link<E>, e: &ESVariant<E>) -> ESize<E>;

    fn childs(&self) -> usize;
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> where 'w: 's;
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()>;

    #[deprecated]
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        (0..self.childs())
            .map(#[inline] |i| self.child(i).unwrap() )
            .collect::<Vec<_>>()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>>;
    
    #[deprecated]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (0..self.childs())
            .map(#[inline] |i| self.child(i).unwrap().in_parent_path(own_path.refc()) )
            .collect::<Vec<_>>()
    }

    /*#[deprecated="Merge the shit into normal evention"]
    fn _route_event(&self, mut l: Link<E>, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,()> {
        let cb = self.child_bounds(l.reference(),&e.1,false)?;
        {
            let c = self.resolve_child(child.index(0))?;
            let mut l = l.for_child(c)?;
            let b = cb[c];
            //TODO FIX corrent compound filter use
            if l._route_event(&EventCompound(e.0.clone(),b,e.2,e.3.clone())/*TODO compounds.with_bounds(b)*/,child.slice(1..))? {
                return Ok(true);
            }
        }
        Ok(if self._accept_child_events() {
            self._event_direct(l,e)
        }else{
            false
        })
    }*/
    
    //fn _accept_child_events(&self) -> bool;

    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget
    #[inline]
    fn resolve<'s>(&'s self, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> where 'w: 's {
        if i.is_empty() {
            return Ok(Resolvable::Widget(self.box_ref()))
        }
        let c = self.resolve_child(i.index(0))?;
        self.child(c).unwrap().resolve_child(i.slice(1..))
    }
    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget
    #[inline]
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        if i.is_empty() {
            return Ok(Resolvable::Widget(self.box_box()))
        }
        let c = self.resolve_child(i.index(0))?;
        self.into_child(c).unwrap_nodebug().resolve_child(i.slice(1..))
    }
    /// child widget by path segment
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        for c in 0..self.childs() {
            if self.child(c).unwrap().resolves_by(p) {
                return Ok(c);
            }
        }
        Err(())
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Bounds,()> {
        if i.is_empty() {
            return Ok(*b)
        }
        let child = self.resolve_child(i.index(0))?;
        let bounds = self.child_bounds(l,b,e,force)?;
        
        Ok(bounds[child])
    }
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()>;
    
    /// attach widget's id to the given parent path
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        parent.attached(SubPath::from_id(self.id()))
    }
    /// if the path segment would resolve to this widget
    #[inline]
    fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        p.resolves_to_id(self.id())
    }

    /// if the widget should be focusable.  
    /// regularly true for interactive widgets, false for layouts.
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
    
    #[inline]
    fn inner<'s>(&'s self) -> Option<&'s dyn Widget<'w,E>> {
        None //TODO fix inner mechanism AsWidget
    }

    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
    }

    /// The impl_traitcast! macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        None
    }
}

pub trait WidgetMut<'w,E>: Widget<'w,E> + WBaseMut<'w,E> where E: Env + 'static {
    /// an alternative way to pass mutations. See [Link::Message]
    #[allow(unused)]
    #[inline]
    fn message(&mut self, m: E::Message) {

    }

    #[allow(unused)]
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        
    }

    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> where 'w: 's;
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()>;

    #[deprecated]
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's;
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>>;

    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget
    #[inline]
    fn resolve_mut<'s>(&'s mut self, i: E::WidgetPath) -> Result<ResolvableMut<'s,E>,()> where 'w: 's { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        if i.is_empty() {
            return Ok(ResolvableMut::Widget(self.box_mut()))
        }
        let c = self.resolve_child(i.index(0))?;
        self.child_mut(c).unwrap().resolve_child_mut(i.slice(1..))
    }

    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget
    #[inline]
    fn into_resolve_mut(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,()> {
        if i.is_empty() {
            return Ok(ResolvableMut::Widget(self.box_box_mut()))
        }
        let c = self.resolve_child(i.index(0))?;
        self.into_child_mut(c).unwrap_nodebug().resolve_child_mut(i.slice(1..))
    }

    #[inline]
    fn inner_mut<'s>(&'s mut self) -> Option<&'s mut dyn WidgetMut<'w,E>> {
        None
    }

    #[inline]
    fn pass(self) -> Self where Self: Sized {
        self
    }

    /// The impl_traitcast! macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        Widget::_as_trait_ref(self,t)
    }
    /// The impl_traitcast_mut! macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_mut(&mut self, t: TypeId) -> Option<TraitObject> {
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
    #[inline]
    fn typeid(&self) -> TypeId {
        <Self as Statize<E>>::_typeid()
    }
    #[inline]
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    #[inline]
    fn erase<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's {
        self
    }
    #[inline]
    fn box_ref<'s>(&'s self) -> WidgetRef<'s,E> where 'w: 's {
        Box::new(self.erase())
    }
    #[inline]
    fn box_box(self: Box<Self>) -> WidgetRef<'w,E> {
        self
    }
    #[inline]
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
    #[inline]
    fn base<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's {
        self
    }
    #[inline]
    fn erase_mut<'s>(&'s mut self) -> &'s mut dyn WidgetMut<'w,E> where 'w: 's {
        self
    }
    #[inline]
    fn box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> where 'w: 's {
        Box::new(self.erase_mut())
    }
    #[inline]
    fn box_box_mut(self: Box<Self>) -> WidgetRefMut<'w,E> {
        self
    }
    #[inline]
    fn boxed(self) -> WidgetRefMut<'w,E> where Self: Sized {
        Box::new(self)
    }
}
