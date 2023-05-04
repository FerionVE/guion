use std::mem::MaybeUninit;
use std::ops::{Range, Deref, DerefMut};

use crate::aliases::{ESize, ERenderer};
use crate::env::Env;
use crate::event_new;
use crate::invalidation::Invalidation;
use crate::layout::Gonstraints;
use crate::layout::Orientation;
use crate::layout::calc::calc_bounds2;
use crate::layout::size::{StdGonstraintAxis, StdGonstraints};
use crate::newpath::{PathResolvusDyn, FixedIdx, PathResolvus, PathFragment, PathStack};
use crate::pathslice::{NewPathStack, PathSliceRef, PathSliceMatch};
use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::render::StdRenderProps;
use crate::root::RootRef;
use crate::util::bounds::Dims;
use crate::util::tabulate::{TabulateResponse, TabulateDirection, TabulateOrigin};
use crate::widget::stack::QueriedCurrentBounds;
use crate::widget::stack::WithCurrentBounds;
use crate::widget::{WidgetDyn, Widget};
use crate::widget_decl::pane_childs::fixed_idx::trans_array_enumerated_mut;

use super::{PaneChildWidget, PaneChilds, PaneChildsDyn, ChildWidgetDynResult, ChildWidgetDynResultMut};

mod impl_tuple;

#[repr(transparent)]
pub struct WidgetsFixedIdx<T>(pub T) where T: ?Sized;

impl<T> Deref for WidgetsFixedIdx<T> where T: ?Sized {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for WidgetsFixedIdx<T> where T: ?Sized {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E,T> PaneChilds<E> for WidgetsFixedIdx<Vec<PaneChildWidget<T,E>>> where T: Widget<E>, E: Env {
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
        for (idx,w) in self.0.iter_mut().enumerate() {
            if w.vali.render | force_render {
                w.widget.render(
                    &mut path.with(FixedIdx(idx as isize)),
                    render_props
                        .slice(w.relative_bounds.unwrap()),
                    renderer,
                    force_render, &mut cache[idx],
                    root.fork(), ctx
                );
                w.vali.render = false;
            }
        }
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
        if let Some(route_to_widget) = route_to_widget {
            if let PathSliceMatch::Match(idx, route_to_widget_inner) = route_to_widget.fetch().slice_forward::<FixedIdx>() {
                if let Some(w) = self.0.get_mut(idx.0 as usize) {
                    let stack = WithCurrentBounds {
                        inner: stack,
                        bounds: bounds.bounds.slice(w.relative_bounds.as_ref().unwrap()),
                        viewport: bounds.viewport.clone(),
                    };

                    let v = w.widget.event_direct(&mut path.with(*idx), &stack, event, Some(route_to_widget_inner), root, ctx);
                    w.invalidate(v);
                    return v;
                }
            }
            return Invalidation::valid();
        }

        let mut vali = Invalidation::valid();

        for (idx,w) in self.0.iter_mut().enumerate() {
            let stack = WithCurrentBounds {
                inner: &stack,
                bounds: bounds.bounds.slice(w.relative_bounds.as_ref().unwrap()),
                viewport: bounds.viewport.clone(),
            };

            let v = w.widget.event_direct(&mut path.with(FixedIdx(idx as isize)), &stack, event, None, root.fork(), ctx);
            w.invalidate(v);
            vali |= v
        }

        vali
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
        let mut constraint_sum = ESize::<E>::add_base(orientation);

        let mut parallel_axis = if relayout.is_some() {Vec::with_capacity(self.len())} else {vec![]};

        for (idx,w) in self.0.iter_mut().enumerate() {
            let constraint = w.constraints.get_or_insert_with(||
                w.widget.size(&mut path.with(FixedIdx(idx as isize)), stack, root.fork(), ctx)
            );

            constraint_sum.add(constraint, orientation);

            // if relayout.is_none() && w.relative_bounds.is_none() {
            //     todo!()
            // }

            if relayout.is_some() {
                parallel_axis.push(constraint.clone().into().par(orientation));
            }
        }

        if let Some(dims) = relayout {
            let new_bounds = calc_bounds2(
                &dims,
                &parallel_axis,
                orientation,
            );

            assert_eq!(new_bounds.len(),self.len());

            for (w,new_bound) in self.0.iter_mut().zip(new_bounds) {
                w.relative_bounds = Some(new_bound);
                w.vali.layout = false;
            }
        }

