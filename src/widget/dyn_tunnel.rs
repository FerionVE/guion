use std::any::Any;
use std::ops::Range;

use crate::event_new::downcast_map::EventDowncastMap;
use crate::queron::Queron;
use crate::traitcast::{WQueryResponder, WQueryResponderGeneric, WQueryGeneric, DowncastMutResponder, DowncastResponder};
use crate::util::error::GuionResolveErrorChildInfo;
use crate::util::tabulate::{TabulateNextChildOrigin, TabulateDirection, TabulateOrigin, TabulateNextChildResponse, TabulateResponse};
use crate::widget_decl::route::UpdateRoute;
use crate::{EventResp, event_new};
use crate::aliases::{ERenderer, ESize};
use crate::env::Env;
use crate::event_new::EventDyn;
use crate::newpath::{PathStackDyn, PathResolvusDyn, PathStack};
use crate::queron::dyn_tunnel::QueronDyn;

use super::id::WidgetID;
use super::{Widget, WBase, WidgetChildDynResult, WidgetChildDynResultMut, WidgetChildResolveDynResult, WidgetChildResolveDynResultMut, WidgetResolveDynResult};
use super::cache::DynWidgetCache;

pub trait WidgetDyn<E> where E: Env + 'static {
    fn id_dyn(&self) -> WidgetID;

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
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp;

    fn size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
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
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp;

    fn _size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E>;

    fn update_dyn(
        &mut self,
        path: &(dyn PathStackDyn<E>+'_),
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    );

    fn end_dyn(
        &mut self,
        path: &(dyn PathStackDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    );

    fn childs_dyn(&self) -> Range<isize>;

    fn child_dyn_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>>;

    fn child_dyn_mut_dyn(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>>;

    fn childs_dyn_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(WidgetChildDynResult<'a,E>) + '_) );

    fn childs_dyn_mut_dyn<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(WidgetChildDynResultMut<'a,E>) + '_) );

    fn resolve_child_dyn_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>>;

    fn resolve_child_dyn_mut_dyn<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>>;

    fn collect_childs_dyn_range_dyn(&self, range: Range<isize>) -> Vec<WidgetChildDynResult<'_,E>>;

    fn collect_childs_dyn_range_mut_dyn(&mut self, range: Range<isize>) -> Vec<WidgetChildDynResultMut<'_,E>>;

    fn send_mutation_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    );

    fn resolve_dyn<'a>(
        &'a self,
        sub_path: &(dyn PathResolvusDyn<E>),
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<WidgetResolveDynResult<'a,E>,E::Error>;

    // fn resolve_child_dyn(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(isize,E::WidgetPath),E::Error>;

    // fn trace_bounds_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
    //     stack: &(dyn QueronDyn<E>+'_), i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error>;

    // fn child_bounds_dyn(&self, path: &(dyn PathStackDyn<E>+'_),
    //     stack: &(dyn QueronDyn<E>+'_), b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()>;

    fn focusable_dyn(&self) -> bool;

    fn _focus_on_mouse_down_dyn(&self) -> bool;

    fn _tabulate_by_tab_dyn(&self) -> bool;

    #[deprecated="Not supposted to be exposed"]
    fn _tabulate_next_child_dyn(&self, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), origin: TabulateNextChildOrigin, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> TabulateNextChildResponse;

    fn _call_tabulate_on_child_idx_dyn(&self, child_idx: isize, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error>;

    fn _tabulate_dyn(&self, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error>;
    
    fn inner_dyn<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's;
    fn innest_dyn<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's;

    fn inner_mut_dyn<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's;
    fn innest_mut_dyn<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's;

    fn respond_downcast_dyn<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) where Self: 'static;
    fn respond_downcast_mut_dyn<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) where Self: 'static;
    fn respond_downcast_recursive_dyn<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) where Self: 'static;
    fn respond_downcast_recursive_mut_dyn<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) where Self: 'static;

    fn debug_type_name_dyn(&self, dest: &mut Vec<&'static str>);

    fn respond_query_dyn<'a>(&'a self, t: WQueryResponder<'_,'a,E>);
    fn respond_query_mut_dyn<'a>(&'a mut self, t: WQueryResponder<'_,'a,E>);

    fn erase2_dyn<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's;
    fn erase2_mut_dyn<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's;

    fn box_box_dyn<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w;

    fn gen_diag_error_resolve_fail_dyn(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error;

    fn guion_resolve_error_child_info_dyn(&self, child_idx: isize) -> GuionResolveErrorChildInfo<E>;
}

