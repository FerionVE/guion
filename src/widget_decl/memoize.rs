use std::any::Any;
use std::marker::PhantomData;
use std::ops::Range;

use crate::aliases::{ESize, ERenderer};
use crate::env::Env;
use crate::event_new;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::render::StdRenderProps;
use crate::root::RootRef;
use crate::traitcast::{WQuery, WQueryResponder, WQueryResponderGeneric, WQueryGeneric, DowncastResponder, DowncastMutResponder};
use crate::util::tabulate;
use crate::widget::id::WidgetID;
use crate::widget::{Widget, WidgetResolveDynResult, WidgetChildDynResultMut, WidgetChildDynResult, WidgetChildResolveDynResultMut, WidgetChildResolveDynResult};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::WidgetDecl;
use super::mutor_trait::MutorEnd;
use super::route::UpdateRoute;

pub struct Memoize<M,T,E> where M: Clone + PartialEq + 'static, T: WidgetDecl<E>, E: Env {
    memoize: M,
    inner: T,
    _p: PhantomData<E>,
}

impl<M,T,E> Memoize<M,T,E> where M: Clone + PartialEq + 'static, T: WidgetDecl<E>, E: Env {
    #[inline]
    pub fn new(memoize: M, inner: T) -> Self {
        Self {
            memoize,
            inner,
            _p: PhantomData,
        }
    }
}

impl<M,T,E> WidgetDecl<E> for Memoize<M,T,E> where M: Clone + PartialEq + 'static, T: WidgetDecl<E>, E: Env {
    type Widget = MemoizeWidget<M,T::Widget,E>;

    #[inline]
    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.inner.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        MemoizeWidget {
            memoize: self.memoize,
            inner: self.inner.build(path, root, ctx),
            _p: PhantomData,
        }
    }

    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        MemoizeWidget {
            memoize: self.memoize.clone(),
            inner: self.inner.instantiate(path, root, ctx),
            _p: PhantomData,
        }
    }

    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        Box::new(self.build(path, root, ctx))
    }

    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        Box::new(self.instantiate(path, root, ctx))
    }

    #[inline]
    fn update(
        &self,
        w: &mut Self::Widget,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        if route.resolvus().is_some() || self.memoize != w.memoize {
            self.inner.update(&mut w.inner, path, route, root, ctx)
        } else {
            Invalidation::new()
        }
    }

    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        let (inner,vali) = if let Some(prev_inner) = prev.query_mut::<WQueryMemoizeRestore>() {
            self.inner.update_restore(prev_inner, path, root, ctx)
        } else {
            prev.end(path, root.fork(), ctx);
            (self.inner.instantiate(path, root, ctx),Invalidation::new())
        };

        (MemoizeWidget {
            memoize: self.memoize.clone(),
            inner,
            _p: PhantomData,
        },vali)
    }
}

pub struct MemoizeWidget<M,T,E> where M: Clone + PartialEq, T: Widget<E>, E: Env {
    memoize: M,
    inner: T,
    _p: PhantomData<E>,
}

impl<M,T,E> Widget<E> for MemoizeWidget<M,T,E> where M: Clone + PartialEq, T: Widget<E>, E: Env {
    type Cache = T::Cache;