        constraint_sum
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
        if let Some(w) = self.0.get(idx) {
            return w.widget._tabulate(&mut path.with(FixedIdx(idx as isize)), stack, op, dir, root, ctx);
        }
        todo!()
    }

    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        for (idx,w) in self.0.iter_mut().enumerate() {
            w.widget.end(&mut path.with(FixedIdx(idx as isize)), root.fork(), ctx);
        }
    }

    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation {
        if let Some(r2) = route.resolving() {
            if let PathSliceMatch::Match(idx, _) = r2.fetch().slice_forward::<FixedIdx>() {
                let v = self.0[idx.0 as usize].widget.update(&mut path.with(*idx), route.for_child_1::<FixedIdx>(), root, ctx);
                self.0[idx.0 as usize].invalidate(v);
                return v;
            }
            return Invalidation::valid();
        }

        let mut vali = Invalidation::valid();

        for (idx,w) in self.0.iter_mut().enumerate() {
            let v = w.widget.update(&mut path.with(FixedIdx(idx as isize)), route.for_child_1::<FixedIdx>(), root.fork(), ctx);
            w.invalidate(v);
            vali |= v
        }

        vali
    }

    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) {
        if let PathSliceMatch::Match(idx, resolve_inner) = resolve.fetch().slice_forward::<FixedIdx>() {
            self.0[idx.0 as usize].widget.send_mutation(&mut path.with(*idx), resolve_inner, args, root, ctx);
        }
    }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        for (idx,w) in self.0.iter_mut().enumerate() {
            w.widget.invalidate_recursive(vali);
        }
    }
}

impl<E,T,const N: usize> PaneChilds<E> for WidgetsFixedIdx<[PaneChildWidget<T,E>;N]> where T: Widget<E>, E: Env {
    type Caches = DefaultHack<[T::Cache;N]>;

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
        for (idx,w) in self.0.iter_mut().enumerate() {
            if w.vali.render | force_render {
                w.widget.render(
                    &mut path.with(FixedIdx(idx as isize)),
                    render_props
                        .slice(w.relative_bounds.unwrap()),
                    renderer,
                    force_render, &mut cache.0[idx],
                    root.fork(), ctx
                );
                w.vali.render = false;
            }
        }
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
        if let Some(route_to_widget) = route_to_widget {
            if let PathSliceMatch::Match(idx, route_to_widget_inner) = route_to_widget.fetch().slice_forward::<FixedIdx>() {
                if let Some(w) = self.0.get_mut(idx.0 as usize) {
                    let stack = WithCurrentBounds {
                        inner: stack,
                        bounds: bounds.bounds.slice(w.relative_bounds.as_ref().unwrap()),
                        viewport: bounds.viewport.clone(),
                    };

                    let v = w.widget.event_direct(&mut path.with(*idx), &stack, event, Some(route_to_widget_inner), root, ctx);
                    w.invalidate(v);
                    return v;
                }
            }
            return Invalidation::valid();
        }

        let mut vali = Invalidation::valid();

        for (idx,w) in self.0.iter_mut().enumerate() {
            let stack = WithCurrentBounds {
                inner: &stack,
                bounds: bounds.bounds.slice(w.relative_bounds.as_ref().unwrap()),
                viewport: bounds.viewport.clone(),
            };

            let v = w.widget.event_direct(&mut path.with(FixedIdx(idx as isize)), &stack, event, None, root.fork(), ctx);
            w.invalidate(v);
            vali |= v
        }

        vali
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
        let mut constraint_sum = ESize::<E>::add_base(orientation);

        let parallel_axis = trans_array_enumerated_mut(&mut self.0, |idx,w|{
            let constraint = w.constraints.get_or_insert_with(||
                w.widget.size(&mut path.with(FixedIdx(idx as isize)), stack, root.fork(), ctx)
            );

            constraint_sum.add(constraint, orientation);

            // if relayout.is_none() && w.relative_bounds.is_none() {
            //     todo!()
            // }

            constraint.clone().into().par(orientation)
        });

