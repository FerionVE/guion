use crate::queron::query::Query;
use crate::root::RootRef;
use crate::{EventResp, event_new};
use crate::aliases::{ERenderer, ESize};
use crate::dispatchor::{AsWidgetsResolveResult, AsWidgetsResolveClosure, AsWidgetsAllClosure, AsWidgetsClosure, AsWidgetsResult, AsWidgetDispatch};
use crate::env::Env;
use crate::layout::{Gonstraints, Orientation};
use crate::layout::calc::calc_bounds2;
use crate::layout::size::StdGonstraintAxis;
use crate::newpath::{PathStack, PathResolvusDyn, PathResolvus, PathFragment};
use crate::queron::Queron;
use crate::render::widgets::RenderStdWidgets;
use crate::render::{with_inside_spacing_border, widget_size_inside_border_type, TestStyleBorderType, TestStyleColorType, StdRenderProps};
use crate::util::bounds::{Dims, Bounds};
use crate::util::tabulate::{TabulateResponse, TabulateDirection, TabulateOrigin};
use crate::widget::as_widget::AsWidget;
use crate::widget::cache::{WidgetCache, StdRenderCachors};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::as_widgets::AsWidgets;
use crate::widget::stack::{QueryCurrentBounds, WithCurrentBounds};

use super::Pane;

impl<E,T> Widget<E> for Pane<E,T> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    T: AsWidgets<E>,
{
    type Cache = PaneCache<E,T::WidgetCache,T::ChildID>;

    fn _render<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        let render_props = StdRenderProps::new(stack);

        render_props.current_std_render_cachors()
            .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

        need_render |= !cache.layout_rendered;

        let new_layout = self.do_layout(
            path,
            &render_props,
            render_props.inside_spacing_border().absolute_bounds.size,
            cache,
            false, //TODO properly transfer rerender influence from parents
            root.fork(),ctx
        );

        if force_render | new_layout {
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

        let render_props = render_props.inside_spacing_border();

        self.childs.idx_range(
            0..self.childs.len(),
            &mut AsWidgetsAllClosure::new(|idx,child_id: <T as AsWidgets<E>>::ChildID,widget:&<T as AsWidgets<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                widget.render(
                    &child_id.push_on_stack(path),
                    &render_props
                        .slice(cache.childs[idx].relative_bounds_cache.unwrap()),
                    renderer,
                    new_layout | force_render,&mut cache.childs[idx].widget_cache,
                    root,ctx
                )
            }),
            root,ctx
        );

