use std::marker::PhantomData;
use std::ops::Range;

use crate::event_new::Event;
use crate::invalidation::Invalidation;
use crate::pathslice::{NewPathStack, PathSliceRef, PathSliceMatch};
use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::widget::id::WidgetID;
use crate::widget::pane_childs::{PaneChilds, ChildWidgetDynResult};
use crate::{EventResp, event_new};
use crate::aliases::{ERenderer, ESize, EStyle};
use crate::env::Env;
use crate::layout::{Gonstraints, Orientation};
use crate::layout::calc::calc_bounds2;
use crate::layout::size::StdGonstraintAxis;
use crate::newpath::{PathStack, PathResolvusDyn, PathResolvus, PathFragment, FixedIdx};
use crate::queron::Queron;
use crate::render::widgets::RenderStdWidgets;
use crate::render::{with_inside_spacing_border, widget_size_inside_border_type, TestStyleBorderType, TestStyleColorType, StdRenderProps};
use crate::util::bounds::{Dims, Bounds};
use crate::util::tabulate::{TabulateResponse, TabulateDirection, TabulateOrigin};
use crate::widget::cache::{StdRenderCachors, WidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::{Widget, WidgetChildResolveDynResult, WidgetChildDynResultMut, WidgetChildDynResult, WidgetChildResolveDynResultMut};
use crate::widget::stack::{QueryCurrentBounds, WithCurrentBounds};

pub struct Pane<E,T> where
    E: Env,
{
    pub(super) id: WidgetID,
    pub(super) childs: T,
    pub(super) orientation: Orientation,
    pub(super) style: EStyle<E>,
    pub(super) layouted_dims: Option<Dims>,
    pub(super) layouted_constraints: Option<ESize<E>>,
    pub(super) rerender_childs: bool,
    pub(super) rerender_full: bool,
}

impl<E,T> Widget<E> for Pane<E,T> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    T: PaneChilds<E>,
{
    type Cache = PaneCache<T::Caches>;

    #[inline]
    fn id(&self) -> WidgetID {
        self.id
    }

    fn _render(
        &mut self,
        path: &mut NewPathStack,
        stack: StdRenderProps<'_,dyn QueronDyn<E>+'_,E,()>,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        let render_props = stack;
        let render_props_inside = render_props.inside_spacing_border();

        if self.layouted_dims != Some(render_props_inside.absolute_bounds.size) || self.layouted_constraints.is_none() {
            self.layouted_constraints = Some(self.childs.constraints(Some(render_props_inside.absolute_bounds.size), self.orientation, path, &render_props, root.fork(), ctx));
            self.layouted_dims = Some(render_props_inside.absolute_bounds.size);
            self.rerender_full = true;
            self.rerender_childs = true;
        }

        force_render |= self.rerender_full;

        let mut need_render = force_render | self.rerender_childs;

        if force_render {
            renderer.fill_rect(
                &render_props
                    .with_style_color_type(TestStyleColorType::Bg),
                ctx
            );
        } else if need_render {
            renderer.fill_border_inner(
                &render_props
                    .with_style_color_type(TestStyleColorType::Bg)
                    .with_style_border_type(TestStyleBorderType::Spacing),
                ctx
            );
        }

        self.childs.render(path, &render_props_inside, renderer, force_render, &mut cache.0, root, ctx);

        self.rerender_childs = false;
        self.rerender_full = false;
        //TODO FIX viewport
    }

    fn _event_direct(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn event_new::EventDyn<E>+'_),
        mut route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        let stack = with_inside_spacing_border(stack);

        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        //dbg!(event._debug(),&event_mode);

        if !event_mode.route_to_childs {return Invalidation::valid();}

        if self.layouted_dims != Some(bounds.bounds.size) || self.layouted_constraints.is_none() {
            self.layouted_constraints = Some(self.childs.constraints(Some(bounds.bounds.size), self.orientation, path, &stack, root.fork(), ctx));
            self.layouted_dims = Some(bounds.bounds.size);
            self.rerender_full = true;
            self.rerender_childs = true;
        }

        if route_to_widget.as_ref().map_or(false, |&r| r.fetch().is_empty() ) {
            if !event_mode.childs_after_resolve {
                // If the event shouldn't broadcast to child widgets of the resolved widget
                return Invalidation::valid();
            }
            // If the event now resolved to us, disable route_to_widget and send to all childs
            route_to_widget = None;
        }

        let vali = self.childs.event(path, &stack, &bounds, event, route_to_widget, root, ctx);

        if vali.render {
            self.rerender_childs = true;
        }
        if vali.layout {
            self.rerender_full = true;
            self.layouted_constraints = None;
            self.layouted_dims = None;
        }

        vali
    }

    fn _size(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> {
        widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| //TODO no bounds available in Widget::size
                self.childs.constraints(None, self.orientation, path, &stack, root, ctx)
        )
    }

    // fn child_bounds<P,Ph>(&self, path: &mut NewPathStack,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> {
    //     //TODO holy stack
    //     // let mut child_sizes = Vec::with_capacity(self.childs());

    //     // self.childs.all(
    //     //     AsWidgetsAllClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>|
    //     //         //TODO bounds could never be used in constraints calc, else we would already need to have the child bounds calculates, which also requires the constraints
    //     //         child_sizes.push( widget.size(SimpleId(_) + path, &stack,root,ctx) )
    //     //     ),
    //     //     root,ctx
    //     // );

    //     // let bounds = calc_bounds(&b.size,&child_sizes,std::convert::identity,self.orientation); 

    //     // Ok(bounds)
    //     todo!()
    // }
    fn childs(&self) -> Range<isize> {
        0 .. self.childs.len() as isize
    }

    fn _call_tabulate_on_child_idx(
        &self,
        idx: isize,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        op: TabulateOrigin,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse,E::Error> {
        self.childs._call_tabulate_on_child_idx(idx as usize, path, stack, op, dir, root, ctx)
    }

    fn focusable(&self) -> bool {
        false
    }

    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.childs.end(path, root, ctx)
    }

    #[inline]
    fn respond_query<'a>(&'a self, _: crate::traitcast::WQueryResponder<'_,'a,E>) {}

    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation {
        let vali = self.childs.update(path, route, root, ctx);

        if vali.render {
            self.rerender_childs = true;
        }
        if vali.layout {
            self.rerender_full = true;
            self.layouted_constraints = None;
            self.layouted_dims = None;
        }

        vali
    }

    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        self.childs.by_index_dyn(idx as usize).map(Into::into)
    }

    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        self.childs.by_index_dyn_mut(idx as usize).map(Into::into)
    }

    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        self.childs.idx_range_dyn(range.start as usize .. range.end as usize, &mut |r| callback(r.into()) )
    }

    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        self.childs.idx_range_dyn_mut(range.start as usize .. range.end as usize, &mut |r| callback(r.into()) )
    }

    fn resolve_child_dyn<'a,'b>(&'a self, path: PathSliceRef<'b>) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        if let PathSliceMatch::Match(idx, path_inner) = path.fetch().slice_forward::<FixedIdx>() {
            self.childs.by_index_dyn(idx.0 as usize).map(|r| WidgetChildResolveDynResult {
                idx: idx.0,
                sub_path: path_inner,
                widget_id: r.widget_id,
                widget: r.widget,
            })
        } else {
            None
        }
    }

    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: PathSliceRef<'b>) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        if let PathSliceMatch::Match(idx, path_inner) = path.fetch().slice_forward::<FixedIdx>() {
            self.childs.by_index_dyn_mut(idx.0 as usize).map(|r| WidgetChildResolveDynResultMut {
                idx: idx.0,
                sub_path: path_inner,
                widget_id: r.widget_id,
                widget: r.widget,
            })
        } else {
            None
        }
    }

    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) {
        self.childs.send_mutation(path, resolve, args, root, ctx)
    }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        self.childs.invalidate_recursive(vali)
    }

    fn respond_query_mut<'a>(&'a mut self, responder: crate::traitcast::WQueryResponder<'_,'a,E>) {
        
    }
}

#[derive(Default)]
pub struct PaneCache<T>(T) where T: Default + Sized + 'static;

impl<T,E> WidgetCache<E> for PaneCache<T> where E: Env, T: Default + Sized + 'static {

}