impl<T,E> WidgetDyn<E> for T where T: Widget<E> + ?Sized, E: Env {
    #[inline]
    fn id_dyn(&self) -> WidgetID {
        self.id()
    }
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
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> EventResp {
        E::EventDowncastMap::event_downcast_map(self, path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn size_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> {
        self.size(path, stack, root, ctx)
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
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> {
        self._size(path, stack, root, ctx)
    }
    #[inline]
    fn update_dyn(
        &mut self,
        path: &(dyn PathStackDyn<E>+'_),
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.update(path, route, root, ctx)
    }
    #[inline]
    fn end_dyn(
        &mut self,
        path: &(dyn PathStackDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.end(path, root, ctx)
    }
    #[inline]
    fn childs_dyn(&self) -> Range<isize> {
        self.childs()
    }
    #[inline]
    fn send_mutation_dyn(
        &self,
        path: &(dyn PathStackDyn<E>+'_),
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.send_mutation(path, resolve, args, root, ctx)
    }

    // #[allow(deprecated)]
    // #[inline]
    // fn child_paths_dyn(&self, own_path: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
    //     self.child_paths(own_path, root, ctx)
    // }

    // #[inline]
    // fn resolve_child_dyn(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(isize,E::WidgetPath),E::Error> {
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
    #[allow(deprecated)]
    #[inline]
    fn _call_tabulate_on_child_idx_dyn(&self, child_idx: isize, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error> {
        self._call_tabulate_on_child_idx(child_idx, path, stack, op, dir, root, ctx)
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
    fn innest_dyn<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        self.innest()
    }
    #[inline]
    fn inner_mut_dyn<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner_mut()
    }
    #[inline]
    fn innest_mut_dyn<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        self.innest_mut()
    }
    #[inline]
    fn respond_downcast_dyn<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast(responder)
    }
    #[inline]
    fn respond_downcast_mut_dyn<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_mut(responder)
    }
    #[inline]
    fn respond_downcast_recursive_dyn<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_recursive(responder)
    }
    #[inline]
    fn respond_downcast_recursive_mut_dyn<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_recursive_mut(responder)
    }
    #[inline]
    fn debug_type_name_dyn(&self, dest: &mut Vec<&'static str>) {
        self.debug_type_name(dest)
    }
    #[inline]
    fn respond_query_dyn<'a>(&'a self, t: WQueryResponder<'_,'a,E>) {
        self.respond_query(t)
    }
    #[inline]
    fn respond_query_mut_dyn<'a>(&'a mut self, t: WQueryResponder<'_,'a,E>) {
        self.respond_query_mut(t)
    }
    #[inline]
    fn erase2_dyn<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self.erase2()
    }
    #[inline]
    fn erase2_mut_dyn<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self.erase2_mut()
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
    fn guion_resolve_error_child_info_dyn(&self, child_idx: isize) -> GuionResolveErrorChildInfo<E> {
        self.guion_resolve_error_child_info(child_idx)
    }
    #[inline]
    fn child_dyn_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        self.child_dyn(idx)
    }
    #[inline]
    fn child_dyn_mut_dyn(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        self.child_dyn_mut(idx)
    }
    #[inline]
    fn childs_dyn_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(WidgetChildDynResult<'a,E>) + '_) ) {
        self.childs_dyn(range, callback)
    }
    #[inline]
    fn childs_dyn_mut_dyn<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(WidgetChildDynResultMut<'a,E>) + '_) ) {
        self.childs_dyn_mut(range, callback)
    }
    #[inline]
    fn resolve_child_dyn_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        self.resolve_child_dyn(path)
    }
    #[inline]
    fn resolve_child_dyn_mut_dyn<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        self.resolve_child_dyn_mut(path)
    }
    #[inline]
    fn collect_childs_dyn_range_dyn(&self, range: Range<isize>) -> Vec<WidgetChildDynResult<'_,E>> {
        self.collect_childs_dyn_range(range)
    }
    #[inline]
    fn collect_childs_dyn_range_mut_dyn(&mut self, range: Range<isize>) -> Vec<WidgetChildDynResultMut<'_,E>> {
        self.collect_childs_dyn_range_mut(range)
    }
    #[inline]
    fn resolve_dyn<'a>(
        &'a self,
        sub_path: &(dyn PathResolvusDyn<E>),
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<WidgetResolveDynResult<'a,E>,E::Error> {
        self.resolve(sub_path, root, ctx)
    }
}

