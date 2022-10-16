use std::marker::PhantomData;

use crate::aliases::{EStyle, ERenderer};
use crate::env::Env;
use crate::event_new::EventDyn;
use crate::event_new::downcast_map::EventDowncastMap;
use crate::newpath::PathStackDyn;
use crate::queron::dyn_tunnel::QueronDyn;

use super::*;
use super::cache::DynWidgetCache;

pub trait WidgetDyn<E> where E: Env + 'static {
    fn render_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    );

    fn event_direct_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn EventDyn<E>+'_),
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp;

    fn size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E>;

    fn _render_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    );

    fn _event_direct_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn EventDyn<E>+'_),
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp;

    fn _size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E>;

    fn childs_dyn(&self) -> usize;

    #[deprecated]
    fn with_child_dyn<'s>(
        &'s self,
        i: usize,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> ProtectedReturn,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;

    #[deprecated]
    fn childs_ref_dyn<'s>(
        &'s self,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut E::Context<'cc>),
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    );
    
    // #[deprecated]
    // fn child_paths_dyn(&self, own_path: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<E::WidgetPath>;

    fn with_resolve_dyn<'s>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> ProtectedReturn,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;

    fn with_resolve_child_dyn<'s>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        callback: &mut dyn for<'w,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'w,E>,E::Error>,&'c mut E::Context<'cc>) -> ProtectedReturn,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;

    // fn resolve_child_dyn(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(usize,E::WidgetPath),E::Error>;

    // fn trace_bounds_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
    //     stack: &(dyn QueronDyn<E>+'_), i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error>;

    // fn child_bounds_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
    //     stack: &(dyn QueronDyn<E>+'_), b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()>;

    fn focusable_dyn(&self) -> bool;

    fn _focus_on_mouse_down_dyn(&self) -> bool;

    fn _tabulate_by_tab_dyn(&self) -> bool;

    #[deprecated="Not supposted to be exposed"]
    fn _tabulate_next_child_dyn(&self, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), origin: TabulateNextChildOrigin, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> TabulateNextChildResponse;

    fn _call_tabulate_on_child_idx_dyn(&self, child_idx: usize, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error>;

    fn _tabulate_dyn(&self, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error>;
    
    fn inner_dyn<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's;

    fn innest_dyn(&self) -> Option<&dyn WidgetDyn<E>>;

    fn as_any_dyn(&self) -> &dyn std::any::Any where Self: 'static;

    fn debug_type_name_dyn(&self, dest: &mut Vec<&'static str>);
    fn debugged_type_name_dyn(&self) -> Vec<&'static str>;

    unsafe fn _as_trait_ref_dyn(&self, t: TypeId) -> Option<TraitObject>;

    fn erase_dyn<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's;

    fn box_ref_dyn<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's;

    fn box_box_dyn<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w;

    fn gen_diag_error_resolve_fail_dyn(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error;

    fn guion_resolve_error_child_info_dyn(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E>;
}

impl<T,E> WidgetDyn<E> for T where T: Widget<E> + ?Sized, E: Env {
    #[inline]
    fn render_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.render(path, stack, renderer, force_render, cache.downcast_mut_or_reset::<T::Cache>(), root, ctx)
    }
    #[inline]
    fn event_direct_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn EventDyn<E>+'_),
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp {
        E::EventDowncastMap::event_downcast_map(self, path, stack, event, route_to_widget, cache.downcast_mut_or_reset::<T::Cache>(), root, ctx)
    }
    #[inline]
    fn size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> {
        self.size(path, stack, cache.downcast_mut_or_reset::<T::Cache>(), root, ctx)
    }
    #[inline]
    fn _render_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self._render(path, stack, renderer, force_render, cache.downcast_mut_or_reset::<T::Cache>(), root, ctx)
    }
    #[inline]
    fn _event_direct_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn EventDyn<E>+'_),
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp {
        todo!()
        //self._event_direct(stack, event, cache.downcast_mut_or_reset::<T::Cache>(), root, ctx)
    }
    #[inline]
    fn _size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        cache: &mut DynWidgetCache<E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> {
        self._size(path, stack, cache.downcast_mut_or_reset::<T::Cache>(), root, ctx)
    }
    #[inline]
    fn childs_dyn(&self) -> usize {
        self.childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn with_child_dyn<'s>(
        &'s self,
        i: usize,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> ProtectedReturn,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn {
        self.with_child(i, callback, root, ctx)
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref_dyn<'s>(
        &'s self,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut E::Context<'cc>),
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) {
        self.childs_ref(callback, root, ctx)
    }
    // #[allow(deprecated)]
    // #[inline]
    // fn child_paths_dyn(&self, own_path: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
    //     self.child_paths(own_path, root, ctx)
    // }
    #[inline]
    fn with_resolve_dyn<'s>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> ProtectedReturn,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn {
        self.with_resolve(sub_path, callback, root, ctx)
    }
    #[inline]
    fn with_resolve_child_dyn<'s>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        callback: &mut dyn for<'w,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'w,E>,E::Error>,&'c mut E::Context<'cc>) -> ProtectedReturn,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn {
        self.with_resolve_child(sub_path, callback, root, ctx)
    }
    // #[inline]
    // fn resolve_child_dyn(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(usize,E::WidgetPath),E::Error> {
    //     self.resolve_child(sub_path, root, ctx)
    // }
    // #[inline]
    // fn trace_bounds_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
    //     stack: &(dyn QueronDyn<E>+'_), i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error> {
    //     self.trace_bounds(path, stack, i, b, e, force, root, ctx)
    // }
    // #[inline]
    // fn child_bounds_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
    //     stack: &(dyn QueronDyn<E>+'_), b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> {
    //     self.child_bounds(path, stack, b, force, root, ctx)
    // }
    #[inline]
    fn focusable_dyn(&self) -> bool {
        self.focusable()
    }
    #[inline]
    fn _focus_on_mouse_down_dyn(&self) -> bool {
        self._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab_dyn(&self) -> bool {
        self._tabulate_by_tab()
    }
    #[inline]
    fn _tabulate_next_child_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_), origin: TabulateNextChildOrigin, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> TabulateNextChildResponse {
        self._tabulate_next_child(path, stack, origin, dir, root, ctx)
    }
    #[inline]
    fn _tabulate_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error> {
        self._tabulate(path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn inner_dyn<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner()
    }
    #[inline]
    fn innest_dyn(&self) -> Option<&dyn WidgetDyn<E>> {
        self.innest()
    }
    #[inline]
    fn as_any_dyn(&self) -> &dyn std::any::Any where Self: 'static {
        Widget::as_any(self)
    }
    #[inline]
    fn debug_type_name_dyn(&self, dest: &mut Vec<&'static str>) {
        self.debug_type_name(dest)
    }
    #[inline]
    fn debugged_type_name_dyn(&self) -> Vec<&'static str> {
        self.debugged_type_name()
    }
    #[inline]
    unsafe fn _as_trait_ref_dyn(&self, t: TypeId) -> Option<TraitObject> {
        self._as_trait_ref(t)
    }
    #[inline]
    fn erase_dyn<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self.erase()
    }
    #[inline]
    fn box_ref_dyn<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's {
        self.box_ref()
    }
    #[inline]
    fn box_box_dyn<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self.box_box()
    }
    #[inline]
    fn gen_diag_error_resolve_fail_dyn(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error {
        self.gen_diag_error_resolve_fail(sub_path, op, root, ctx)
    }
    #[inline]
    fn guion_resolve_error_child_info_dyn(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        self.guion_resolve_error_child_info(child_idx)
    }
    #[inline]
    fn _call_tabulate_on_child_idx_dyn(&self, child_idx: usize, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Result<TabulateResponse<E>,<E as Env>::Error> {
        self._call_tabulate_on_child_idx(child_idx, path, stack, op, dir, root, ctx)
    }
}

impl<E> Widget<E> for dyn WidgetDyn<E> + '_ where E: Env {
    #[inline]
    fn render<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.render_dyn(path._erase(), stack.erase(), renderer, force_render, cache, root, ctx)
    }
    #[inline]
    fn event_direct<P,Ph,Evt>(
        &self,
        path: &Ph,
        stack: &P,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        self.event_direct_dyn(path._erase(), stack.erase(), event.erase(), route_to_widget, cache, root, ctx)
    }
    #[inline]
    fn size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.size_dyn(path._erase(), stack.erase(), cache, root, ctx)
    }
    #[inline]
    fn _render<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self._render_dyn(path._erase(), stack.erase(), renderer, force_render, cache, root, ctx)
    }
    #[inline]
    fn _event_direct<P,Ph,Evt>(
        &self,
        path: &Ph,
        stack: &P,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        self._event_direct_dyn(path._erase(), stack.erase(), event.erase(), route_to_widget, cache, root, ctx)
    }
    #[inline]
    fn _size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self._size_dyn(path._erase(), stack.erase(), cache, root, ctx)
    }
    #[inline]
    fn childs(&self) -> usize {
        self.childs_dyn()
    }
    #[allow(deprecated)]
    #[inline]
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        let mut callback_return: Option<R> = None;
        self.with_child_dyn(
            i,
            &mut |w,ctx| {
                let r = (callback)(w,ctx);
                callback_return = Some(r);
                ProtectedReturn(PhantomData)
            },
            root, ctx,
        );
        callback_return.unwrap()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref<'s,F>(
        &'s self,
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    )
    where
        F: for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut E::Context<'cc>)
    {
        self.childs_ref_dyn(&mut callback, root, ctx)
    }
    // #[allow(deprecated)]
    // #[inline]
    // fn child_paths(&self, own_path: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
    //     self.child_paths_dyn(own_path, root, ctx)
    // }
    #[allow(deprecated)]
    #[inline]
    fn with_resolve<'s,F,R>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> R
    {
        let mut callback_return: Option<R> = None;
        self.with_resolve_dyn(
            sub_path,
            &mut |w,ctx| {
                let r = (callback)(w,ctx);
                callback_return = Some(r);
                ProtectedReturn(PhantomData)
            },
            root, ctx,
        );
        callback_return.unwrap()
    }

    fn with_resolve_child<'s,F,R>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'w,E>,E::Error>,&'c mut E::Context<'cc>) -> R
    {
        let mut callback_return: Option<R> = None;
        self.with_resolve_child_dyn(
            sub_path,
            &mut |w,ctx| {
                let r = (callback)(w,ctx);
                callback_return = Some(r);
                ProtectedReturn(PhantomData)
            },
            root, ctx,
        );
        callback_return.unwrap()
    }
    // #[inline]
    // fn resolve_child(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(usize,E::WidgetPath),E::Error> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
    //     self.resolve_child_dyn(sub_path, root, ctx)
    // }
    // #[inline]
    // fn trace_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     self.trace_bounds_dyn(path._erase(), stack.erase(), i, b, e, force, root, ctx)
    // }
    // #[inline]
    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     self.child_bounds_dyn(path._erase(), stack.erase(), b, force, root, ctx)
    // }
    #[inline]
    fn focusable(&self) -> bool {
        self.focusable_dyn()
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        self._focus_on_mouse_down_dyn()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        self._tabulate_by_tab_dyn()
    }
    #[allow(deprecated)]
    #[inline]
    fn _tabulate_next_child<P,Ph>(&self, path: &Ph,
        stack: &P, origin: TabulateNextChildOrigin, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> TabulateNextChildResponse where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self._tabulate_next_child_dyn(path._erase(), stack.erase(), origin, dir, root, ctx)
    }
    #[inline]
    fn _tabulate<P,Ph>(&self, path: &Ph,
        stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self._tabulate_dyn(path._erase(), stack.erase(), op, dir, root, ctx)
    }
    #[inline]
    fn inner<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner_dyn()
    }
    #[inline]
    fn innest(&self) -> Option<&dyn WidgetDyn<E>> {
        self.innest_dyn()
    }
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self.as_any_dyn()
    }
    #[inline]
    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        self.debug_type_name_dyn(dest) //TODO push WidgetDyn use on debug
    }
    #[inline]
    fn debugged_type_name(&self) -> Vec<&'static str> {
        self.debugged_type_name_dyn()
    }
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        self._as_trait_ref_dyn(t)
    }
    #[inline]
    fn erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self //return this dyn WidgetDyn for perf
    }
    #[inline]
    fn box_ref<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's {
        self.box_ref_dyn() //TODO deprecate box_
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self.box_box_dyn()
    }
    #[inline]
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        unreachable!()
    }

    //TODO cold
    fn gen_diag_error_resolve_fail(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error {
        self.gen_diag_error_resolve_fail_dyn(sub_path, op, root, ctx)
    }

    //TODO cold
    fn guion_resolve_error_child_info(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        self.guion_resolve_error_child_info_dyn(child_idx)
    }

    type Cache = DynWidgetCache<E>;

    #[inline]
    fn _call_tabulate_on_child_idx<P,Ph>(&self, idx: usize, path: &Ph, stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized
    {
        self._call_tabulate_on_child_idx_dyn(idx, path._erase(), stack.erase(), op, dir, root, ctx)
    }
}

impl<E> WBase<E> for dyn WidgetDyn<E> + '_ where E: Env {
    fn type_name(&self) -> &'static str {
        todo!()
    }

    fn _erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self
    }

    fn _box_ref<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's {
        self.box_ref_dyn()
    }

    fn _box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self.box_box_dyn()
    }

    fn _boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        unreachable!()
    }

    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self.as_any_dyn()
    }
}
