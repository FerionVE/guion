use std::marker::PhantomData;

use crate::aliases::{ERenderer, ESize, EEvent};
use crate::ctx::Context;
use crate::dispatchor::{AsWidgetClosure, AsWidgetsResolveClosure, AsWidgetDispatch, AsWidgetsClosure};
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::MouseMove;
use crate::layout::GonstraintAxis;
use crate::layout::Gonstraints;
use crate::layout::Orientation;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::style::standard::cursor::StdCursor;
use crate::{event_new, EventResp, impl_traitcast};
use crate::newpath::{PathStack, PathResolvusDyn, FixedIdx, PathResolvus, PathFragment};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, TestStyleBorderType, widget_size_inside_border_type, with_inside_spacing_border, TestStyleVariant};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::util::bounds::Bounds;
use crate::util::tabulate::{TabulateResponse, TabulateOrigin, TabulateDirection};
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widget::cache::{WidgetCache, StdRenderCachors};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::as_widget::AsWidget;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::as_widgets::fixed_idx::WidgetsFixedIdx;
use crate::widget::stack::{QueryCurrentBounds, WithCurrentBounds};
use crate::widgets::soft_single_child_resolve_check;
use crate::widgets::util::state::AtomState;

use super::SplitPane;

impl<E,L,R,V,TrMut> Widget<E> for SplitPane<E,L,R,V,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    WidgetsFixedIdx<(L,R)>: AsWidgets<E>,
    L: AsWidget<E>,
    R: AsWidget<E>,
    V: AtomState<E,f32>,
    TrMut: MutorEndBuilder<f32,E>,
{
    type Cache = SplitPaneCache<L::WidgetCache,R::WidgetCache,E>;

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


        let bounds = self.calc_bounds(&render_props.inside_spacing_border().absolute_bounds,self.state.get(ctx));

        if cache.center_start_cachor != Some(bounds[1] - render_props.inside_spacing_border().absolute_bounds.off) {
            need_render = true;
            force_render = true;
            cache.center_start_cachor = Some(bounds[1] - render_props.inside_spacing_border().absolute_bounds.off);
        }

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

        let render_props = render_props.inside_spacing_border();

        if ctx.state().is_hovered(path._erase()) {
            let cursor = match self.orientation {
                Orientation::Horizontal => StdCursor::SizeWE,
                Orientation::Vertical => StdCursor::SizeNS,
            };

            renderer.set_cursor_specific(&cursor.into(),ctx);
        }

        if need_render {
            renderer.fill_rect(
                &render_props
                    .slice_absolute(&bounds[1])
                    .with_style_color_type(TestStyleColorType::Fg)
                    .with_style_type(
                        TestStyleVariant {
                            hovered: ctx.state().is_hovered(path._erase()),
                            selected: ctx.state().is_focused(path._erase()),
                            activated: false, //self.pressed(ctx).is_some(),
                            disabled: false, //self.locked, //TODO add locked
                            ..Default::default()
                        }
                    )
                    ,
                ctx
            );
        }

        self.childs.0.0.with_widget(
            &mut AsWidgetClosure::<_,L,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                widget.render(
                    &FixedIdx(0usize).push_on_stack(path),
                    &render_props.slice_absolute(&bounds[0]),
                    renderer,
                    force_render, &mut cache.child_caches.0,
                    root,ctx
                )
            }),
            root.fork(),ctx
        );
        self.childs.0.1.with_widget(
            &mut AsWidgetClosure::<_,R,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                widget.render(
                    &FixedIdx(1usize).push_on_stack(path),
                    &render_props.slice_absolute(&bounds[2]),
                    renderer,
                    force_render, &mut cache.child_caches.1,
                    root,ctx
                )
            }),
            root,ctx
        );
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

        let current = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self && route_to_widget.map_or(true, |i| i.inner().is_none() );

        if !event_mode.route_to_childs && !receive_self {return false;}

        let o = self.orientation;
        let mut bounds = self.calc_bounds(current.bounds,self.state.get(ctx)); 

        let mut passed = false;

        if receive_self {
        //if let Some(e) = e.slice_bounds(&bounds[1]).filter(&l) {
            if let Some(mm) = event.query_variant::<MouseMove>(path,&stack) {
                //if mouse is down and was pressed on us
                if let Some(_) = ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,path._erase()) {
                    let cursor = ctx.state().cursor_pos().expect("TODO");
                    let mut cx = cursor.par(o);
                    let (mut wx0, ww) = current.bounds.par(o);
                    let mut wx1 = wx0 + ww as i32;

                    let l_min = self.childs.0.0.with_widget(
                        &mut AsWidgetClosure::<_,L,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                            widget.size(&FixedIdx(0usize).push_on_stack(path), &stack, &mut cache.child_caches.0, root,ctx) //TODO It can't be! We can't already have the bounds for the widget when constraining
                        }),
                        root.fork(),ctx
                    ).par(o).min();
                    let r_min = self.childs.0.1.with_widget(
                        &mut AsWidgetClosure::<_,R,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                            widget.size(&FixedIdx(1usize).push_on_stack(path), &stack, &mut cache.child_caches.1, root,ctx)
                        }),
                        root.fork(),ctx
                    ).par(o).min();

                    wx0 += (self.width/2) as i32;
                    wx1 -= (self.width/2) as i32;

                    let ewx0 = wx0 + l_min as i32;
                    let ewx1 = wx1 - r_min as i32;

                    cx = cx.clamp(ewx0, ewx1-1);
                    
                    if ewx1 > ewx0 {
                        let ww = wx1 - wx0;
                        cx = cx - wx0;
                        let fcx = (cx as f32)/(ww as f32);

                        if let Some(t) = self.updater.build_box_mut_event(fcx) {
                            ctx.mutate_closure(t)
                        }

                        bounds = self.calc_bounds(current.bounds,fcx);
                    }
                }
            }
        //}
        }
        if event_mode.route_to_childs {
            if route_to_widget.as_ref().map_or(false, |&r| PathResolvus::inner(r).is_none() ) {
                // If the event now resolved to us, disable route_to_widget and send to all childs
                route_to_widget = None;
            }

            if soft_single_child_resolve_check(route_to_widget, FixedIdx(0)) {
                self.childs.0.0.with_widget(
                    &mut AsWidgetClosure::<_,L,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                        let stack = WithCurrentBounds {
                            inner: &stack,
                            bounds: current.bounds & &bounds[0],
                            viewport: current.viewport.clone(),
                        };

                        passed |= widget.event_direct(
                            &FixedIdx(0).push_on_stack(path),
                            &stack,
                            event,
                            route_to_widget.and_then(PathResolvus::inner),
                            &mut cache.child_caches.0, 
                            root,ctx
                        );
                    }),
                    root.fork(),ctx
                );
            }
            if soft_single_child_resolve_check(route_to_widget, FixedIdx(1)) {
                self.childs.0.1.with_widget(
                    &mut AsWidgetClosure::<_,R,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                        let stack = WithCurrentBounds {
                            inner: &stack,
                            bounds: current.bounds & &bounds[2],
                            viewport: current.viewport.clone(),
                        };

                        passed |= widget.event_direct(
                            &FixedIdx(1).push_on_stack(path),
                            &stack,
                            event,
                            route_to_widget.and_then(PathResolvus::inner),
                            &mut cache.child_caches.1, 
                            root,ctx
                        );
                    }),
                    root.fork(),ctx
                );
            }
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
        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| {
                let mut s = ESize::<E>::add_base(self.orientation);

                self.childs.0.0.with_widget(
                    &mut AsWidgetClosure::<_,L,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                        s.add( &widget.size(&FixedIdx(0usize).push_on_stack(path), &stack, &mut cache.child_caches.0, root,ctx), self.orientation )
                    }),
                    root.fork(),ctx
                );
                s.add_space(self.width,self.orientation);
                self.childs.0.1.with_widget(
                    &mut AsWidgetClosure::<_,R,_,E>::new(|widget,root,ctx: &mut E::Context<'_>| {
                        s.add( &widget.size(&FixedIdx(1usize).push_on_stack(path), &stack, &mut cache.child_caches.1, root,ctx), self.orientation )
                    }),
                    root,ctx
                );

                s
            }
        );

        size
    }

    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     Ok(self.calc_bounds(b,self.state.get(ctx)).into())
    // }
    fn childs(&self) -> usize {
        self.childs.len()
    }

    fn with_child<'s,F,Ret>(
        &'s self,
        i: usize,
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> Ret
    where
        F: for<'www,'ww,'c,'cc> FnMut(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> Ret
    {
        self.childs.by_index(
            i,
            &mut AsWidgetsClosure::<_,WidgetsFixedIdx<(L,R)>,_,E>::new(#[inline] |result,_,ctx: &mut E::Context<'_>|
                match result {
                    Some(v) => (callback)(Ok(v.widget.erase()),ctx),
                    None => (callback)(Err(()),ctx)
                }
            ),
            root,ctx
        )
    }

    fn with_resolve_child<'s,F,Ret>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> Ret
    where
        F: for<'a,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> Ret
    {
        if sub_path.inner().is_none() { return (callback)(Err(todo!()),ctx); }

        self.childs.resolve(
            sub_path,
            &mut AsWidgetsResolveClosure::<_,WidgetsFixedIdx<(L,R)>,_,E>::new(|result,root,ctx: &mut E::Context<'_>| {
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
        let rootf = root.fork();

        self.childs.by_index(
            idx,
            &mut AsWidgetsClosure::<_,WidgetsFixedIdx<(L,R)>,_,E>::new(#[inline] |result,_,ctx: &mut E::Context<'_>|
                match result {
                    Some(v) => v.widget._tabulate(&v.child_id.push_on_stack(path), stack, op.clone(), dir, root.fork(), ctx),
                    None => Err(todo!()),
                }
            ),
            rootf,ctx
        )
    }

    fn focusable(&self) -> bool {
        false
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn AtomState<E,f32> => |s| &s.state;
    );
}

impl<E,L,R,V,TrMut> SplitPane<E,L,R,V,TrMut> where
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

impl<E,L,R,V,TrMut> AsWidget<E> for SplitPane<E,L,R,V,TrMut> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where Self: 'v;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<Ret>(&self, f: &mut (dyn AsWidgetDispatch<Self,Ret,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Ret {
        f.call(self, root, ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        w
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
