use crate::dispatchor::{AsWidgetsClosure, AsWidgetsAllClosure};
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::cache::{WidgetCache, StdRenderCachors};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::{QueryCurrentBounds, WithCurrentBounds, for_child_widget};

use super::*;

impl<'w,E,T> Widget<E> for Pane<'w,E,T> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    T: AsWidgets<'w,E>,
{
    type Cache = PaneCache<E,T::WidgetCache>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        let render_props = StdRenderProps::new(stack);

        render_props.current_std_render_cachors()
            .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

        need_render |= !cache.layout_rendered;

        if need_render {
            renderer.fill_border_inner(
                &render_props
                    .with_style_color_type(TestStyleColorType::Bg)
                    .with_style_border_type(TestStyleBorderType::Spacing),
                ctx
            );
        }

        let render_props = render_props.inside_spacing_border();

        let new_layout = self.do_layout(
            &render_props,
            render_props.absolute_bounds.size,
            cache,
            false, //TODO properly transfer rerender influence from parents
            root.fork(),ctx
        );

        self.childs.all(
            AsWidgetsAllClosure::new(|idx,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                widget.render(
                    &for_child_widget(
                        render_props
                            .slice(cache.childs[idx].relative_bounds_cache.unwrap()),
                        widget
                    ),
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

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);

        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        //dbg!(event._debug(),&event_mode);

        if !event_mode.route_to_childs {return false;}

        self.do_layout(
            &stack,
            bounds.bounds.size,
            cache,
            false, //TODO properly transfer rerender influence from parents
            root.fork(),ctx
        );

        let mut passed = false;

        self.childs.all(
            AsWidgetsAllClosure::new(|idx,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                let stack = WithCurrentBounds {
                    inner: for_child_widget(&stack,widget),
                    bounds: bounds.bounds.slice(cache.childs[idx].relative_bounds_cache.as_ref().unwrap()),
                    viewport: bounds.viewport.clone(),
                };
    
                passed |= widget.event_direct(&stack,event,&mut cache.childs[idx].widget_cache,root,ctx);
            }),
            root,ctx
        );

        passed
    }

    fn _size<P>(
        &self,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| //TODO no bounds available in Widget::size
                self.do_gonstraints(
                    &stack,
                    cache,
                    root,ctx
                )
        )
    }

    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        //TODO holy stack
        // let mut child_sizes = Vec::with_capacity(self.childs());

        // self.childs.all(
        //     AsWidgetsAllClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>|
        //         //TODO bounds could never be used in constraints calc, else we would already need to have the child bounds calculates, which also requires the constraints
        //         child_sizes.push( widget.size(&for_child_widget(&stack,widget),root,ctx) )
        //     ),
        //     root,ctx
        // );

        // let bounds = calc_bounds(&b.size,&child_sizes,std::convert::identity,self.orientation); 

        // Ok(bounds)
        todo!()
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        self.childs.by_index(
            i,
            AsWidgetsClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,_,ctx: &mut E::Context<'_>|
                (callback)(Ok(widget.erase()),ctx)
            ),
            root,ctx
        ).unwrap_or_else(|| todo!()/*(callback)(Err(()),ctx)*/ )
    }

    fn focusable(&self) -> bool {
        false
    }
}

impl<'w,E,T> Pane<'w,E,T> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    T: AsWidgets<'w,E>,
{
    fn do_gonstraints(
        &self,
        stack: &(impl Queron<E> + ?Sized),
        cache: &mut PaneCache<E,T::WidgetCache>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> ESize<E> {
        cache.childs.resize_with(self.childs(), Default::default);

        if cache.current_gonstraints.is_some() {return cache.current_gonstraints.clone().unwrap();}

        let mut all_gonstraints = ESize::<E>::empty();

        for (idx,child_cache) in cache.childs.iter_mut().enumerate() {
            self.childs.by_index(
                idx,
                AsWidgetsClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    if child_cache.widget_id != Some(widget.id()) {
                        *child_cache = Default::default();
                        child_cache.widget_id = Some(widget.id());
                    }

                    let current_gonstraint = child_cache.current_gonstraint.get_or_insert_with(||
                        widget.size(&for_child_widget(&stack,widget), &mut child_cache.widget_cache, root,ctx)
                    );

                    all_gonstraints.add(&current_gonstraint, self.orientation);
                }),
                root.fork(),ctx
            );
        }

