//! Widgets are interfaced in two Traits for immutable and mutable operations
//! 
//! The Traits features interface for querying e.g. id or style, and also accessing or resolving child widgets
//! 
//! Note that some functions in the traits are not meant to be called from external, but over [`Link`]'s methods  

use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;

use self::dyn_tunnel::WidgetDyn;
use self::stack::QueryCurrentWidget;

use super::*;
use std::any::{TypeId, type_name};
use traitcast::TraitObject;

pub mod dyn_tunnel;

//pub mod link;
pub mod as_widget;
pub mod ext;
#[doc(hidden)]
pub mod imp;
// pub mod resolved;
//pub mod root;
pub mod as_widgets;
// #[deprecated="Replaced by AsWidgets"]
// pub mod array;
pub mod ident;

pub mod stack;

/// Core Trait of guion ™️
pub trait Widget<E>: WBase<E> + /*TODO bring back AsWidgetImplemented*/ where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    /// ![RENDER](https://img.shields.io/badge/-render-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![RENDER](https://img.shields.io/badge/-render-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::render`]
    fn _render<P>(&self, stack: &P, r: &mut ERenderer<'_,E>) where P: Queron<E> + ?Sized;
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::event`](Link::send_event)
    fn _event_direct<P>(&self, stack: &P, e: &EventCompound<E>) -> EventResp where P: Queron<E> + ?Sized;
    /// ![LAYOUT](https://img.shields.io/badge/-layout-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![LAYOUT](https://img.shields.io/badge/-layout-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::size`]
    fn _size<P>(&self, stack: &P, e: &EStyle<E>) -> ESize<E> where P: Queron<E> + ?Sized;

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn childs(&self) -> usize;
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    #[deprecated]
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R;

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    #[deprecated]
    fn childs_ref<'s,F>(
        &'s self,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    )
    where
        F: for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut E::Context<'cc>)
    {
        for i in 0..self.childs() {
            self.with_child(
                i,
                #[inline] |wg,ctx| (callback)(i,wg.unwrap(),ctx), 
                root, ctx,
            );
        }
    }
    
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    #[deprecated]
    fn child_paths(&self, own_path: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
        let mut dest = Vec::with_capacity(self.childs());

        for i in 0..self.childs() {
            self.with_child(
                i,
                #[inline] |wg,ctx| {
                    let w = wg.unwrap();
                    dest.push(w.in_parent_path(own_path.refc()));
                }, 
                root, ctx,
            );
        }

        dest
    }

    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// Resolve a deep child item by the given relative path
    /// 
    /// An empty path will resolve to this widget
    /// 
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but through [`Widgets::widget`]
    #[inline]
    #[deprecated]
    fn with_resolve<'s,F,R>(
        &'s self,
        i: E::WidgetPath,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> R
    {
        if i.is_empty() {
            return (callback)(Ok(self.erase()),ctx);
        }
        //TODO resolve_child could also return it's ref resolve
        match self.resolve_child(&i,root.fork(),ctx) {
            Ok((c,sub)) => {
                //TODO assert that with_child always calls the callback
                self.with_child(
                    c,
                    #[inline] |w,cb| (callback)(Ok(w.unwrap()),ctx),
                    root, ctx,
                )
            },
            Err(e) => {
                (callback)(Err(e),ctx)
            },
        }
    }
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// To (or through) which child path would the given sub_path resolve?
    /// 
    /// Returns the child index and the subpath inside the child widget to resolve further
    /// 
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but through [`Widgets::widget`]
    #[inline]
    fn resolve_child(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(usize,E::WidgetPath),E::Error> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        for c in 0..self.childs() {
            if let Some(r) = self.with_child(
                c, 
                #[inline] |w,_| w.unwrap().resolved_by_path(sub_path),
                root.fork(), ctx,
            ) {
                return Ok((c,r.sub_path));
            }
        }
        Err(self.gen_diag_error_resolve_fail(sub_path, "resolve",root,ctx))
    }
    /// ![LAYOUT](https://img.shields.io/badge/-resolving-000?style=flat-square)
    #[inline]
    fn trace_bounds<P>(&self, stack: &P, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error> where P: Queron<E> + ?Sized {
        if i.is_empty() {
            return Ok(*b)
        }
        let (child,_) = self.resolve_child(&i,root.fork(),ctx)?;
        let bounds = self.child_bounds(stack,b,e,force)?;
        
        Ok(bounds[child])
    }
    /// ![LAYOUT](https://img.shields.io/badge/-resolving-000?style=flat-square)
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized;
    
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// Attach widget's id to the given parent path
    #[inline]
    #[deprecated]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        parent.for_child_widget_id(self.id())
    }
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// Refer [`WidgetPath::resolves_thru`](WidgetPath::resolves_thru_child_id)
    /// 
    /// `sub_path`: subpath in parent widget (which contains this widget as child) which would probably resolve to/through this widget
    #[inline]
    #[deprecated]
    fn resolved_by_path(&self, sub_path: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        E::WidgetPath::resolves_thru_child_id(self.id(), sub_path)
    }

    /// If the widget should be focusable
    /// 
    /// Regularly true for interactive widgets, false for layouts.
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

    /// Determines the next child in this widget in the tabulation step
    #[inline]
    fn _tabulate_next_child<P>(&self, stack: &P, origin: TabulateNextChildOrigin, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> TabulateNextChildResponse where P: Queron<E> + ?Sized {
        match origin {
            // This widget is entered
            TabulateNextChildOrigin::Enter => match dir {
                // This widget is entered forwards, if focusable tabulate into this
                TabulateDirection::Forward if self.focusable() => TabulateNextChildResponse::This,
                // Entered forwards, not focusable but has childs, try tabulate into first child
                TabulateDirection::Forward if self.childs() != 0 => TabulateNextChildResponse::Child(0),
                // Entered backwards and has childs, try tabulate into last child
                TabulateDirection::Backward if self.childs() != 0 => TabulateNextChildResponse::Child(self.childs()-1),
                // Entered backwards, doesn't have childs but is focusable, try tabulate into this
                TabulateDirection::Backward if self.focusable() => TabulateNextChildResponse::This,
                // No childs and not focusable, leave this widget (resumes traverse in parent widget)
                _ => TabulateNextChildResponse::Leave,
            }
            // This widget was focused, tabulate away from this widget
            TabulateNextChildOrigin::This => match dir {
                // If forward and has childs, tabulate into first child
                TabulateDirection::Forward if self.childs() != 0 => TabulateNextChildResponse::Child(0),
                // Else, leave this widget (resumes traverse in parent widget)
                _ => TabulateNextChildResponse::Leave,
            }
            // Tabulate from previous child of this widget
            TabulateNextChildOrigin::Child(child_id) => match dir { //assert!(child_id < self.childs());
                // If forward and child after origin child, tabulate into next child
                TabulateDirection::Forward if child_id < self.childs().saturating_sub(1) => TabulateNextChildResponse::Child(child_id+1),
                // If backwards, and child before origin child, tabulate into previous child
                TabulateDirection::Backward if self.childs() != 0 && child_id != 0 => TabulateNextChildResponse::Child(child_id-1),
                // If backwards, and no childs before origin child, but this widget is focusable, tabulate into this widget
                TabulateDirection::Backward if self.focusable() => TabulateNextChildResponse::This,
                // Else, leave this widget (resumes traverse in parent widget)
                _ => TabulateNextChildResponse::Leave,
            }
        }
    }

    fn _tabulate<P>(&self, stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error> where P: Queron<E> + ?Sized {
        let current_path = QueryCurrentWidget.query_in(stack).unwrap().path;
        // fn to tabulate to the next child away from the previous child (child_id None = self)
        let enter_child_sub = |child_id: usize, to: TabulateOrigin<E>| -> Result<TabulateResponse<E>,E::Error> {
            self.with_child(
                child_id,
                #[inline] |child_widget, ctx| {
                    let child_widget = child_widget.unwrap();
                    let stack = stack::for_child_widget(stack, child_widget);
                    child_widget._tabulate(&stack, to, dir, root, ctx)
                }, 
                root, ctx
            )
        };
        let next_child = |mut child_id: Option<usize>| -> Result<TabulateResponse<E>,E::Error> {
            loop {
                // determine the targeted next child
                let targeted_child = self._tabulate_next_child(
                    stack,
                    TabulateNextChildOrigin::child_or_this(child_id),
                    dir,
                    root, ctx,
                );

                match targeted_child {
                    // enter child or repeat
                    TabulateNextChildResponse::Child(t) => match enter_child_sub(t,TabulateOrigin::Enter)? {
                        TabulateResponse::Done(v) => return Ok(TabulateResponse::Done(v)),
                        TabulateResponse::Leave => {
                            // couldn't enter next child, repeat
                            child_id = Some(t);
                            continue
                        },
                    }
                    TabulateNextChildResponse::This =>
                        if self.focusable() {
                            return Ok(TabulateResponse::Done(current_path.clone()))
                        }else{
                            // we aren't focusable, repeat
                            child_id = None;
                            continue
                        },
                    TabulateNextChildResponse::Leave => break,
                }
            }
            Ok(TabulateResponse::Leave)
        };
        // tabulate into specific child, either in resolve phase or enter
        let enter_child = |child_id: usize, to: TabulateOrigin<E>| -> Result<TabulateResponse<E>,E::Error> {
            match enter_child_sub(child_id,to)? {
                TabulateResponse::Done(v) => return Ok(TabulateResponse::Done(v)),
                TabulateResponse::Leave => return next_child(Some(child_id)),
            }
        };
        match op {
            TabulateOrigin::Resolve(p) => {
                if !p.is_empty() {
                    // pass 1: resolve to previous focused widget
                    let (child_id,sub_path) = self.resolve_child(&p,root.fork(),ctx)?;
                    return enter_child(child_id, TabulateOrigin::Resolve(sub_path));
                }else{
                    // pass 2: we are the previous focused widget and should tabulate away
                    return next_child( None);
                }
            },
            TabulateOrigin::Enter => {
                // we got entered from the parent widget

                let enter_dir = self._tabulate_next_child(
                    stack,
                    TabulateNextChildOrigin::Enter,
                    dir,
                    root, ctx,
                );

                match enter_dir {
                    // tabulate into enter the targeted child
                    TabulateNextChildResponse::Child(t) => return enter_child(t, TabulateOrigin::Enter),
                    // tabulate to self
                    TabulateNextChildResponse::This => return Ok(TabulateResponse::Done(current_path.clone())),
                    TabulateNextChildResponse::Leave => return Ok(TabulateResponse::Leave),
                }
            },
        }
    }
    
    #[inline]
    fn inner<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        None
    }
    #[inline]
    fn innest(&self) -> Option<&dyn WidgetDyn<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
        let mut i = self.erase();
        loop {
            let v = i.inner();
            if let Some(v) = v {
                i = v;
            }else{
                return Some(i);
            }
        }
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        WBase::as_any(self)
    }

    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        dest.push(self.type_name());
    }
    fn debugged_type_name(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        self.debug_type_name(&mut v);
        v.shrink_to_fit();
        v
    }

    /// ![TRAITCAST](https://img.shields.io/badge/-traitcast-000?style=flat-square)  
    /// The [`impl_traitcast`] macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        None
    }

    /// Use this to turn to dyn Widget
    #[inline]
    fn erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        WBase::_erase(self)
    }

    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// Box reference of this widget immutable. Use [`WidgetMut::box_mut`] to box into mutable [`WidgetRef`](WidgetRefMut).
    #[inline]
    fn box_ref<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's {
        WBase::_box_ref(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// Move widget into box immutable. Use [`WidgetMut::box_box_mut`] to box into mutable [`WidgetRef`](WidgetRefMut).
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        WBase::_box_box(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// Move widget into box immutable. Use [`WidgetMut::boxed_mut`] to box into mutable [`WidgetRef`](WidgetRefMut).
    #[inline]
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        WBase::_boxed(self)
    }

    #[inline(never)]
    fn gen_diag_error_resolve_fail(&self, sub_path: &E::WidgetPath, op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error {
        /*
        Failed to resolve 3/5/2 in Pane<Vec<WidgetRefMut<E>>>
            Child #0: 6 Button<E,Label<&str>>
            Child #1: 8 CheckBox<E,Label<&str>>
            Child #2: 4 ProgressBar<E>
        */
        /*
        Traitcast(_mut) from Label<E,&str> to ICheckBox<E> not implemented (strip "dyn " prefix)
        */
        let widget_type = self.debugged_type_name();
        let child_info = (0..self.childs())
            .map(#[inline] |i| self.with_child(i,|w,_| w.unwrap().guion_resolve_error_child_info(i) ,root,ctx)  )
            .collect::<Vec<_>>();
        GuionError::ResolveError(Box::new(ResolveError{
            op,
            sub_path: sub_path.clone(),
            widget_type,
            child_info,
        })).into()
    }

    #[inline(never)]
    fn guion_resolve_error_child_info(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        GuionResolveErrorChildInfo {
            child_idx,
            widget_type: self.debugged_type_name(),
            widget_path_if_path: None,
            widget_id: Some(self.id()),
        }
    }
}

/// This trait is blanket implemented for all widget and provides functions which require compile-time knowledge of types
#[doc(hidden)]
pub trait WBase<E> where E: Env {
    fn type_name(&self) -> &'static str;
    fn _erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's;
    // fn _as_wcow<'s>(&'s self) -> WidgetRef<'s,E>;
    // fn _box_into_wcow<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w;
    // fn _into_wcow<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w;
    fn _box_ref<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's;
    fn _box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w;
    fn _boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w;
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static;
}
impl<T,E> WBase<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    #[inline]
    fn _erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self
    }
    // #[inline]
    // fn _as_wcow<'s>(&'s self) -> WidgetRef<'s,E> {
    //     WCow::Borrowed(self)
    // }
    // #[inline]
    // fn _box_into_wcow<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
    //     WCow::Owned(self)
    // }
    // #[inline]
    // fn _into_wcow<'w>(self) -> WidgetRef<'w,E> where Self: Sized + 'w {
    //     let b = Box::new(self);
    //     WCow::Owned(b)
    // }
    #[inline]
    fn _box_ref<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's {
        //Box::new(self)
        todo!()
    }
    #[inline]
    fn _box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self
    }
    #[inline]
    fn _boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        Box::new(self)
    }
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}
