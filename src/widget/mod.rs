//! Widgets are interfaced in two Traits for immutable and mutable operations
//! 
//! The Traits features interface for querying e.g. id or style, and also accessing or resolving child widgets
//! 
//! Note that some functions in the traits are not meant to be called from external, but over [`Link`]'s methods  

use std::any::{type_name, Any};
use std::ops::Range;

use crate::ctx::Context;
use crate::env::Env;
use crate::intercept::WidgetIntercept;
use crate::invalidation::Invalidation;
use crate::queron::Queron;
use crate::root::RootRef;
use crate::traitcast::{WQueryResponder, WQueryResponderGeneric, WQueryGeneric, WQuery, DowncastResponder, DowncastMutResponder};
use crate::util::error::{GuionError, ResolveError, GuionResolveErrorChildInfo};
use crate::util::tabulate::{TabulateNextChildOrigin, TabulateDirection, TabulateNextChildResponse, TabulateOrigin, TabulateResponse};
use crate::widget_decl::route::UpdateRoute;
use crate::{EventResp, event_new};
use crate::aliases::{ERenderer, ESize};
use crate::newpath::{PathResolvusDyn, PathStack, PathResolvus};

use self::cache::WidgetCache;
use self::dyn_tunnel::WidgetDyn;
use self::id::WidgetID;

pub mod dyn_tunnel;

pub mod as_widget;
pub mod ext;
#[doc(hidden)]
pub mod imp;
//pub mod root;
pub mod pane_childs;
// #[deprecated="Replaced by AsWidgets"]
// pub mod array;
pub mod ident;

pub mod stack;

pub mod cache;

pub mod id;

pub mod declared;

pub struct WidgetResolveDynResult<'a,E> {
    pub widget_id: WidgetID,
    pub widget: &'a (dyn WidgetDyn<E>+'a),
}
pub struct WidgetResolveDynResultMut<'a,E> {
    pub widget_id: WidgetID,
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
}

pub struct WidgetChildDynResult<'a,E> {
    pub idx: isize,
    pub widget_id: WidgetID,
    pub widget: &'a (dyn WidgetDyn<E>+'a),
}
pub struct WidgetChildDynResultMut<'a,E> {
    pub idx: isize,
    pub widget_id: WidgetID,
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
}

pub struct WidgetChildResolveDynResult<'a,'b,E> {
    pub idx: isize,
    pub sub_path: &'b (dyn PathResolvusDyn<E>+'b),
    pub widget_id: WidgetID,
    pub widget: &'a (dyn WidgetDyn<E>+'a),
}
pub struct WidgetChildResolveDynResultMut<'a,'b,E> {
    pub idx: isize,
    pub sub_path: &'b (dyn PathResolvusDyn<E>+'b),
    pub widget_id: WidgetID,
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
}