        cache.current_gonstraints = Some(all_gonstraints.clone());

        all_gonstraints
    }

    fn do_layout(
        &self,
        stack: &(impl Queron<E> + ?Sized),
        dims_inside_border: Dims,
        cache: &mut PaneCache<E,T::WidgetCache>,
        mut need_relayout: bool,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> bool {
        cache.childs.resize_with(self.childs(), Default::default);

        if cache.orientation_cachor != Some((dims_inside_border,self.orientation)) {
            need_relayout = true;
            cache.orientation_cachor = Some((dims_inside_border,self.orientation));
        }

        cache.current_layouted &= !need_relayout;

        if cache.current_layouted {return false;}

        let mut all_gonstraints = ESize::<E>::empty();

        for (idx,child_cache) in cache.childs.iter_mut().enumerate() {
            self.childs.by_index(
                idx,
                AsWidgetsClosure::new(|_,_,_,widget:&<T as AsWidgets<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    if child_cache.widget_id != Some(widget.id()) {
                        *child_cache = Default::default();
                        child_cache.widget_id = Some(widget.id());
                        need_relayout = true;
                    }

                    need_relayout |= child_cache.relative_bounds_cache.is_none();

                    let current_gonstraint = child_cache.current_gonstraint.get_or_insert_with(||
                        widget.size(&for_child_widget(&stack,widget), &mut child_cache.widget_cache, root,ctx)
                    );

                    all_gonstraints.add(&current_gonstraint, self.orientation);

                    if child_cache.gonstraint_cachor != child_cache.current_gonstraint {
                        child_cache.gonstraint_cachor = child_cache.current_gonstraint.clone();
                        need_relayout = true;
                    }
                }),
                root.fork(),ctx
            );
        }

        cache.current_layouted &= !need_relayout;

        cache.current_gonstraints = Some(all_gonstraints);

        if need_relayout {
            cache.layout_rendered = false;

            let new_bounds = calc_bounds(
                &dims_inside_border,
                &cache.childs,
                |child| child.gonstraint_cachor.clone().unwrap(),
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

impl<'z,E,T> AsWidget<'z,E> for Pane<'z,E,T> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct PaneCache<E,ChildCache> where E: Env, ChildCache: WidgetCache<E> {
    std_render_cachors: Option<StdRenderCachors<E>>,
    orientation_cachor: Option<(Dims,Orientation)>,
    childs: Vec<PaneCacheChild<E,ChildCache>>,
    //valid_layout: bool,
    current_gonstraints: Option<ESize<E>>,
    current_layouted: bool,
    layout_rendered: bool,
    //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
}

pub struct PaneCacheChild<E,ChildCache> where E: Env, ChildCache: WidgetCache<E> {
    current_gonstraint: Option<ESize<E>>,
    relative_bounds_cache: Option<Bounds>,
    gonstraint_cachor: Option<ESize<E>>,
    widget_id: Option<E::WidgetID>,
    widget_cache: ChildCache,
}

impl<E,ChildCache> Default for PaneCacheChild<E,ChildCache> where E: Env, ChildCache: WidgetCache<E> {
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

impl<E,ChildCache> WidgetCache<E> for PaneCache<E,ChildCache> where E: Env, ChildCache: WidgetCache<E> {
    fn reset_current(&mut self) {
        self.current_gonstraints = None;
        self.current_layouted = false;
        for c in &mut self.childs {
            c.current_gonstraint = None;
            c.widget_cache.reset_current();
        }
    }
}