        if let Some(dims) = relayout {
            let new_bounds = calc_bounds2(
                &dims,
                &parallel_axis,
                orientation,
            );

            assert_eq!(new_bounds.len(),self.len());

            for (w,new_bound) in self.0.iter_mut().zip(new_bounds) {
                w.relative_bounds = Some(new_bound);
                w.vali.layout = false;
            }
        }

        constraint_sum
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
        if let Some(w) = self.0.get(idx) {
            return w.widget._tabulate(&mut path.with(FixedIdx(idx as isize)), stack, op, dir, root, ctx);
        }
        todo!()
    }

    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        for (idx,w) in self.0.iter_mut().enumerate() {
            w.widget.end(&mut path.with(FixedIdx(idx as isize)), root.fork(), ctx);
        }
    }

    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation {
        if let Some(r2) = route.resolving() {
            if let PathSliceMatch::Match(idx, _) = r2.fetch().slice_forward::<FixedIdx>() {
                let v = self.0[idx.0 as usize].widget.update(&mut path.with(*idx), route.for_child_1::<FixedIdx>(), root, ctx);
                self.0[idx.0 as usize].invalidate(v);
                return v;
            }
            return Invalidation::valid();
        }

        let mut vali = Invalidation::valid();

        for (idx,w) in self.0.iter_mut().enumerate() {
            let v = w.widget.update(&mut path.with(FixedIdx(idx as isize)), route.for_child_1::<FixedIdx>(), root.fork(), ctx);
            w.invalidate(v);
            vali |= v
        }

        vali
    }

    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) {
        if let PathSliceMatch::Match(idx, resolve_inner) = resolve.fetch().slice_forward::<FixedIdx>() {
            self.0[idx.0 as usize].widget.send_mutation(&mut path.with(*idx), resolve_inner, args, root, ctx);
        }
    }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        for (idx,w) in self.0.iter_mut().enumerate() {
            w.widget.invalidate_recursive(vali);
        }
    }
}

impl<E,T> PaneChildsDyn<E> for WidgetsFixedIdx<Vec<PaneChildWidget<T,E>>> where T: Widget<E>, E: Env {
    type ChildID = FixedIdx;

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|widget| ChildWidgetDynResult {
            widget: &widget.widget,
            widget_id: widget.widget.id(),
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|widget| ChildWidgetDynResultMut {
            widget_id: widget.widget.id(),
            widget: &mut widget.widget,
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &widget.widget,
                widget_id: widget.widget.id(),
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: widget.widget.id(),
                widget: &mut widget.widget,
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get(idx as usize) {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget_id: widget.id(),
    //                 widget,
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get_mut(idx as usize) {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: widget.id(),
    //                 widget,
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}

impl<E,T,const N: usize> PaneChildsDyn<E> for WidgetsFixedIdx<[PaneChildWidget<T,E>;N]> where T: Widget<E>, E: Env {
    type ChildID = FixedIdx;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|widget| ChildWidgetDynResult {
            widget: &widget.widget,
            widget_id: widget.widget.id(),
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|widget| ChildWidgetDynResultMut {
            widget_id: widget.widget.id(),
            widget: &mut widget.widget,
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &widget.widget,
                widget_id: widget.widget.id(),
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: widget.widget.id(),
                widget: &mut widget.widget,
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get(idx as usize) {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget,
    //                 widget_id: widget.id(),
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: PathSliceRef<'b>) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get_mut(idx as usize) {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: widget.id(),
    //                 widget,
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}

pub struct DefaultHack<T>(T);

impl<T,const N: usize> Default for DefaultHack<[T;N]> where T: Default {
    #[inline]
    fn default() -> Self {
        unsafe { 
            let mut dest: MaybeUninit<[T;N]> = MaybeUninit::uninit();
            for entry in &mut *(dest.as_mut_ptr() as *mut [MaybeUninit<T>;N]) {
                entry.write(T::default());
            }
            Self(dest.assume_init())
        }
    }
}

impl<T,const N: usize> AsRef<[T]> for DefaultHack<[T;N]> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.0[..]
    }
}
impl<T,const N: usize> AsMut<[T]> for DefaultHack<[T;N]> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0[..]
    }
}

impl<T,const N: usize> Clone for DefaultHack<[T;N]> where T: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
