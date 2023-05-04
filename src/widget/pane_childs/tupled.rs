use std::ops::{Range, Deref, DerefMut};

use crate::aliases::{ERenderer, ESize};
use crate::env::Env;
use crate::event_new;
use crate::invalidation::Invalidation;
use crate::layout::Orientation;
use crate::newpath::{PathFragment, PathResolvusDyn, PathResolvus, PathStack};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::render::StdRenderProps;
use crate::root::RootRef;
use crate::util::bounds::Dims;
use crate::util::tabulate::{TabulateOrigin, TabulateResponse, TabulateDirection};
use crate::widget::Widget;
use crate::widget::stack::QueriedCurrentBounds;

use super::{PaneChilds, PaneChildWidget, PaneChildsDyn, ChildWidgetDynResult, ChildWidgetDynResultMut};
use super::fixed_idx::DefaultHack;

#[repr(transparent)]
pub struct Tupled<T>(pub T) where T: ?Sized;

impl<T> Deref for Tupled<T> where T: ?Sized {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Tupled<T> where T: ?Sized {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E,I,T> PaneChilds<E> for Tupled<Vec<(I,PaneChildWidget<T,E>)>> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Caches = Vec<T::Cache>;

    fn render(
        &mut self,
        path: &mut NewPathStack,
        render_props: &StdRenderProps<'_,dyn QueronDyn<E>+'_,E,()>,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Caches,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        todo!()
    }

    fn event(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        bounds: &QueriedCurrentBounds,
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation {
        todo!()
    }

    fn constraints(
        &mut self,
        relayout: Option<Dims>,
        orientation: Orientation,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> ESize<E> {
        todo!()
    }

    fn _call_tabulate_on_child_idx(
        &self,
        idx: usize,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        op: TabulateOrigin,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse,E::Error> {
        todo!()
    }

    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        todo!()
    }

    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation {
        todo!()
    }

    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) {
        todo!()
    }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        todo!()
    }
}

impl<E,I,T,const N: usize> PaneChilds<E> for Tupled<[(I,PaneChildWidget<T,E>);N]> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Caches = Vec<T::Cache>;

    fn render(
        &mut self,
        path: &mut NewPathStack,
        render_props: &StdRenderProps<'_,dyn QueronDyn<E>+'_,E,()>,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Caches,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        todo!()
    }

    fn event(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        bounds: &QueriedCurrentBounds,
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation {
        todo!()
    }

    fn constraints(
        &mut self,
        relayout: Option<Dims>,
        orientation: Orientation,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> ESize<E> {
        todo!()
    }

    fn _call_tabulate_on_child_idx(
        &self,
        idx: usize,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        op: TabulateOrigin,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse,E::Error> {
        todo!()
    }

    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        todo!()
    }

    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation {
        todo!()
    }

    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) {
        todo!()
    }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        todo!()
    }
}

impl<E,I,T> PaneChildsDyn<E> for Tupled<Vec<(I,PaneChildWidget<T,E>)>> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type ChildID = I;

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|(id,inner)| ChildWidgetDynResult {
            widget: &inner.widget,
            widget_id: inner.widget.id(),
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<super::ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|(id,inner)| ChildWidgetDynResultMut {
            widget_id: inner.widget.id(),
            widget: &mut inner.widget,
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &v.widget,
                widget_id: v.widget.id(),
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    #[inline]
    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(super::ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: v.widget.id(),
                widget: &mut v.widget,
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget: inner,
    //                 widget_id: inner.id(),
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: inner.id(),
    //                 widget: inner,
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}

impl<E,I,T,const N: usize> PaneChildsDyn<E> for Tupled<[(I,PaneChildWidget<T,E>);N]> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type ChildID = I;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|(id,inner)| ChildWidgetDynResult {
            widget: &inner.widget,
            widget_id: inner.widget.id(),
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<super::ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|(id,inner)| ChildWidgetDynResultMut {
            widget_id: inner.widget.id(),
            widget: &mut inner.widget,
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &v.widget,
                widget_id: v.widget.id(),
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    #[inline]
    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(super::ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: v.widget.id(),
                widget: &mut v.widget,
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget: inner,
    //                 widget_id: inner.id(),
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: inner.id(),
    //                 widget: inner,
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}