impl<E> Widget<E> for dyn WidgetDyn<E> + '_ where E: Env {
    type Cache = DynWidgetCache<E>;
    
    #[inline]
    fn id(&self) -> WidgetID {
        self.id_dyn()
    }
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
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        self.event_direct_dyn(path._erase(), stack.erase(), event.erase(), route_to_widget, root, ctx)
    }
    #[inline]
    fn size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.size_dyn(path._erase(), stack.erase(), root, ctx)
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
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        self._event_direct_dyn(path._erase(), stack.erase(), event.erase(), route_to_widget, root, ctx)
    }
    #[inline]
    fn _size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self._size_dyn(path._erase(), stack.erase(), root, ctx)
    }
    #[inline]
    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        self.update_dyn(path._erase(), route, root, ctx)
    }
    #[inline]
    fn end<Ph>(
        &mut self,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        self.end_dyn(path._erase(), root, ctx)
    }
    #[inline]
    fn childs(&self) -> Range<isize> {
        self.childs_dyn()
    }

    #[inline]
    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        self.child_dyn_dyn(idx)
    }
    #[inline]
    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        self.child_dyn_mut_dyn(idx)
    }
    #[inline]
    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        self.childs_dyn_dyn(range, &mut callback)
    }
    #[inline]
    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        self.childs_dyn_mut_dyn(range, &mut callback)
    }
    #[inline]
    fn resolve_child_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        self.resolve_child_dyn_dyn(path)
    }
    #[inline]
    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        self.resolve_child_dyn_mut_dyn(path)
    }
    #[inline]
    fn collect_childs_dyn_range(&self, range: Range<isize>) -> Vec<WidgetChildDynResult<'_,E>> {
        self.collect_childs_dyn_range_dyn(range)
    }
    #[inline]
    fn collect_childs_dyn_range_mut(&mut self, range: Range<isize>) -> Vec<WidgetChildDynResultMut<'_,E>> {
        self.collect_childs_dyn_range_mut_dyn(range)
    }
    #[inline]
    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        self.send_mutation_dyn(path._erase(), resolve, args, root, ctx)
    }
    #[inline]
    fn resolve<'a>(
        &'a self,
        sub_path: &(dyn PathResolvusDyn<E>),
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<WidgetResolveDynResult<'a,E>,E::Error> {
        self.resolve_dyn(sub_path, root, ctx)
    }
    // #[inline]
    // fn trace_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Bounds,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     self.trace_bounds_dyn(path._erase(), stack.erase(), i, b, e, force, root, ctx)
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
    fn _call_tabulate_on_child_idx<P,Ph>(&self, idx: isize, path: &Ph, stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized
    {
        self._call_tabulate_on_child_idx_dyn(idx, path._erase(), stack.erase(), op, dir, root, ctx)
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
    fn innest<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        self.innest_dyn()
    }
    #[inline]
    fn inner_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner_mut_dyn()
    }
    #[inline]
    fn innest_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        self.innest_mut_dyn()
    }
    #[inline]
    fn respond_downcast<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_dyn(responder)
    }
    #[inline]
    fn respond_downcast_mut<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_mut_dyn(responder)
    }
    #[inline]
    fn respond_downcast_recursive<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_recursive_dyn(responder)
    }
    #[inline]
    fn respond_downcast_recursive_mut<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        self.respond_downcast_recursive_mut_dyn(responder)
    }

    #[inline]
    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        dest.push(self._wbase_type_name());
        self.debug_type_name_dyn(dest) //TODO push WidgetDyn use on debug
    }

    #[inline]
    fn respond_query<'a>(&'a self, t: WQueryResponder<'_,'a,E>) {
        self.respond_query_dyn(t)
    }

    #[inline]
    fn respond_query_mut<'a>(&'a mut self, t: WQueryResponder<'_,'a,E>) {
        self.respond_query_mut_dyn(t)
    }

    #[inline]
    fn respond_query_generic<'a,Q,G>(&'a self, _: WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: WQueryGeneric<E> + ?Sized, G: ?Sized {
        //TODO generic query not supported on dyn tunnel
    }

    #[inline]
    fn erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self //return this dyn WidgetDyn for perf
    }
    #[inline]
    fn erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self //return this dyn WidgetDyn for perf
    }
    #[inline]
    fn erase2<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self.erase2_dyn()
    }
    #[inline]
    fn erase2_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self.erase2_mut_dyn()
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
    fn guion_resolve_error_child_info(&self, child_idx: isize) -> GuionResolveErrorChildInfo<E> {
        self.guion_resolve_error_child_info_dyn(child_idx)
    }
}

impl<E> WBase<E> for dyn WidgetDyn<E> + '_ where E: Env {
    fn _wbase_type_name(&self) -> &'static str {
        std::any::type_name::<dyn WidgetDyn<E>>()
    }

    fn _wbase_erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self
    }

    fn _wbase_erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self
    }

    fn _wbase_box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self.box_box_dyn()
    }

    fn _wbase_boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        unreachable!()
    }

    fn _wbase_as_any(&self) -> &dyn Any where Self: 'static {
        todo!()
    }

    fn _wbase_as_any_mut(&mut self) -> &mut dyn Any where Self: 'static {
        todo!()
    }
}
