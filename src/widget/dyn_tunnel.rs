use std::marker::PhantomData;

use crate::aliases::{EStyle, ERenderer};
use crate::env::Env;
use crate::event::compound::EventCompound;
use crate::queron::dyn_tunnel::QueronDyn;

use super::*;

pub trait WidgetDyn<E> where E: Env + 'static {
    fn id_dyn(&self) -> E::WidgetID;

    fn _render_dyn(&self, stack: &(dyn QueronDyn<E>+'_), r: &mut ERenderer<'_,E>);

    fn _event_direct_dyn(&self, stack: &(dyn QueronDyn<E>+'_), e: &EventCompound<E>) -> EventResp;

    fn _size_dyn(&self, stack: &(dyn QueronDyn<E>+'_), e: &EStyle<E>) -> ESize<E>;

    fn childs_dyn(&self) -> usize;

    #[deprecated]
    fn with_child_dyn<'s>(
        &'s self,
        i: usize,
        callback: Box<dyn for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut <E as Env>::Context<'cc>) -> ProtectedReturn>,
        root: <E as Env>::RootRef<'s>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> ProtectedReturn;

    #[deprecated]
    fn childs_ref_dyn<'s>(
        &'s self,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut E::Context<'cc>),
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    );
    
    #[deprecated]
    fn child_paths_dyn(&self, own_path: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<E::WidgetPath>;

    #[deprecated]
    fn with_resolve_dyn<'s>(
        &'s self,
        i: E::WidgetPath,
        callback: Box<dyn for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut <E as Env>::Context<'cc>) -> ProtectedReturn>,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;

    fn resolve_child_dyn(&self, sub_path: &E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(usize,E::WidgetPath),E::Error>;

    fn trace_bounds_dyn(&self, stack: &(dyn QueronDyn<E>+'_), i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error>;

    fn child_bounds_dyn(&self, stack: &(dyn QueronDyn<E>+'_), b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()>;
    
    #[deprecated]
    fn in_parent_path_dyn(&self, parent: E::WidgetPath) -> E::WidgetPath;

    #[deprecated]
    fn resolved_by_path_dyn(&self, sub_path: &E::WidgetPath) -> Option<ResolvesThruResult<E>>;

    fn focusable_dyn(&self) -> bool;

    fn _focus_on_mouse_down_dyn(&self) -> bool;

    fn _tabulate_by_tab_dyn(&self) -> bool;

    #[deprecated="Not supposted to be exposed"]
    fn _tabulate_next_child_dyn(&self, stack: &(dyn QueronDyn<E>+'_), origin: TabulateNextChildOrigin, dir: TabulateDirection) -> TabulateNextChildResponse;

    fn _tabulate_dyn(&self, stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,E::Error>;
    
    fn inner_dyn<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's;

    fn innest_dyn(&self) -> Option<&dyn WidgetDyn<E>>;

    fn as_any_dyn(&self) -> &dyn std::any::Any where Self: 'static;

    fn debug_type_name_dyn(&self, dest: &mut Vec<&'static str>);
    fn debugged_type_name_dyn(&self) -> Vec<&'static str>;

    unsafe fn _as_trait_ref_dyn(&self, t: TypeId) -> Option<TraitObject>;

    fn erase_dyn<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's;

    fn box_ref_dyn<'s>(&'s self) -> Box<dyn WidgetDyn<E>+'s> where Self: 's;

    fn box_box_dyn<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w;

    fn gen_diag_error_resolve_fail_dyn(&self, sub_path: &E::WidgetPath, op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error;

    fn guion_resolve_error_child_info_dyn(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E>;
}

impl<T,E> WidgetDyn<E> for T where T: Widget<E> + ?Sized, E: Env {
    #[inline]
    fn id_dyn(&self) -> <E as Env>::WidgetID {
        self.id()
    }
    #[inline]
    fn _render_dyn(&self, stack: &(dyn QueronDyn<E>+'_), r: &mut ERenderer<'_,E>) {
        self._render(stack, r)
    }
    #[inline]
    fn _event_direct_dyn(&self, stack: &(dyn QueronDyn<E>+'_), e: &EventCompound<E>) -> EventResp {
        self._event_direct(stack, e)
    }
    #[inline]
    fn _size_dyn(&self, stack: &(dyn QueronDyn<E>+'_), e: &EStyle<E>) -> ESize<E> {
        self._size(stack, e)
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
        callback: Box<dyn for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut <E as Env>::Context<'cc>) -> ProtectedReturn>,
        root: <E as Env>::RootRef<'s>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> ProtectedReturn {
        self.with_child(i, callback, root, ctx)
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref_dyn<'s>(
        &'s self,
        callback: &mut dyn for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut <E as Env>::Context<'cc>),
        root: <E as Env>::RootRef<'s>,
        ctx: &mut <E as Env>::Context<'_>
    ) {
        self.childs_ref(callback, root, ctx)
    }
    #[allow(deprecated)]
    #[inline]
    fn child_paths_dyn(&self, own_path: <E as Env>::WidgetPath, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Vec<<E as Env>::WidgetPath> {
        self.child_paths(own_path, root, ctx)
    }
    #[allow(deprecated)]
    #[inline]
    fn with_resolve_dyn<'s>(
        &'s self,
        i: <E as Env>::WidgetPath,
        callback: Box<dyn for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut <E as Env>::Context<'cc>) -> ProtectedReturn>,
        root: <E as Env>::RootRef<'s>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> ProtectedReturn {
        self.with_resolve(i, callback, root, ctx)
    }
    #[inline]
    fn resolve_child_dyn(&self, sub_path: &<E as Env>::WidgetPath, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Result<(usize,<E as Env>::WidgetPath),<E as Env>::Error> {
        self.resolve_child(sub_path, root, ctx)
    }
    #[inline]
    fn trace_bounds_dyn(&self, stack: &(dyn QueronDyn<E>+'_), i: <E as Env>::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,<E as Env>::Error> {
        self.trace_bounds(stack, i, b, e, force)
    }
    #[inline]
    fn child_bounds_dyn(&self, stack: &(dyn QueronDyn<E>+'_), b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        self.child_bounds(stack, b, e, force)
    }
    #[allow(deprecated)]
    #[inline]
    fn in_parent_path_dyn(&self, parent: <E as Env>::WidgetPath) -> <E as Env>::WidgetPath {
        self.in_parent_path(parent)
    }
    #[allow(deprecated)]
    #[inline]
    fn resolved_by_path_dyn(&self, sub_path: &<E as Env>::WidgetPath) -> Option<ResolvesThruResult<E>> {
        self.resolved_by_path(sub_path)
    }
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
    fn _tabulate_next_child_dyn(&self, stack: &(dyn QueronDyn<E>+'_), origin: TabulateNextChildOrigin, dir: TabulateDirection) -> TabulateNextChildResponse {
        self._tabulate_next_child(stack, origin, dir)
    }
    #[inline]
    fn _tabulate_dyn(&self, stack: &(dyn QueronDyn<E>+'_), op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,<E as Env>::Error> {
        self._tabulate_dyn(stack, op, dir)
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
    fn gen_diag_error_resolve_fail_dyn(&self, sub_path: &<E as Env>::WidgetPath, op: &'static str, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> <E as Env>::Error {
        self.gen_diag_error_resolve_fail(sub_path, op, root, ctx)
    }
    #[inline]
    fn guion_resolve_error_child_info_dyn(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        self.guion_resolve_error_child_info(child_idx)
    }
}

impl<E> Widget<E> for dyn WidgetDyn<E> + '_ where E: Env {
    #[inline]
    fn id(&self) -> <E as Env>::WidgetID {
        self.id_dyn()
    }
    #[inline]
    fn _render<P>(&self, stack: &P, r: &mut ERenderer<'_,E>) where P: Queron<E> + ?Sized {
        self._render_dyn(stack.erase(), r)
    }
    #[inline]
    fn _event_direct<P>(&self, stack: &P, e: &EventCompound<E>) -> EventResp where P: Queron<E> + ?Sized {
        self._event_direct_dyn(stack.erase(), e)
    }
    #[inline]
    fn _size<P>(&self, stack: &P, e: &EStyle<E>) -> ESize<E> where P: Queron<E> + ?Sized {
        self._size_dyn(stack.erase(), e)
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
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        let mut callback_return: Option<R> = None;
        self.with_child_dyn(
            i,
            Box::new(#[inline] |w,ctx| {
                let r = (callback)(w,ctx);
                callback_return = Some(r);
                ProtectedReturn(PhantomData)
            }),
            root, ctx,
        );
        callback_return.unwrap()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref<'s,F>(
        &'s self,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    )
    where
        F: for<'w,'ww,'c,'cc> FnMut(usize,&'w (dyn WidgetDyn<E>+'ww),&'c mut E::Context<'cc>)
    {
        self.childs_ref_dyn(&mut callback, root, ctx)
    }
    #[allow(deprecated)]
    #[inline]
    fn child_paths(&self, own_path: <E as Env>::WidgetPath, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Vec<<E as Env>::WidgetPath> {
        self.child_paths_dyn(own_path, root, ctx)
    }
    #[allow(deprecated)]
    #[inline]
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
        let mut callback_return: Option<R> = None;
        self.with_resolve(
            i,
            Box::new(#[inline] |w,ctx| {
                let r = (callback)(w,ctx);
                callback_return = Some(r);
                ProtectedReturn(PhantomData)
            }),
            root, ctx,
        );
        callback_return.unwrap()
    }
    #[inline]
    fn resolve_child(&self, sub_path: &<E as Env>::WidgetPath, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Result<(usize,<E as Env>::WidgetPath),<E as Env>::Error> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        self.resolve_child_dyn(sub_path, root, ctx)
    }
    #[inline]
    fn trace_bounds<P>(&self, stack: &P, i: <E as Env>::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,<E as Env>::Error> where P: Queron<E> + ?Sized {
        self.trace_bounds_dyn(stack.erase(), i, b, e, force)
    }
    #[inline]
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        self.child_bounds_dyn(stack.erase(), b, e, force)
    }
    #[allow(deprecated)]
    #[inline]
    fn in_parent_path(&self, parent: <E as Env>::WidgetPath) -> <E as Env>::WidgetPath {
        self.in_parent_path_dyn(parent)
    }
    #[allow(deprecated)]
    #[inline]
    fn resolved_by_path(&self, sub_path: &<E as Env>::WidgetPath) -> Option<ResolvesThruResult<E>> {
        self.resolved_by_path_dyn(sub_path)
    }
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
    fn _tabulate_next_child<P>(&self, stack: &P, origin: TabulateNextChildOrigin, dir: TabulateDirection) -> TabulateNextChildResponse where P: Queron<E> + ?Sized {
        self._tabulate_next_child_dyn(stack.erase(), origin, dir)
    }
    #[inline]
    fn _tabulate<P>(&self, stack: &P, op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,<E as Env>::Error> where P: Queron<E> + ?Sized {
        self._tabulate_dyn(stack.erase(), op, dir)
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
        Box::new(self).box_box_dyn() //TODO remove all the boxed bs
    }

    //TODO cold
    fn gen_diag_error_resolve_fail(&self, sub_path: &<E as Env>::WidgetPath, op: &'static str, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> <E as Env>::Error {
        self.gen_diag_error_resolve_fail_dyn(sub_path, op, root, ctx)
    }

    //TODO cold
    fn guion_resolve_error_child_info(&self, child_idx: usize) -> GuionResolveErrorChildInfo<E> {
        self.guion_resolve_error_child_info_dyn(child_idx)
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
        Box::new(self)
    }

    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self.as_any_dyn()
    }
}
