use std::any::{TypeId, Any};
use std::convert::Infallible;
use std::ops::Range;

use crate::invalidation::Invalidation;
use crate::traitcast::{WQueryResponder, WQueryGeneric, WQueryResponderGeneric, DowncastMutResponder, DowncastResponder};
use crate::util::error::GuionResolveErrorChildInfo;
use crate::util::tabulate;
use crate::widget_decl::route::UpdateRoute;
use crate::{EventResp, event_new};
use crate::aliases::{ESize, ERenderer};
use crate::env::Env;
use crate::newpath::{PathStack, PathFragment, PathResolvusDyn};
use crate::queron::Queron;

use super::{Widget, WidgetChildDynResult, WidgetResolveDynResult, WidgetChildDynResultMut, WidgetResolveDynResultMut, WidgetChildResolveDynResult, WidgetChildResolveDynResultMut};
use super::dyn_tunnel::WidgetDyn;

impl<E> Widget<E> for Infallible where E: Env {
    type Cache = ();

    fn id(&self) -> super::id::WidgetID {
        match *self {}
    }

    fn _render<P,Ph>(
        &mut self,
        _: &Ph,
        _: &P,
        _: &mut ERenderer<'_,E>,
        _: bool,
        _: &mut Self::Cache,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        match *self {}
    }

    fn _event_direct<P,Ph,Evt>(
        &mut self,
        _: &Ph,
        _: &P,
        _: &Evt,
        _: Option<&(dyn PathResolvusDyn<E>+'_)>,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        match *self {}
    }

    fn _size<P,Ph>(
        &mut self,
        _: &Ph,
        _: &P,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        match *self {}
    }

    fn update<Ph>(
        &mut self,
        _: &Ph,
        _: UpdateRoute<'_,E>,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        match *self {}
    }

    fn end<Ph>(
        &mut self,
        _: &Ph,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        match *self {}
    }

    fn childs(&self) -> Range<isize> {
        match *self {}
    }

    fn child_dyn(&self, _: isize) -> Option<WidgetChildDynResult<'_,E>> {
        match *self {}
    }

    fn child_dyn_mut(&mut self, _: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        match *self {}
    }

    fn childs_dyn<'a,F>(&'a self, _: Range<isize>, _: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        match *self {}
    }

    fn childs_dyn_mut<'a,F>(&'a mut self, _: Range<isize>, _: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        match *self {}
    }

    fn resolve_child_dyn<'a,'b>(&'a self, _: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        match *self {}
    }

    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, _: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        match *self {}
    }

    fn send_mutation<Ph>(
        &mut self,
        _: &Ph,
        _: &(dyn PathResolvusDyn<E>+'_),
        _: &dyn Any,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        match *self {}
    }

    fn focusable(&self) -> bool {
        match *self {}
    }

    fn _call_tabulate_on_child_idx<P,Ph>(&self, _: isize, _: &Ph, _: &P, _: tabulate::TabulateOrigin<E>, _: tabulate::TabulateDirection, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        match *self {}
    }

    fn respond_query<'a>(&'a self, _: WQueryResponder<'_,'a,E>) {
        match *self {}
    }

    fn respond_query_mut<'a>(&'a mut self, _: WQueryResponder<'_,'a,E>) {
        match *self {}
    }

    fn invalidate_recursive(&mut self, _: Invalidation) {
        match *self {}
    }
}

impl<TT,E> Widget<E> for Box<TT> where TT: Widget<E> + ?Sized, E: Env {
    type Cache = TT::Cache;