    #[inline]
    fn id(&self) -> WidgetID {
        self.inner.id()
    }
    #[inline]
    fn render(
        &mut self,
        path: &mut NewPathStack,
        stack: StdRenderProps<'_,dyn QueronDyn<E>+'_,E,()>,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.inner.render(path, stack, renderer, force_render, cache, root, ctx)
    }
    #[inline]
    fn event_direct(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        self.inner.event_direct(path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn size(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> {
        self.inner.size(path, stack, root, ctx)
    }
    #[inline]
    fn _render(
        &mut self,
        path: &mut NewPathStack,
        stack: StdRenderProps<'_,dyn QueronDyn<E>+'_,E,()>,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.inner._render(path, stack, renderer, force_render, cache, root, ctx)
    }
    #[inline]
    fn _event_direct(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        self.inner._event_direct(path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn _size(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> {
        self.inner._size(path, stack, root, ctx)
    }
    #[inline]
    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        self.inner.update(path, route, root, ctx)
    }
    #[inline]
    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.inner.end(path, root, ctx)
    }
    #[inline]
    fn childs(&self) -> Range<isize> {
        self.inner.childs()
    }
    #[inline]
    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        self.inner.child_dyn(idx)
    }
    #[inline]
    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        self.inner.child_dyn_mut(idx)
    }
    #[inline]
    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, callback: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        self.inner.childs_dyn(range, callback)
    }
    #[inline]
    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        self.inner.childs_dyn_mut(range, callback)
    }
    #[inline]
    fn resolve_child_dyn<'a,'b>(&'a self, path: PathSliceRef<'b>) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        self.inner.resolve_child_dyn(path)
    }
    #[inline]
    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: PathSliceRef<'b>) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        self.inner.resolve_child_dyn_mut(path)
    }
    #[inline]
    fn collect_childs_dyn_range(&self, range: Range<isize>) -> Vec<WidgetChildDynResult<'_,E>> {
        self.inner.collect_childs_dyn_range(range)
    }
    #[inline]
    fn collect_childs_dyn_range_mut(&mut self, range: Range<isize>) -> Vec<WidgetChildDynResultMut<'_,E>> {
        self.inner.collect_childs_dyn_range_mut(range)
    }
    #[inline]
    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.inner.send_mutation(path, resolve, args, root, ctx)
    }
    #[inline]
    fn resolve<'a>(
        &'a self,
        sub_path: PathSliceRef,
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<WidgetResolveDynResult<'a,E>,E::Error> {
        self.inner.resolve(sub_path, root, ctx)
    }
    #[inline]
    fn focusable(&self) -> bool {
        self.inner.focusable()
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        self.inner._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        self.inner._tabulate_by_tab()
    }
    #[inline]
    fn _tabulate_next_child(&self, path: &mut NewPathStack, stack: &(dyn QueronDyn<E>+'_), origin: tabulate::TabulateNextChildOrigin, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> tabulate::TabulateNextChildResponse {
        self.inner._tabulate_next_child(path, stack, origin, dir, root, ctx)
    }
    #[inline]
    fn _call_tabulate_on_child_idx(&self, idx: isize, path: &mut NewPathStack, stack: &(dyn QueronDyn<E>+'_), op: tabulate::TabulateOrigin, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse,E::Error> {
        self.inner._call_tabulate_on_child_idx(idx, path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn _tabulate(&self, path: &mut NewPathStack, stack: &(dyn QueronDyn<E>+'_), op: tabulate::TabulateOrigin, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse,E::Error> {
        self.inner._tabulate(path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn invalidate_recursive(&mut self, vali: Invalidation) {
        self.inner.invalidate_recursive(vali)
    }
    #[inline]
    fn inner<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        //TODO
        self.inner.inner()
    }
    #[inline]
    fn innest<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner.innest()
    }
    #[inline]
    fn inner_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner.inner_mut()
    }
    #[inline]
    fn innest_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
        self.inner.innest_mut()
    }
    #[inline]
    fn respond_downcast<'a>(&'a self, mut responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast() {
            *v = Some(self);
        } else {
            self.inner.respond_downcast(responder)
        }
    }
    #[inline]
    fn respond_downcast_mut<'a>(&'a mut self, mut responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast() {
            *v = Some(self);
        } else {
            self.inner.respond_downcast_mut(responder)
        }
    }
    #[inline]
    fn respond_downcast_recursive<'a>(&'a self, mut responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast() {
            *v = Some(self);
        } else {
            self.inner.respond_downcast_recursive(responder)
        }
    }
    #[inline]
    fn respond_downcast_recursive_mut<'a>(&'a mut self, mut responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast() {
            *v = Some(self);
        } else {
            self.inner.respond_downcast_recursive_mut(responder)
        }
    }
    #[inline]
    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        self.inner.debug_type_name(dest)
    }
    #[inline]
    fn debugged_type_name(&self) -> Vec<&'static str> {
        self.inner.debugged_type_name()
    }
    #[inline]
    fn pass(self) -> Self where Self: Sized {
        self
    }
    #[inline]
    fn respond_query<'a>(&'a self, responder: WQueryResponder<'_,'a,E>) {
        self.inner.respond_query(responder);
    }
    #[inline]
    fn respond_query_mut<'a>(&'a mut self, mut responder: WQueryResponder<'_,'a,E>) {
        if let Some(h) = responder.try_downcast::<WQueryMemoizeRestore>() {
            *h = Some(self);
        } else {
            self.inner.respond_query_mut(responder);
        }
    }
    #[inline]
    fn respond_query_generic<'a,Q,G>(&'a self, responder: WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: WQueryGeneric<E> + ?Sized, G: ?Sized {
        self.inner.respond_query_generic::<Q,G>(responder)
    }
    #[inline]
    fn erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self.inner.erase()
    }
    #[inline]
    fn erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self.inner.erase_mut()
    }
    #[inline]
    fn erase2<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        self.inner.erase2()
    }
    #[inline]
    fn erase2_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        self.inner.erase2_mut()
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        self
    }
    #[inline]
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        Box::new(self)
    }
    #[inline]
    fn gen_diag_error_resolve_fail(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error {
        self.inner.gen_diag_error_resolve_fail(sub_path, op, root, ctx)
    }
    #[inline]
    fn guion_resolve_error_child_info(&self, child_idx: isize) -> crate::util::error::GuionResolveErrorChildInfo<E> {
        self.inner.guion_resolve_error_child_info(child_idx)
    }
}

pub struct WQueryMemoizeRestore;

impl<E> WQuery<E> for WQueryMemoizeRestore where E: Env {
    type Result<'a> = &'a mut dyn WidgetDyn<E>;
}
