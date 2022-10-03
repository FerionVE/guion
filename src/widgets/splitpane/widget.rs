use super::*;
use util::state::*;
use crate::dispatchor::{AsWidgetClosure, AsWidgetsClosure};
use crate::event::key::Key;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::style::standard::cursor::StdCursor;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::cache::{StdRenderCachors, WidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::{for_child_widget, QueryCurrentBounds, WithCurrentBounds}; //TODO fix req of this import

impl<'w,E,L,R,V,TrMut> Widget<E> for SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    (L,R): AsWidgets<'w,E>,
    L: AsWidget<'w,E>,
    R: AsWidget<'w,E>,
    V: AtomState<E,f32>,
    TrMut: TriggerMut<E>,
{
    type Cache = SplitPaneCache<L::WidgetCache,R::WidgetCache,E>;

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

        if need_render {
            renderer.fill_border_inner(
                &render_props
                    .with_style_color_type(TestStyleColorType::Bg)
                    .with_style_border_type(TestStyleBorderType::Spacing),
                ctx
            );
        }

        let render_props = render_props.inside_spacing_border();

        let bounds = self.calc_bounds(&render_props.absolute_bounds,self.state.get(ctx));

        if cache.center_start_cachor != Some(bounds[1] - render_props.absolute_bounds.off) {
            need_render = true;
            force_render = true;
            cache.center_start_cachor = Some(bounds[1] - render_props.absolute_bounds.off);
        }

        if need_render {
            if ctx.state().is_hovered(&self.id) {
                let cursor = match self.orientation {
                    Orientation::Horizontal => StdCursor::SizeWE,
                    Orientation::Vertical => StdCursor::SizeNS,
                };

                renderer.set_cursor_specific(&cursor.into(),ctx);
            }

            renderer.fill_rect(
                &render_props
                    .slice_absolute(&bounds[1])
                    .with_style_color_type(TestStyleColorType::Fg)
                    .with_vartype(
                        ctx.state().is_hovered(&self.id),
                        ctx.state().is_focused(&self.id),
                        false, //self.pressed(ctx).is_some(),
                        false, //self.locked, //TODO add locked
                    ),
                ctx
            );
        }

        {
            self.childs.0.with_widget(
                AsWidgetClosure::new(|widget: &<L as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    widget.render(
                        &for_child_widget(render_props.slice_absolute(&bounds[0]),widget),
                        renderer,
                        force_render, &mut cache.child_caches.0,
                        root,ctx
                    )
                }),
                root.fork(),ctx
            );
        }
        {
            self.childs.1.with_widget(
                AsWidgetClosure::new(|widget: &<R as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    widget.render(
                        &for_child_widget(render_props.slice_absolute(&bounds[2]),widget),
                        renderer,
                        force_render, &mut cache.child_caches.1,
                        root,ctx
                    )
                }),
                root,ctx
            );
        }
        {
            //TODO render center
        }
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

        let current = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        if !event_mode.route_to_childs && !event_mode.receive_self {return false;}

        let o = self.orientation;
        let mut bounds = self.calc_bounds(&current.bounds,self.state.get(ctx)); 

        let mut passed = false;

        if event_mode.receive_self {
        //if let Some(e) = e.slice_bounds(&bounds[1]).filter(&l) {
            if let Some(mm) = event.query_variant::<MouseMove,_>(&stack) {
                //if mouse is down and was pressed on us
                if let Some(_) = ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,self.id.clone()) {
                    let cursor = ctx.state().cursor_pos().expect("TODO");
                    let mut cx = cursor.par(o);
                    let (mut wx0, ww) = current.bounds.par(o);
                    let mut wx1 = wx0 + ww as i32;

                    let l_min = self.childs.0.with_widget(
                        AsWidgetClosure::new(|widget: &<L as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                            widget.size(&for_child_widget(&stack,widget), &mut cache.child_caches.0, root,ctx) //TODO It can't be! We can't already have the bounds for the widget when constraining
                        }),
                        root.fork(),ctx
                    ).par(o).min();
                    let r_min = self.childs.1.with_widget(
                        AsWidgetClosure::new(|widget: &<R as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                            widget.size(&for_child_widget(&stack,widget), &mut cache.child_caches.1, root,ctx)
                        }),
                        root.fork(),ctx
                    ).par(o).min();

                    wx0 += (self.width/2) as i32;
                    wx1 -= (self.width/2) as i32;

                    let ewx0 = wx0 + l_min as i32;
                    let ewx1 = wx1 - r_min as i32;

                    cx = cx.min(ewx1-1).max(ewx0);
                    
                    if ewx1 > ewx0 {
                        let ww = wx1 - wx0;
                        cx = cx - wx0;
                        let fcx = (cx as f32)/(ww as f32);

                        if let Some(t) = self.updater.boxed(fcx) {
                            ctx.mutate_closure(t)
                        }

                        bounds = self.calc_bounds(&current.bounds,fcx);
                    }
                }
            }
        //}
        }
        if event_mode.route_to_childs {
            self.childs.0.with_widget(
                AsWidgetClosure::new(|widget: &<L as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    let stack = WithCurrentBounds {
                        inner: for_child_widget(&stack,widget),
                        bounds: current.bounds & &bounds[0],
                        viewport: current.viewport.clone(),
                    };
        
                    passed |= widget.event_direct(&stack,event, &mut cache.child_caches.0, root,ctx);
                }),
                root.fork(),ctx
            );
            self.childs.1.with_widget(
                AsWidgetClosure::new(|widget: &<R as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    let stack = WithCurrentBounds {
                        inner: for_child_widget(&stack,widget),
                        bounds: current.bounds & &bounds[2],
                        viewport: current.viewport.clone(),
                    };
        
                    passed |= widget.event_direct(&stack,event, &mut cache.child_caches.1, root,ctx);
                }),
                root,ctx
            );
        }
        passed
    }

    fn _size<P>(
        &self,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| {
                let mut s = ESize::<E>::empty();

                self.childs.0.with_widget(
                    AsWidgetClosure::new(|widget: &<L as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                        s.add( &widget.size(&for_child_widget(&stack,widget), &mut cache.child_caches.0, root,ctx), self.orientation )
                    }),
                    root.fork(),ctx
                );
                s.add_space(self.width,self.orientation);
                self.childs.1.with_widget(
                    AsWidgetClosure::new(|widget: &<R as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                        s.add( &widget.size(&for_child_widget(&stack,widget), &mut cache.child_caches.1, root,ctx), self.orientation )
                    }),
                    root,ctx
                );

                s
            }
        );

        size
    }

    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        Ok(self.calc_bounds(b,self.state.get(ctx)).into())
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }

    fn with_child<'s,F,Ret>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> Ret
    where
        F: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> Ret
    {
        self.childs.by_index(
            i,
            AsWidgetsClosure::new(|_,_,_,widget:&<(L,R) as AsWidgets<'_, E>>::Widget<'_>,_,ctx: &mut E::Context<'_>|
                (callback)(Ok(widget.erase()),ctx)
            ),
            root,ctx
        ).unwrap_or_else(|| todo!()/*(callback)(Err(()),ctx)*/ )
    }

    fn focusable(&self) -> bool {
        false
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn AtomState<E,f32> => |s| &s.state;
    );
}