    #[inline]
    fn id(&self) -> super::id::WidgetID {
        (**self).id()
    }
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
        (**self).render(path, stack, renderer, force_render, cache, root, ctx)
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
        (**self).event_direct(path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        (**self).size(path, stack, root, ctx)
    }
    #[inline]
    fn _render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        (**self)._render(path, stack, renderer, force_render, cache, root, ctx)
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
        (**self)._event_direct(path, stack, event, route_to_widget, root, ctx)
    }
    #[inline]
    fn _size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        (**self)._size(path, stack, root, ctx)
    }
    #[inline]
    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        (**self).update(path, route, root, ctx)
    }
    #[inline]
    fn end<Ph>(
        &mut self,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        (**self).end(path, root, ctx)
    }
    #[inline]
    fn childs(&self) -> Range<isize> {
        (**self).childs()
    }
    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        (**self).child_dyn(idx)
    }
    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        (**self).child_dyn_mut(idx)
    }
    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, callback: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        (**self).childs_dyn(range, callback)
    }
    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        (**self).childs_dyn_mut(range, callback)
    }
    fn resolve_child_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        (**self).resolve_child_dyn(path)
    }
    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        (**self).resolve_child_dyn_mut(path)
    }
    fn collect_childs_dyn_range(&self, range: Range<isize>) -> Vec<WidgetChildDynResult<'_,E>> {
        (**self).collect_childs_dyn_range(range)
    }
    fn collect_childs_dyn_range_mut(&mut self, range: Range<isize>) -> Vec<WidgetChildDynResultMut<'_,E>> {
        (**self).collect_childs_dyn_range_mut(range)
    }
    fn send_mutation<Ph>(
        &mut self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        (**self).send_mutation(path, resolve, args, root, ctx)
    }
    fn resolve<'a>(
        &'a self,
        sub_path: &(dyn PathResolvusDyn<E>),
        root: E::RootRef<'a>,
        ctx: &mut E::Context<'_>
    ) -> Result<WidgetResolveDynResult<'a,E>,E::Error> {
        (**self).resolve(sub_path, root, ctx)
    }
    #[inline]
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        (**self)._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        (**self)._tabulate_by_tab()
    }
    #[inline]
    fn _tabulate_next_child<P,Ph>(&self, path: &Ph, stack: &P, origin: tabulate::TabulateNextChildOrigin, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> tabulate::TabulateNextChildResponse where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        (**self)._tabulate_next_child(path, stack, origin, dir, root, ctx)
    }
    #[inline]
    fn _call_tabulate_on_child_idx<P,Ph>(&self, idx: isize, path: &Ph, stack: &P, op: tabulate::TabulateOrigin<E>, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        (**self)._call_tabulate_on_child_idx(idx, path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn _tabulate<P,Ph>(&self, path: &Ph, stack: &P, op: tabulate::TabulateOrigin<E>, dir: tabulate::TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<tabulate::TabulateResponse<E>,E::Error> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        (**self)._tabulate(path, stack, op, dir, root, ctx)
    }
    #[inline]
    fn inner<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's {
        (**self).inner()
    }
    #[inline]
    fn innest<'s>(&self) -> Option<&(dyn WidgetDyn<E>+'s)> where Self: 's { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
        (**self).innest()
    }
    #[inline]
    fn inner_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's {
        (**self).inner_mut()
    }
    #[inline]
    fn innest_mut<'s>(&mut self) -> Option<&mut (dyn WidgetDyn<E>+'s)> where Self: 's { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn WidgetDyn<E>+'w)> where Self: 'w
        (**self).innest_mut()
    }
    #[inline]
    fn invalidate_recursive(&mut self, vali: Invalidation) {
        (**self).invalidate_recursive(vali)
    }
    #[inline]
    fn respond_downcast<'a>(&'a self, mut responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        } else {
            (**self).respond_downcast(responder)
        }
    }
    #[inline]
    fn respond_downcast_mut<'a>(&'a mut self, mut responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        } else {
            (**self).respond_downcast_mut(responder)
        }
    }
    #[inline]
    fn respond_downcast_recursive<'a>(&'a self, mut responder: DowncastResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        } else {
            (**self).respond_downcast_recursive(responder)
        }
    }
    #[inline]
    fn respond_downcast_recursive_mut<'a>(&'a mut self, mut responder: DowncastMutResponder<'_,'a,E>) where Self: 'static {
        if let Some(v) = responder.try_downcast::<Self>() {
            *v = Some(self);
        } else {
            (**self).respond_downcast_recursive_mut(responder)
        }
    }
    // #[inline]
    // fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
    //     dest.push("Box<...>");
    //     (**self).debug_type_name(dest)
    // }
    #[inline]
    fn pass(self) -> Self where Self: Sized {
        self
    }
    #[inline]
    fn query<'a,T>(&'a self) -> Option<T::Result<'a>> where T: crate::traitcast::WQuery<E> + ?Sized, Self: 'a {
        (**self).query::<T>()
    }
    #[inline]
    fn query_generic<'a,T,G>(&'a self) -> Option<T::Result<'a,G>> where T: WQueryGeneric<E> + ?Sized, G: ?Sized, Self: 'a {
        (**self).query_generic::<T,G>()
    }
    #[inline]
    fn respond_query<'a>(&'a self, responder: WQueryResponder<'_,'a,E>) {
        (**self).respond_query(responder)
    }
    #[inline]
    fn respond_query_mut<'a>(&'a mut self, responder: WQueryResponder<'_,'a,E>) {
        (**self).respond_query_mut(responder)
    }
    #[inline]
    fn respond_query_generic<'a,Q,G>(&'a self, responder: WQueryResponderGeneric<'_,'a,Q,G,E>) where Q: WQueryGeneric<E> + ?Sized, G: ?Sized {
        (**self).respond_query_generic::<Q,G>(responder)
    }

    #[inline]
    fn erase<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        (**self).erase()
    }

    #[inline]
    fn erase_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        (**self).erase_mut()
    }

    #[inline]
    fn erase2<'s>(&self) -> &(dyn WidgetDyn<E>+'s) where Self: 's {
        (**self).erase2()
    }

    #[inline]
    fn erase2_mut<'s>(&mut self) -> &mut (dyn WidgetDyn<E>+'s) where Self: 's {
        (**self).erase2_mut()
    }

    #[inline]
    fn box_box<'w>(self: Box<Self>) -> Box<dyn WidgetDyn<E>+'w> where Self: 'w {
        <TT as Widget<E>>::box_box(*self)
    }

    #[inline]
    fn boxed<'w>(self) -> Box<dyn WidgetDyn<E>+'w> where Self: Sized + 'w {
        <TT as Widget<E>>::box_box(self)
    }

    #[inline]
    fn gen_diag_error_resolve_fail(&self, sub_path: &(dyn PathResolvusDyn<E>+'_), op: &'static str, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> E::Error {
        (**self).gen_diag_error_resolve_fail(sub_path, op, root, ctx)
    }

    #[inline]
    fn guion_resolve_error_child_info(&self, child_idx: isize) -> GuionResolveErrorChildInfo<E> {
        (**self).guion_resolve_error_child_info(child_idx)
    }
}

impl<'a,E> From<WidgetChildDynResult<'a,E>> for WidgetResolveDynResult<'a,E> {
    #[inline]
    fn from(v: WidgetChildDynResult<'a,E>) -> Self {
        Self {
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}
impl<'a,E> From<WidgetChildDynResultMut<'a,E>> for WidgetResolveDynResultMut<'a,E> {
    #[inline]
    fn from(v: WidgetChildDynResultMut<'a,E>) -> Self {
        Self {
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}

impl<'a,'b,E> From<WidgetChildResolveDynResult<'a,'b,E>> for WidgetResolveDynResult<'a,E> {
    #[inline]
    fn from(v: WidgetChildResolveDynResult<'a,'b,E>) -> Self {
        Self {
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}
impl<'a,'b,E> From<WidgetChildResolveDynResultMut<'a,'b,E>> for WidgetResolveDynResultMut<'a,E> {
    #[inline]
    fn from(v: WidgetChildResolveDynResultMut<'a,'b,E>) -> Self {
        Self {
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}

// impl<'a,CID,E> From<AsWidgetsDynResult<'a,CID,E>> for WidgetResolveDynResult<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResult<'a,CID,E>) -> Self {
//         Self {
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }
// impl<'a,CID,E> From<AsWidgetsDynResultMut<'a,CID,E>> for WidgetResolveDynResultMut<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResultMut<'a,CID,E>) -> Self {
//         Self {
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }

// impl<'a,'b,CID,E> From<AsWidgetsDynResolveResult<'a,'b,CID,E>> for WidgetResolveDynResult<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResolveResult<'a,'b,CID,E>) -> Self {
//         Self {
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }
// impl<'a,'b,CID,E> From<AsWidgetsDynResolveResultMut<'a,'b,CID,E>> for WidgetResolveDynResultMut<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResolveResultMut<'a,'b,CID,E>) -> Self {
//         Self {
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }

impl<'a,'b,E> From<WidgetChildResolveDynResult<'a,'b,E>> for WidgetChildDynResult<'a,E> {
    #[inline]
    fn from(v: WidgetChildResolveDynResult<'a,'b,E>) -> Self {
        Self {
            idx: v.idx,
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}
impl<'a,'b,E> From<WidgetChildResolveDynResultMut<'a,'b,E>> for WidgetChildDynResultMut<'a,E> {
    #[inline]
    fn from(v: WidgetChildResolveDynResultMut<'a,'b,E>) -> Self {
        Self {
            idx: v.idx,
            widget: v.widget,
            widget_id: v.widget_id,
        }
    }
}


// impl<'a,CID,E> From<AsWidgetsDynResult<'a,CID,E>> for WidgetChildDynResult<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResult<'a,CID,E>) -> Self {
//         Self {
//             idx: v.idx,
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }
// impl<'a,CID,E> From<AsWidgetsDynResultMut<'a,CID,E>> for WidgetChildDynResultMut<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResultMut<'a,CID,E>) -> Self {
//         Self {
//             idx: v.idx,
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }

// impl<'a,'b,CID,E> From<AsWidgetsDynResolveResult<'a,'b,CID,E>> for WidgetChildDynResult<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResolveResult<'a,'b,CID,E>) -> Self {
//         Self {
//             idx: v.idx,
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }
// impl<'a,'b,CID,E> From<AsWidgetsDynResolveResultMut<'a,'b,CID,E>> for WidgetChildDynResultMut<'a,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResolveResultMut<'a,'b,CID,E>) -> Self {
//         Self {
//             idx: v.idx,
//             widget: v.widget,
//             widget_id: v.widget_id,
//         }
//     }
// }

// impl<'a,'b,CID,E> From<AsWidgetsDynResolveResult<'a,'b,CID,E>> for WidgetChildResolveDynResult<'a,'b,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResolveResult<'a,'b,CID,E>) -> Self {
//         Self {
//             idx: v.idx,
//             widget: v.widget,
//             sub_path: v.resolvus,
//             widget_id: v.widget_id,
//         }
//     }
// }
// impl<'a,'b,CID,E> From<AsWidgetsDynResolveResultMut<'a,'b,CID,E>> for WidgetChildResolveDynResultMut<'a,'b,E> where CID: PathFragment<E> + Clone + 'static {
//     #[inline]
//     fn from(v: AsWidgetsDynResolveResultMut<'a,'b,CID,E>) -> Self {
//         Self {
//             idx: v.idx,
//             widget: v.widget,
//             sub_path: v.resolvus,
//             widget_id: v.widget_id,
//         }
//     }
// }
