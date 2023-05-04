use std::marker::PhantomData;

use crate::aliases::{ERenderer, ESize, EEvent};
use crate::ctx::Context;
use crate::dispatchor::{AsWidgetClosure, AsWidgetDispatch, AsWidgetsOnWithChild, AsWidgetsOnWithResolveChild, AsWidgetsOnTabulate};
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
use crate::{event_new, EventResp};
use crate::newpath::{PathStack, PathResolvusDyn, FixedIdx, PathResolvus, PathFragment};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, TestStyleBorderType, widget_size_inside_border_type, with_inside_spacing_border, TestStyleVariant};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::util::bounds::Bounds;
use crate::util::tabulate::{TabulateResponse, TabulateOrigin, TabulateDirection};
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widget::cache::{RenderCache, StdRenderCachors};
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

    fn _render(
        &self,
        path: &mut NewPathStack,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
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

        if ctx.state().is_hovered(path) {
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
                            hovered: ctx.state().is_hovered(path),
                            selected: ctx.state().is_focused(path),
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
            &mut AsWidgetClosure::<'_,_,L,_,E>::new(|widget,root,ctx| {
                widget.render(
                    &FixedIdx(0usize) path,
                    &render_props.slice_absolute(&bounds[0]),
                    renderer,
                    force_render, &mut cache.child_caches.0,
                    root,ctx
                )
            }),
            root.fork(),ctx
        );
        self.childs.0.1.with_widget(
            &mut AsWidgetClosure::<'_,_,R,_,E>::new(|widget,root,ctx| {
                widget.render(
                    &FixedIdx(1usize) path,
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
    
    fn _event_direct(
        &self,
        path: &mut NewPathStack,
        stack: &P,
        event: &(dyn event_new::EventDyn<E>+'_),
        mut route_to_widget: Option<PathSliceRef>,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
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
                if let Some(_) = ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,path) {
                    let cursor = ctx.state().cursor_pos().expect("TODO");
                    let mut cx = cursor.par(o);
                    let (mut wx0, ww) = current.bounds.par(o);
                    let mut wx1 = wx0 + ww as i32;

                    let l_min = self.childs.0.0.with_widget(
                        &mut AsWidgetClosure::<'_,_,L,_,E>::new(|widget,root,ctx| {
                            widget.size(&FixedIdx(0usize) path, &stack, &mut cache.child_caches.0, root,ctx) //TODO It can't be! We can't already have the bounds for the widget when constraining
                        }),
                        root.fork(),ctx
                    ).par(o).min();
                    let r_min = self.childs.0.1.with_widget(
                        &mut AsWidgetClosure::<'_,_,R,_,E>::new(|widget,root,ctx| {
                            widget.size(&FixedIdx(1usize) path, &stack, &mut cache.child_caches.1, root,ctx)
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
                    &mut AsWidgetClosure::<'_,_,L,_,E>::new(|widget,root,ctx| {
                        let stack = WithCurrentBounds {
                            inner: &stack,
                            bounds: current.bounds & &bounds[0],
                            viewport: current.viewport.clone(),
                        };

                        passed |= widget.event_direct(
                            &FixedIdx(0) path,
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
                    &mut AsWidgetClosure::<'_,_,R,_,E>::new(|widget,root,ctx| {
                        let stack = WithCurrentBounds {
                            inner: &stack,
                            bounds: current.bounds & &bounds[2],
                            viewport: current.viewport.clone(),
                        };

                        passed |= widget.event_direct(
                            &FixedIdx(1) path,
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

    fn _size(
        &self,
        path: &mut NewPathStack,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> {
        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| {
                let mut s = ESize::<E>::add_base(self.orientation);

                self.childs.0.0.with_widget(
                    &mut AsWidgetClosure::<'_,_,L,_,E>::new(|widget,root,ctx| {
                        s.add( &widget.size(&FixedIdx(0usize) path, &stack, &mut cache.child_caches.0, root,ctx), self.orientation )
                    }),
                    root.fork(),ctx
                );
                s.add_space(self.width,self.orientation);
                self.childs.0.1.with_widget(
                    &mut AsWidgetClosure::<'_,_,R,_,E>::new(|widget,root,ctx| {
                        s.add( &widget.size(&FixedIdx(1usize) path, &stack, &mut cache.child_caches.1, root,ctx), self.orientation )
                    }),
                    root,ctx
                );

                s
            }
        );

        size
    }

    // fn child_bounds<P,Ph>(&self, path: &mut NewPathStack,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> {
    //     Ok(self.calc_bounds(b,self.state.get(ctx)).into())
    // }
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
        F: for<'www,'ww,'c,'cc> FnMut(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> Ret
    {
        self.childs.by_index(
            i,
            AsWidgetsOnWithChild(
                Some(callback),
                PhantomData
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
            AsWidgetsOnWithResolveChild(
                Some(callback),
                PhantomData
            ),
            root,ctx,
        )
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
        let rootf = root.fork();

        self.childs.by_index(
            idx,
            AsWidgetsOnTabulate(
                path, stack, op, dir, PhantomData
            ),
            rootf,ctx
        ).expect("TODO")
    }

    fn focusable(&self) -> bool {
        false
    }

    #[inline]
    fn respond_query<'a>(&'a self, mut r: crate::traitcast::WQueryResponder<'_,'a,E>) {
        r.try_respond::<dyn AtomState<E,f32>>(#[inline] || &self.state);
    }
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
    type Widget<'v,'z> = Self where 'z: 'v, Self: 'z;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,Ret>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,Ret,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Ret
    where
        Self: 'w
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct SplitPaneCache<LCache,RCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LCache: RenderCache<E>, RCache: RenderCache<E> {
    child_caches: (LCache,RCache),
    std_render_cachors: Option<StdRenderCachors<E>>,
    center_start_cachor: Option<Bounds>,
    _p: PhantomData<E>,
    //TODO cachor borders and colors
}

impl<LCache,RCache,E> RenderCache<E> for SplitPaneCache<LCache,RCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LCache: RenderCache<E>, RCache: RenderCache<E> {
    fn reset_current(&mut self) {
        self.child_caches.0.reset_current();
        self.child_caches.1.reset_current();
    }
}
