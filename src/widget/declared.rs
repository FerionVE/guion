use std::any::Any;
use std::marker::PhantomData;
use std::ops::Range;

use crate::aliases::ESize;
use crate::env::Env;
use crate::event_new;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::queron::Queron;
use crate::root::RootRef;
use crate::traitcast::{WQuery, WQueryResponder, WQueryGeneric, WQueryResponderGeneric, DowncastResponder, DowncastMutResponder};
use crate::util::tabulate;
use crate::widget_decl::route::UpdateRoute;
use crate::widget_decl::{WidgetDeclCallback, WidgetDeclCallbackMode};

use super::dyn_tunnel::WidgetDyn;
use super::{Widget, WidgetChildDynResultMut, WidgetChildDynResult, WidgetChildResolveDynResultMut, WidgetChildResolveDynResult};

pub struct WidgetDeclarative<I,W,F,E>
where
    F: FnMut(
        E::RootRef<'_>,
        &I,
        //Arc<dyn for<'a> Fn(E::RootMut<'a>,&'a ()) -> &'a mut I>,
        WidgetDeclCallback<'_,W,E>,
        &mut E::Context<'_>,
    ),
    W: Widget<E> + 'static,
    I: 'static,
    E: Env,
{
    pub(crate) data: I,
    pub(crate) decl: F,
    pub(crate) inner: W,
    _p: PhantomData<E>,
}

impl<I,W,F,E> WidgetDeclarative<I,W,F,E>
where
    F: FnMut(
        E::RootRef<'_>,
        &I,
        //Arc<dyn for<'a> Fn(E::RootMut<'a>,&'a ()) -> &'a mut I>,
        WidgetDeclCallback<'_,W,E>,
        &mut E::Context<'_>,
    ),
    W: Widget<E> + 'static,
    I: 'static,
    E: Env,
{
    //TODO widget can't yet be created right here as the mutor to &mut I isn't known yet here
    pub fn new<Ph>(mut decl: F, data: I, root: E::RootRef<'_>, path: &Ph, ctx: &mut E::Context<'_>) -> Self where Ph: PathStack<E> + ?Sized {
        let mut dest = None;
        
        let op = WidgetDeclCallback {
            root: root.fork(),
            path: path._erase(),
            route: UpdateRoute::none(),
            command: WidgetDeclCallbackMode::Instantiate(&mut dest),
        };

        (decl)(root, &data, op, ctx);

        Self {
            data,
            decl,
            inner: dest.unwrap(),
            _p: PhantomData,
        }
    }
}

impl<XI,XW,XF,E> Widget<E> for WidgetDeclarative<XI,XW,XF,E>
where
    XF: FnMut(
        E::RootRef<'_>,
        &XI,
        //Arc<dyn for<'a> Fn(E::RootMut<'a>,&'a ()) -> &'a mut XI>,
        WidgetDeclCallback<'_,XW,E>,
        &mut E::Context<'_>,
    ),
    XW: Widget<E> + 'static,
    XI: 'static,
    E: Env,
{
    type Cache = XW::Cache;

    #[inline]
    fn id(&self) -> super::id::WidgetID {
        self.inner.id()
    }
    #[inline]
    fn render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut crate::aliases::ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner.render(path, stack, renderer, force_render, cache, root, ctx)
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
        self.inner.event_direct(path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner.size(path, stack, root, ctx)
    }
    #[inline]
    fn _render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut crate::aliases::ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner._render(path, stack, renderer, force_render, cache, root, ctx)
    }
    #[inline]
    fn _event_direct<P,Ph,Evt>(
        &mut self,
        path: &Ph,
        stack: &P,
        event: &Evt, // what if e.g. bounds change, if it's validated by parents then it's not signaled here
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        self.inner._event_direct(path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn _size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner._size(path, stack, root, ctx)
    }
    #[inline]
    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        let mut vali = Invalidation::new();

        let op = WidgetDeclCallback {
            root: root.fork(),
            path: path._erase(),
            route: route.clone(), //TODO
            command: WidgetDeclCallbackMode::Update(&mut self.inner, &mut vali),
        };

        (self.decl)(root.fork(), &self.data, op, ctx);

        self.inner.update(path, route, root, ctx) | vali
    }
    #[inline]
    fn end<Ph>(
        &mut self,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
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
    fn resolve_child_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        self.inner.resolve_child_dyn(path)
    }
    #[inline]
    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
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
    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        self.inner.send_mutation(path, resolve, args, root, ctx)
    }
    #[inline]
    fn resolve<'a>(
        &'a self,
        sub_path: &(dyn PathResolvusDyn<E>),
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<super::WidgetResolveDynResult<'a,E>,E::Error> {
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
    fn _tabulate_next_child<P,Ph>(&self, path: &Ph, stack: &P, origin: tabulate::TabulateNextChildOrigin, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> tabulate::TabulateNextChildResponse where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner._tabulate_next_child(path, stack, origin, dir, root, ctx)
    }
    #[inline]
    fn _call_tabulate_on_child_idx<P,Ph>(&self, idx: isize, path: &Ph, stack: &P, op: tabulate::TabulateOrigin<E>, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner._call_tabulate_on_child_idx(idx, path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn _tabulate<P,Ph>(&self, path: &Ph, stack: &P, op: tabulate::TabulateOrigin<E>, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.inner._tabulate(path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn inner<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        self.inner.inner()
    }
    #[inline]
    fn innest<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
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
    fn invalidate_recursive(&mut self, vali: Invalidation) {
        self.inner.invalidate_recursive(vali)
    }
    #[inline]
    fn respond_downcast<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) {
        self.inner.respond_downcast(responder)
    }
    #[inline]
    fn respond_downcast_mut<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) {
        self.inner.respond_downcast_mut(responder)
    }
    #[inline]
    fn respond_downcast_recursive<'a>(&'a self, responder: DowncastResponder<'_,'a,E>) {
        self.inner.respond_downcast_recursive(responder)
    }
    #[inline]
    fn respond_downcast_recursive_mut<'a>(&'a mut self, responder: DowncastMutResponder<'_,'a,E>) {
        self.inner.respond_downcast_recursive_mut(responder)
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
    fn respond_query<'a>(&'a self, mut responder: WQueryResponder<'_,'a,E>) {
        if !responder.try_respond::<WQueryDeclData<XI>>(|| &self.data ) {
            self.inner.respond_query(responder);
        }
    }
    #[inline]
    fn respond_query_mut<'a>(&'a mut self, mut responder: WQueryResponder<'_,'a,E>) {
        if !responder.try_respond::<WQueryDeclDataMut<XI>>(|| &mut self.data ) {
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

pub struct WQueryDeclData<I>(pub PhantomData<I>) where I: 'static;

impl<E,I> WQuery<E> for WQueryDeclData<I> where E: Env {
    type Result<'a> = &'a I;
}

pub struct WQueryDeclDataMut<I>(pub PhantomData<I>) where I: 'static;

impl<E,I> WQuery<E> for WQueryDeclDataMut<I> where E: Env {
    type Result<'a> = &'a mut I;
}