        cache.layout_rendered = true;
        //TODO FIX viewport
    }

    fn _event_direct<P,Ph,Evt>(
        &self,
        path: &Ph,
        stack: &P,
        event: &Evt,
        mut route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);

        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        //dbg!(event._debug(),&event_mode);

        if !event_mode.route_to_childs {return false;}

        self.do_layout(
            path,
            &stack,
            bounds.bounds.size,
            cache,
            false, //TODO properly transfer rerender influence from parents
            root.fork(),ctx
        );

        let mut passed = false;

        if route_to_widget.as_ref().map_or(false, |&r| PathResolvus::inner(r).is_none() ) {
            // If the event now resolved to us, disable route_to_widget and send to all childs
            route_to_widget = None;
        }

        if let Some(route_to_widget) = route_to_widget {
            self.childs.resolve(
                route_to_widget,
                &mut AsWidgetsResolveClosure::new(|result: Option<AsWidgetsResolveResult<T,E>>,root,ctx: &mut E::Context<'_>| {
                    if let Some(result) = result {
                        let stack = WithCurrentBounds {
                            inner: &stack,
                            bounds: bounds.bounds.slice(cache.childs[result.idx].relative_bounds_cache.as_ref().unwrap()),
                            viewport: bounds.viewport.clone(),
                        };
            
                        passed |= result.widget.event_direct(
                            &result.child_id.push_on_stack(path),
                            &stack,
                            event,
                            Some(result.resolvus),
                            &mut cache.childs[result.idx].widget_cache,
                            root,ctx);
                    }
                }),
                root,ctx
            )
        } else {
            self.childs.idx_range(
                0..self.childs.len(), //TODO there could be a prefilter which checks whether idx child bounds visible in visible-filter mode
                &mut AsWidgetsAllClosure::new(|idx,child_id: <T as AsWidgets<E>>::ChildID,widget:&<T as AsWidgets<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                    let stack = WithCurrentBounds {
                        inner: &stack,
                        bounds: bounds.bounds.slice(cache.childs[idx].relative_bounds_cache.as_ref().unwrap()),
                        viewport: bounds.viewport.clone(),
                    };
        
                    passed |= widget.event_direct(
                        &child_id.push_on_stack(path),
                        &stack,
                        event,
                        None, //TODO this should be done by the AsWidgets
                        &mut cache.childs[idx].widget_cache,
                        root,ctx);
                }),
                root,ctx
            );
        }

        passed
    }

    fn _size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| //TODO no bounds available in Widget::size
                self.do_gonstraints(
                    path,
                    &stack,
                    cache,
                    root,ctx
                )
        )
    }

    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
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
    fn childs(&self) -> usize {
        self.childs.len()
    }
    
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'www,'ww,'c,'cc> FnMut(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        self.childs.by_index(
            i,
            &mut AsWidgetsClosure::new(#[inline] |result: Option<AsWidgetsResult<T,E>>,_,ctx: &mut E::Context<'_>|
                match result {
                    Some(v) => (callback)(Ok(v.widget.erase()),ctx),
                    None => (callback)(Err(()),ctx)
                }
            ),
            root,ctx
        )
    }

    fn with_resolve_child<'s,F,R>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'a,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R
    {
        if sub_path.inner().is_none() { return (callback)(Err(todo!()),ctx); }

        self.childs.resolve(
            sub_path,
            &mut AsWidgetsResolveClosure::new(|result: Option<AsWidgetsResolveResult<T,E>>,root,ctx: &mut E::Context<'_>| {
                match result {
                    Some(result) => (callback)(Ok(WidgetWithResolveChildDyn {
                        idx: result.idx,
                        sub_path: result.resolvus,
                        widget: result.widget.erase(),
                    }),ctx),
                    None => (callback)(Err(todo!()),ctx),
                }
            }),
            root,ctx,
        )
    }

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        idx: usize,
        path: &Ph,
        stack: &P,
        op: TabulateOrigin<E>,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized
    {
        self.childs.by_index(
            idx,
            &mut AsWidgetsClosure::new(#[inline] |result: Option<AsWidgetsResult<T,E>>,_,ctx: &mut E::Context<'_>|
                match result {
                    Some(v) => v.widget._tabulate(&v.child_id.push_on_stack(path), stack, op.clone(), dir, root.fork(), ctx),
                    None => Err(todo!()),
                }
            ),
            root.fork(),ctx
        )
    }

    fn focusable(&self) -> bool {
        false
    }
}

impl<E,T> Pane<E,T> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    T: AsWidgets<E>,
{
    fn do_gonstraints(
        &self,
        path: &(impl PathStack<E> + ?Sized),
        stack: &(impl Queron<E> + ?Sized),
        cache: &mut PaneCache<E,T::WidgetCache,T::ChildID>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> {
        if let Some(gonstraints) = cache.current_gonstraints.as_ref() {return gonstraints.clone();}

        cache.childs.resize_with(self.childs(), Default::default);

        let mut all_gonstraints = ESize::<E>::add_base(self.orientation);

        self.childs.idx_range(
            0..self.childs.len(),
            &mut AsWidgetsAllClosure::new(|idx,child_id: <T as AsWidgets<E>>::ChildID,widget:&<T as AsWidgets<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                let child_cache = &mut cache.childs[idx];

                if child_cache.widget_id != Some(child_id.clone()) {
                    *child_cache = Default::default();
                    child_cache.widget_id = Some(child_id.clone());
                }

                let current_gonstraint = child_cache.current_gonstraint.get_or_insert_with(||
                    widget.size(&child_id.push_on_stack(path), &stack, &mut child_cache.widget_cache, root,ctx)
                );

                all_gonstraints.add(current_gonstraint, self.orientation);
            }),
            root.fork(),ctx
        );

        cache.current_gonstraints = Some(all_gonstraints.clone());

        all_gonstraints
    }

