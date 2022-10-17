use crate::aliases::{ERenderer, EEvent, ESize};
use crate::ctx::Context;
use crate::ctx::clipboard::CtxClipboardAccess;
use crate::dispatchor::{AsWidgetClosure, AsWidgetDispatch};
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::KbdPress;
use crate::layout::{Gonstraints, GonstraintAxis};
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::util::bounds::Bounds;
use crate::util::tabulate::{TabulateResponse, TabulateDirection, TabulateOrigin};
use crate::widget::cache::{StdRenderCachors, WidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widgets::util::state::AtomState;
use crate::{event_new, impl_traitcast, EventResp};
use crate::newpath::{PathStack, SimpleId, PathResolvusDyn, PathResolvus, PathFragment};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, with_inside_spacing_border, with_inside_border_by_type, TestStyleBorderType};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::util::{ScrollOff, normalize_scroll_off};
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::as_widget::AsWidget;
use crate::widget::stack::{QueryCurrentBounds, WithCurrentBounds};
use crate::widgets::soft_single_child_resolve_check;

use super::{Area, ScrollUpdate};

impl<'w,E,W,Scroll,MutFn> Widget<E> for Area<'w,E,W,Scroll,MutFn> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    W: AsWidget<E>,
    Scroll: AtomState<E,ScrollOff>,
    MutFn: MutorEndBuilder<ScrollUpdate,E>,
{
    type Cache = AreaCache<W::WidgetCache,E>;

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

        let (sx,sy) = self.scroll.get(ctx);

        if cache.scroll_cachor != Some(((sx,sy),self.negative_scroll)) {
            need_render = true;
            force_render = true;
            cache.scroll_cachor = Some(((sx,sy),self.negative_scroll));
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

        if need_render {
            renderer.fill_border_inner(
                &render_props
                    .with_style_border_type(TestStyleBorderType::Component)
                    .with_style_color_type(TestStyleColorType::Border)
                    .with_vartype(
                        false, //ctx.state().is_hovered(&self.id),
                        ctx.state().is_focused(path._erase()),
                        false, //self.pressed(ctx).is_some(),
                        false, //self.locked,
                    ),
                ctx
            );
        }

        let render_props = render_props.inside_border_of_type(TestStyleBorderType::Component);

        let rect = render_props.absolute_bounds;

        let inner_size = self.inner.with_widget(
            &mut AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                widget.size(&SimpleId(AreaChild).push_on_stack(path), &render_props, &mut cache.inner_cache, root,ctx)
            }),
            root.fork(),ctx
        );

        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let (sx,sy) = normalize_scroll_off((sx,sy), (iw,ih).into(), rect.size,true);

        let inner_rect = Bounds::from_xywh(rect.x()-sx as i32, rect.y()-sy as i32, iw, ih);

        self.inner.with_widget(
            &mut AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                let render_props = render_props
                    .fork_with(|r| {
                        r.absolute_bounds = inner_rect;
                        r.absolute_viewport = rect;
                    });

                widget.render(
                    &SimpleId(AreaChild).push_on_stack(path), &render_props,
                    renderer,
                    force_render, &mut cache.inner_cache,
                    root,ctx
                )
            }),
            root,ctx
        );
    }

    fn _event_direct<P,Ph,Evt>(
        &self,
        path: &Ph,
        stack: &P,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);
        let stack = with_inside_border_by_type(stack,TestStyleBorderType::Component);
        
        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self && route_to_widget.map_or(true, |i| i.inner().is_none() );

        if !event_mode.route_to_childs && !receive_self {return false;}
        
        let rect = bounds.bounds;

        let (osx,osy) = self.scroll.get(ctx);

        let inner_size = self.inner.with_widget(
            &mut AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                widget.size(&SimpleId(AreaChild).push_on_stack(path), &stack, &mut cache.inner_cache, root,ctx)
            }),
            root.fork(),ctx
        );

        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let (sx,sy) = normalize_scroll_off((osx,osy), (iw,ih).into(), rect.size,true);

        let inner_rect = Bounds::from_xywh(rect.x()-sx as i32, rect.y()-sy as i32, iw, ih);

        let mut passed = false;

        if event_mode.route_to_childs {
            if soft_single_child_resolve_check(route_to_widget.clone(),SimpleId(AreaChild)) {
                self.inner.with_widget(
                    &mut AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                        let stack = WithCurrentBounds {
                            inner: &stack,
                            bounds: inner_rect,
                            viewport: *rect,
                        };
            
                        passed |= widget.event_direct(&SimpleId(AreaChild).push_on_stack(path), &stack, event, route_to_widget.and_then(PathResolvus::inner), &mut cache.inner_cache, root,ctx);
                    }),
                    root.fork(),ctx
                )
            }
        }

        if !passed && event_mode.receive_self { //TODO passed stack doof
            if let Some(ee) = event.query_variant::<KbdPress<E>,_,_>(path,&stack) {
                if
                    ee.key == MatchKeyCode::KbdUp || ee.key == MatchKeyCode::KbdDown ||
                    ee.key == MatchKeyCode::KbdLeft || ee.key == MatchKeyCode::KbdRight
                {
                    let (mut nx,mut ny) = (sx,sy);

                    if ee.key == MatchKeyCode::KbdUp {
                        ny = ny.saturating_sub(4);
                    }
                    if ee.key == MatchKeyCode::KbdDown {
                        ny += 4;
                    }
                    if ee.key == MatchKeyCode::KbdLeft {
                        nx = nx.saturating_sub(4);
                    }
                    if ee.key == MatchKeyCode::KbdRight {
                        nx += 4;
                    }

                    let (nx,ny) = normalize_scroll_off((nx,ny), (iw,ih).into(), rect.size,true);

                    let su = ScrollUpdate{offset:(nx-osx,ny-osy)};

                    if su.offset != (0,0) {
                        if let Some(t) = self.scroll_updater.build_box_mut_event(su) {
                            ctx.mutate_closure(t);
                            passed = true;
                        }
                    }
                }
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
        self.size.clone()
    }

    fn childs(&self) -> usize {
        1
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
        if i != 0 { return (callback)(Err(()),ctx); }
        
        self.inner.with_widget(
            &mut AsWidgetClosure::new(move |widget: &<W as AsWidget<E>>::Widget<'_,'_>,_,ctx: &mut E::Context<'_>|
                (callback)(Ok(widget.erase()),ctx)
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
        if sub_path.try_fragment::<SimpleId<AreaChild>>().is_some() {
            self.inner.with_widget(
                &mut AsWidgetClosure::new(move |widget: &<W as AsWidget<E>>::Widget<'_,'_>,_,ctx: &mut E::Context<'_>|
                    (callback)(
                        Ok(WidgetWithResolveChildDyn {
                            idx: 0,
                            sub_path: sub_path.inner().unwrap(),
                            widget: widget.erase(),
                        }),
                        ctx,
                    )
                ),
                root,ctx
            )
        } else {
            (callback)(Err(todo!()),ctx)
        }
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
        if idx != 0 { return Err(todo!()); }

        let rootf = root.fork();

        self.inner.with_widget(
            &mut AsWidgetClosure::new(move |widget: &<W as AsWidget<E>>::Widget<'_,'_>,_,ctx: &mut E::Context<'_>|
                widget._tabulate(&SimpleId(AreaChild).push_on_stack(path), stack, op.clone(), dir, root.fork(), ctx)
            ),
            rootf,ctx
        )
    }
    
    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     todo!() // TODO complete inner bounds or just view
    // }
    fn focusable(&self) -> bool {
        false //TODO
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn AtomState<E,ScrollOff> => |s| &s.scroll;
    );
}

impl<E,W,Scroll,MutFn> AsWidget<E> for Area<'_,E,W,Scroll,MutFn> where Self: Widget<E>, E: Env {
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

#[derive(Default)]
pub struct AreaCache<InnerCache,E> where E: Env, InnerCache: WidgetCache<E> {
    inner_cache: InnerCache,
    scroll_cachor: Option<(ScrollOff,bool)>,
    std_render_cachors: Option<StdRenderCachors<E>>,
    //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
}

impl<InnerCache,E> WidgetCache<E> for AreaCache<InnerCache,E> where E: Env, InnerCache: WidgetCache<E> {
    fn reset_current(&mut self) {
        self.inner_cache.reset_current()
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct AreaChild;