/// Core Trait of guion ™️
pub trait Widget<E>: WBase<E> + /*TODO bring back AsWidgetImplemented*/ where E: Env + 'static {
    type Cache: WidgetCache<E> + 'static;

    fn id(&self) -> WidgetID;

    #[inline]
    fn render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        ctx.build_intercept()._render(self, path, stack, renderer, force_render, cache, root, ctx)
    }
    #[inline]
    fn event_direct<P,Ph,Evt>(
        &mut self,
        path: &Ph,
        stack: &P,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        ctx.build_intercept()._event_direct(self, path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        ctx.build_intercept()._size(self, path, stack, root, ctx)
    }

    /// ![RENDER](https://img.shields.io/badge/-render-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![RENDER](https://img.shields.io/badge/-render-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::render`]
    fn _render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::event`](Link::send_event)
    fn _event_direct<P,Ph,Evt>(
        &mut self,
        path: &Ph,
        stack: &P,
        event: &Evt, // what if e.g. bounds change, if it's validated by parents then it's not signaled here
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized;
    /// ![LAYOUT](https://img.shields.io/badge/-layout-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![LAYOUT](https://img.shields.io/badge/-layout-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::size`]
    fn _size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;

    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized;

    #[inline]
    fn end<Ph>(
        &mut self,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {}

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn childs(&self) -> Range<isize>;

    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>>;

    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>>;

    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, callback: F) where F: FnMut(WidgetChildDynResult<'a,E>);

    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>);

    fn resolve_child_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>>;

    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>>;

    fn collect_childs_dyn_range(&self, range: Range<isize>) -> Vec<WidgetChildDynResult<'_,E>> {
        let mut dest = Vec::with_capacity(range.len());
        self.childs_dyn(range, &mut |result| dest.push(result) );
        dest
    }

    fn collect_childs_dyn_range_mut(&mut self, range: Range<isize>) -> Vec<WidgetChildDynResultMut<'_,E>> {
        let mut dest = Vec::with_capacity(range.len());
        self.childs_dyn_mut(range, &mut |result| dest.push(result) );
        dest
    }

    fn send_mutation<Ph>(
        &mut self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized;

    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// Resolve a deep child item by the given relative path
    /// 
    /// An empty path will resolve to this widget
    /// 
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but through [`Widgets::widget`]
    fn resolve<'a>(
        &'a self,
        sub_path: &(dyn PathResolvusDyn<E>),
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<WidgetResolveDynResult<'a,E>,E::Error> {
        if sub_path.inner().is_none() {
            return Ok(WidgetResolveDynResult {
                widget_id: self.id(),
                widget: self.erase(),
            });
        }

        if let Some(child) = self.resolve_child_dyn(sub_path) {
            child.widget.resolve(child.sub_path, root, ctx)
        } else {
            Err(().into()) //TODO
        }
    }

    // /// ![LAYOUT](https://img.shields.io/badge/-resolving-000?style=flat-square)
    // #[inline]
    // fn trace_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     if i.is_empty() {
    //         return Ok(*b)
    //     }
    //     let (child,_) = self.resolve_child(&i,root.fork(),ctx)?;
    //     let bounds = self.child_bounds(stack,b,force,root,ctx)?;
        
    //     Ok(bounds[child])
    // }
    // /// ![LAYOUT](https://img.shields.io/badge/-resolving-000?style=flat-square)
    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;
    
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// Attach widget's id to the given parent path

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
    fn _tabulate_next_child<P,Ph>(&self, path: &Ph, stack: &P, origin: TabulateNextChildOrigin, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> TabulateNextChildResponse where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let childs = self.childs();
        
        match origin {
            // This widget is entered
            TabulateNextChildOrigin::Enter => match dir {
                // This widget is entered forwards, if focusable tabulate into this
                TabulateDirection::Forward if self.focusable() => TabulateNextChildResponse::This,
                // Entered forwards, not focusable but has childs, try tabulate into first child
                TabulateDirection::Forward if !childs.is_empty() => TabulateNextChildResponse::Child(childs.start),
                // Entered backwards and has childs, try tabulate into last child
                TabulateDirection::Backward if !childs.is_empty()=> TabulateNextChildResponse::Child(childs.end - 1),
                // Entered backwards, doesn't have childs but is focusable, try tabulate into this
                TabulateDirection::Backward if self.focusable() => TabulateNextChildResponse::This,
                // No childs and not focusable, leave this widget (resumes traverse in parent widget)
                _ => TabulateNextChildResponse::Leave,
            }
            // This widget was focused, tabulate away from this widget
            TabulateNextChildOrigin::This => match dir {
                // If forward and has childs, tabulate into first child
                TabulateDirection::Forward if !childs.is_empty() => TabulateNextChildResponse::Child(childs.start),
                // Else, leave this widget (resumes traverse in parent widget)
                _ => TabulateNextChildResponse::Leave,
            }
            // Tabulate from previous child of this widget
            TabulateNextChildOrigin::Child(child_id) => match dir { //assert!(child_id < self.childs());
                // If forward and child after origin child, tabulate into next child
                TabulateDirection::Forward if child_id < childs.end.saturating_sub(1) => TabulateNextChildResponse::Child(child_id+1),
                // If backwards, and child before origin child, tabulate into previous child
                TabulateDirection::Backward if !childs.is_empty() && child_id > childs.start => TabulateNextChildResponse::Child(child_id-1),
                // If backwards, and no childs before origin child, but this widget is focusable, tabulate into this widget
                TabulateDirection::Backward if self.focusable() => TabulateNextChildResponse::This,
                // Else, leave this widget (resumes traverse in parent widget)
                _ => TabulateNextChildResponse::Leave,
            }
        }
    }

    #[deprecated]
    fn _call_tabulate_on_child_idx<P,Ph>(&self, idx: isize, path: &Ph, stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized;

    fn _tabulate<P,Ph>(&self, path: &Ph, stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let current_path = path;
        // fn to tabulate to the next child away from the previous child (child_id None = self)
        let enter_child_sub = |child_id: isize, to: TabulateOrigin<E>, ctx: &mut E::Context<'_>| -> Result<TabulateResponse<E>,E::Error> {
            self._call_tabulate_on_child_idx(child_id, path, stack, to.clone(), dir, root.fork(), ctx)
        };
        let next_child = |mut child_id: Option<isize>, ctx: &mut E::Context<'_>| -> Result<TabulateResponse<E>,E::Error> {
            loop {
                // determine the targeted next child
                let targeted_child = self._tabulate_next_child(
                    path,
                    stack,
                    TabulateNextChildOrigin::child_or_this(child_id),
                    dir,
                    root.fork(), ctx,
                );

                match targeted_child {
                    // enter child or repeat
                    TabulateNextChildResponse::Child(t) => match enter_child_sub(t,TabulateOrigin::Enter,ctx)? {
                        TabulateResponse::Done(v) => return Ok(TabulateResponse::Done(v)),
                        TabulateResponse::Leave => {
                            // couldn't enter next child, repeat
                            child_id = Some(t);
                            continue
                        },
                    }
                    TabulateNextChildResponse::This =>
                        if self.focusable() {
                            return Ok(TabulateResponse::Done((current_path.to_resolvus(),self.id())))
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
        let enter_child = |child_id: isize, to: TabulateOrigin<E>, ctx: &mut E::Context<'_>| -> Result<TabulateResponse<E>,E::Error> {
            match enter_child_sub(child_id,to,ctx)? {
                TabulateResponse::Done(v) => Ok(TabulateResponse::Done(v)),
                TabulateResponse::Leave => next_child(Some(child_id),ctx),
            }
        };
        match op {
            TabulateOrigin::Resolve(p) => {
                if p.inner().is_some() {
                    // pass 1: resolve to previous focused widget
                    match self.resolve_child_dyn(p) {
                        Some(result) => enter_child(result.idx, TabulateOrigin::Resolve(result.sub_path),ctx),
                        None => Err(todo!()),
                    }
                }else{
                    // pass 2: we are the previous focused widget and should tabulate away
                    next_child(None,ctx)
                }
            },
            TabulateOrigin::Enter => {
                // we got entered from the parent widget

                let enter_dir = self._tabulate_next_child(
                    path,
                    stack,
                    TabulateNextChildOrigin::Enter,
                    dir,
                    root.fork(), ctx,
                );

                match enter_dir {
                    // tabulate into enter the targeted child
                    TabulateNextChildResponse::Child(t) => enter_child(t, TabulateOrigin::Enter,ctx),
                    // tabulate to self
                    TabulateNextChildResponse::This => Ok(TabulateResponse::Done((current_path.to_resolvus(),self.id()))),
                    TabulateNextChildResponse::Leave => Ok(TabulateResponse::Leave),
                }
            },
        }
    }
    
    #[inline]
    fn inner<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        None
    }
    #[inline]
    fn innest<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
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
    fn inner_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        None
    }
    #[inline]
    fn innest_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
        // let mut i = self.erase_mut();
        // loop {
        //     let v = i.inner_mut();
        //     if let Some(v) = v {
        //         i = v;
        //     }else{
        //         return Some(i);
        //     }
        // }
        todo!()
    }
    
    fn invalidate_recursive(&mut self, vali: Invalidation);

    //fn mon(&self);

    #[inline]
    fn downcast<'a,T>(&'a self) -> Option<&'a T> where T: ?Sized + 'static, Self: 'static {
        let mut response: Option<&'a T> = None;
        self.respond_downcast(DowncastResponder::new::<T>(&mut response));
        response
    }
    #[inline]
    fn downcast_mut<'a,T>(&'a mut self) -> Option<&'a mut T> where T: ?Sized + 'static, Self: 'static {
        let mut response: Option<&'a mut T> = None;
        self.respond_downcast_mut(DowncastMutResponder::new::<T>(&mut response));
        response
    }
    #[inline]
    fn respond_downcast<'a>(&'a self, mut responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        }
    }
    #[inline]
    fn respond_downcast_mut<'a>(&'a mut self, mut responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        }
    }
    #[inline]
    fn downcast_recursive<'a,T>(&'a self) -> Option<&'a T> where T: ?Sized + 'static, Self: 'static {
        let mut response: Option<&'a T> = None;
        self.respond_downcast_recursive(DowncastResponder::new::<T>(&mut response));
        response
    }
    #[inline]
    fn downcast_recursive_mut<'a,T>(&'a mut self) -> Option<&'a mut T> where T: ?Sized + 'static, Self: 'static {
        let mut response: Option<&'a mut T> = None;
        self.respond_downcast_recursive_mut(DowncastMutResponder::new::<T>(&mut response));
        response
    }
    #[inline]
    fn respond_downcast_recursive<'a>(&'a self, mut responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        } else if let Some(v) = self.inner() {
            v.respond_downcast_recursive(responder)
        }
    }
    #[inline]
    fn respond_downcast_recursive_mut<'a>(&'a mut self, mut responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        } else if let Some(v) = self.inner_mut() {
            v.respond_downcast_recursive_mut(responder)
        }
    }

    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        dest.push(self._wbase_type_name());
    }
    fn debugged_type_name(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        self.debug_type_name(&mut v);
        // v.shrink_to_fit();
        v
    }

    #[inline]
    fn pass(self) -> Self where Self: Sized {
        self
    }

    #[inline]
    fn query<'a,T>(&'a self) -> Option<T::Result<'a>> where T: WQuery<E> + ?Sized, Self: 'a {
        let mut response: Option<T::Result<'a>> = None;
        self.respond_query(WQueryResponder::new::<T>(&mut response));
        response
    }

    #[inline]
    fn query_mut<'a,T>(&'a mut self) -> Option<T::Result<'a>> where T: WQuery<E> + ?Sized, Self: 'a {
        let mut response: Option<T::Result<'a>> = None;
        self.respond_query_mut(WQueryResponder::new::<T>(&mut response));
        response
    }

    #[inline]
    fn query_generic<'a,T,G>(&'a self) -> Option<T::Result<'a,G>> where T: WQueryGeneric<E> + ?Sized, G: ?Sized, Self: 'a {
        let mut response: Option<T::Result<'a,G>> = None;
        self.respond_query_generic::<T,G>(WQueryResponderGeneric::new(&mut response));
        response
    }

    fn respond_query<'a>(&'a self, responder: WQueryResponder<'_,'a,E>);

    fn respond_query_mut<'a>(&'a mut self, responder: WQueryResponder<'_,'a,E>);

    #[inline]
    fn respond_query_generic<'a,Q,G>(&'a self, responder: WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: WQueryGeneric<E> + ?Sized, G: ?Sized {}

    /// Use this to turn to dyn Widget
    /// 
    /// This avoids flattening through dyn to be zero-cost
    #[inline]
    fn erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        WBase::_wbase_erase(self)
    }

    /// Use this to turn to dyn Widget
    /// 
    /// This avoids flattening through dyn to be zero-cost
    #[inline]
    fn erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        WBase::_wbase_erase_mut(self)
    }

    /// Use this to turn to dyn Widget
    /// 
    /// This also flattens direct layers of Box and dyn
    #[inline]
    fn erase2<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        WBase::_wbase_erase(self)
    }

    /// Use this to turn to dyn Widget
    /// 
    /// This also flattens direct layers of Box and dyn
    #[inline]
    fn erase2_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        WBase::_wbase_erase_mut(self)
    }

    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// Move widget into box immutable. Use [`WidgetMut::box_box_mut`] to box into mutable [`WidgetRef`](WidgetRefMut).
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        WBase::_wbase_box_box(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// Move widget into box immutable. Use [`WidgetMut::boxed_mut`] to box into mutable [`WidgetRef`](WidgetRefMut).
    #[inline]
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        WBase::_wbase_boxed(self)
    }

    #[inline(never)]
    fn gen_diag_error_resolve_fail(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error {
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

        let child_info = {
            let childs = self.childs();

            let mut dest = Vec::with_capacity(childs.len());

            self.childs_dyn(childs, #[inline] |w| 
                dest.push( w.widget.guion_resolve_error_child_info(w.idx) )
            );

            dest
        };

        GuionError::ResolveError(Box::new(ResolveError{
            op,
            sub_path: todo!(),
            widget_type,
            child_info,
        })).into()
    }

    #[inline(never)]
    fn guion_resolve_error_child_info(&self, child_idx: isize) -> GuionResolveErrorChildInfo<E> {
        GuionResolveErrorChildInfo {
            child_idx,
            widget_type: self.debugged_type_name(),
            // widget_path_if_path: None,
            // widget_id: Some(self.id()),
            path: todo!(),
        }
    }
}

/// This trait is blanket implemented for all widget and provides functions which require compile-time knowledge of types
#[doc(hidden)]
pub trait WBase<E> where E: Env {
    fn _wbase_type_name(&self) -> &'static str;
    fn _wbase_erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's;
    fn _wbase_erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's;
    fn _wbase_box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w;
    fn _wbase_boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w;
    fn _wbase_as_any(&self) -> &dyn std::any::Any where Self: 'static;
    fn _wbase_as_any_mut(&mut self) -> &mut dyn std::any::Any where Self: 'static;
}
impl<T,E> WBase<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn _wbase_type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    #[inline]
    fn _wbase_erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self
    }
    #[inline]
    fn _wbase_erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self
    }
    #[inline]
    fn _wbase_box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self
    }
    #[inline]
    fn _wbase_boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        Box::new(self)
    }
    #[inline]
    fn _wbase_as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    #[inline]
    fn _wbase_as_any_mut(&mut self) -> &mut dyn std::any::Any where Self: 'static {
        self
    }
}