    fn do_layout(
        &self,
        path: &(impl PathStack<E> + ?Sized),
        stack: &(impl Queron<E> + ?Sized),
        dims_inside_border: Dims,
        cache: &mut PaneCache<E,T::WidgetCache,T::ChildID>,
        mut need_relayout: bool,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> bool {
        if cache.orientation_cachor != Some((dims_inside_border,self.orientation)) {
            need_relayout = true;
            cache.orientation_cachor = Some((dims_inside_border,self.orientation));
        }

        cache.current_layouted &= !need_relayout;

        if cache.current_layouted {return false;}

        cache.childs.resize_with(self.childs(), Default::default);

        let mut all_gonstraints = ESize::<E>::add_base(self.orientation);

        self.childs.idx_range(
            0..self.childs.len(),
            &mut AsWidgetsAllClosure::new(|idx,child_id: <T as AsWidgets<E>>::ChildID,widget:&<T as AsWidgets<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                let child_cache = &mut cache.childs[idx];

                if child_cache.widget_id != Some(child_id.clone()) {
                    *child_cache = Default::default();
                    child_cache.widget_id = Some(child_id.clone());
                    need_relayout = true;
                }

                need_relayout |= child_cache.relative_bounds_cache.is_none();

                let current_gonstraint = child_cache.current_gonstraint.get_or_insert_with(||
                    widget.size(&child_id.push_on_stack(path), &stack, &mut child_cache.widget_cache, root,ctx)
                );

                all_gonstraints.add(current_gonstraint, self.orientation);

                if child_cache.gonstraint_cachor != child_cache.current_gonstraint {
                    child_cache.gonstraint_cachor = child_cache.current_gonstraint.clone();
                    need_relayout = true;
                }
            }),
            root.fork(),ctx
        );

        cache.current_layouted &= !need_relayout;

        cache.current_gonstraints = Some(all_gonstraints);

        if need_relayout {
            cache.layout_rendered = false;

            let par_axis = cache.childs.iter()
                .map(|child_cache| child_cache.gonstraint_cachor.clone().unwrap().into().par(self.orientation) )
                .collect::<Vec<StdGonstraintAxis>>();

            let new_bounds = calc_bounds2(
                &dims_inside_border,
                &par_axis,
                self.orientation,
            );

            assert_eq!(new_bounds.len(),cache.childs.len());
            assert_eq!(new_bounds.len(),self.childs());

            for (child_cache,new_bound) in cache.childs.iter_mut().zip(new_bounds) {
                child_cache.relative_bounds_cache = Some(new_bound);
            }
        }

        cache.current_layouted = true;

        need_relayout
    }
}

impl<E,T> AsWidget<E> for Pane<E,T> where Self: Widget<E>, E: Env {
    type Widget<'v,'z> = Self where 'z: 'v, Self: 'z;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,R>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        f.call(self, root, ctx)
    }
}

pub struct PaneCache<E,ChildCache,ChildIDCachor> where E: Env, ChildCache: WidgetCache<E>, ChildIDCachor: Clone + 'static {
    std_render_cachors: Option<StdRenderCachors<E>>,
    orientation_cachor: Option<(Dims,Orientation)>,
    childs: Vec<PaneCacheChild<E,ChildCache,ChildIDCachor>>,
    //valid_layout: bool,
    current_gonstraints: Option<ESize<E>>,
    current_layouted: bool,
    layout_rendered: bool,
    //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
}

pub struct PaneCacheChild<E,ChildCache,ChildIDCachor> where E: Env, ChildCache: WidgetCache<E>, ChildIDCachor: Clone + 'static {
    current_gonstraint: Option<ESize<E>>,
    relative_bounds_cache: Option<Bounds>,
    gonstraint_cachor: Option<ESize<E>>,
    widget_id: Option<ChildIDCachor>, //TODO ALARM how to cachor ChildIDs
    widget_cache: ChildCache,
}

impl<E,ChildCache,ChildIDCachor> Default for PaneCacheChild<E,ChildCache,ChildIDCachor> where E: Env, ChildCache: WidgetCache<E>, ChildIDCachor: Clone + 'static {
    #[inline]
    fn default() -> Self {
        Self {
            relative_bounds_cache: None,
            gonstraint_cachor: None,
            widget_id: None,
            widget_cache: Default::default(),
            current_gonstraint: None,
        }
    }
}

impl<E,ChildCache,ChildIDCachor> Default for PaneCache<E,ChildCache,ChildIDCachor> where E: Env, ChildCache: WidgetCache<E>, ChildIDCachor: Clone + 'static {
    #[inline]
    fn default() -> Self {
        Self {
            std_render_cachors: None,
            orientation_cachor: None,
            childs: Vec::new(),
            current_gonstraints: None,
            current_layouted: false,
            layout_rendered: false,
        }
    }
}

impl<E,ChildCache,ChildIDCachor> WidgetCache<E> for PaneCache<E,ChildCache,ChildIDCachor> where E: Env, ChildCache: WidgetCache<E>, ChildIDCachor: Clone + 'static {
    fn reset_current(&mut self) {
        self.current_gonstraints = None;
        self.current_layouted = false;
        for c in &mut self.childs {
            c.current_gonstraint = None;
            c.widget_cache.reset_current();
        }
    }
}