impl<'w,E,L,R,V,TrMut> SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
    V: AtomState<E,f32>,
{
    fn calc_bounds(&self, b: &Bounds, v: f32) -> [Bounds;3] { //TODO WHY does calc_bounds in pane return relative bounds and this does absolute bounds
        let handle_width = self.width.min(b.w());
        let o = self.orientation;
        let (x,w) = b.par(o);
        let (y,h) = b.unpar(o);
        let w0 = ((w as f32 - handle_width as f32)*v.clamp(0.0,1.0)) as u32;
        let w2 = w - w0 - handle_width;
        let x1 = x + w0 as i32;
        let x2 = x1 + handle_width as i32;
        let left = Bounds::from_ori(x, y, w0, h, o);
        let center = Bounds::from_ori(x1, y, handle_width, h, o);
        let right = Bounds::from_ori(x2, y, w2, h, o);
        [left,center,right]
    }
}

impl<'z,E,L,R,V,TrMut> AsWidget<'z,E> for SplitPane<'z,E,L,R,V,TrMut> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,F,Ret>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Ret
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,Ret,E>
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct SplitPaneCache<LCache,RCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LCache: WidgetCache<E>, RCache: WidgetCache<E> {
    child_caches: (LCache,RCache),
    std_render_cachors: Option<StdRenderCachors<E>>,
    center_start_cachor: Option<Bounds>,
    _p: PhantomData<E>,
    //TODO cachor borders and colors
}

impl<LCache,RCache,E> WidgetCache<E> for SplitPaneCache<LCache,RCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LCache: WidgetCache<E>, RCache: WidgetCache<E> {
    fn reset_current(&mut self) {
        self.child_caches.0.reset_current();
        self.child_caches.1.reset_current();
    }
}
