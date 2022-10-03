use crate::dispatchor::AsWidgetClosure;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::widget::cache::{StdRenderCachors, WidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::{for_child_widget, QueryCurrentBounds, WithCurrentBounds};

use super::*;
use util::{state::*};

impl<'w,E,W,Scroll,MutFn> Widget<E> for Area<'w,E,W,Scroll,MutFn> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    W: AsWidget<'w,E>,
    Scroll: AtomState<E,ScrollOff>,
    MutFn: TriggerMut<E>,
{
    type Cache = AreaCache<W::WidgetCache,E>;

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
                        ctx.state().is_focused(&self.id),
                        false, //self.pressed(ctx).is_some(),
                        false, //self.locked,
                    ),
                ctx
            );
        }

        let render_props = render_props.inside_border_of_type(TestStyleBorderType::Component);

        let rect = render_props.absolute_bounds;

        let inner_size = self.inner.with_widget(
            AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                widget.size(&for_child_widget(&render_props,widget), &mut cache.inner_cache, root,ctx)
            }),
            root.fork(),ctx
        );

        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let (sx,sy) = normalize_scroll_off((sx,sy), (iw,ih).into(), rect.size,true);

        let inner_rect = Bounds::from_xywh(rect.x()-sx as i32, rect.y()-sy as i32, iw, ih);

        self.inner.with_widget(
            AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                let render_props = render_props
                    .fork_with(|r| {
                        r.absolute_bounds = inner_rect;
                        r.absolute_viewport = rect;
                    });

                widget.render(
                    &for_child_widget(render_props,widget),
                    renderer,
                    force_render, &mut cache.inner_cache,
                    root,ctx
                )
            }),
            root,ctx
        );
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
        let stack = with_inside_border_by_type(stack,TestStyleBorderType::Component);
        
        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        if !event_mode.route_to_childs && !event_mode.receive_self {return false;}
        
        let rect = bounds.bounds;

        let (osx,osy) = self.scroll.get(ctx);

        let inner_size = self.inner.with_widget(
            AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                widget.size(&for_child_widget(&stack,widget), &mut cache.inner_cache, root,ctx)
            }),
            root.fork(),ctx
        );

        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let (sx,sy) = normalize_scroll_off((osx,osy), (iw,ih).into(), rect.size,true);

        let inner_rect = Bounds::from_xywh(rect.x()-sx as i32, rect.y()-sy as i32, iw, ih);

        let mut passed = false;

        if event_mode.route_to_childs {
            // let mut l = l.for_child(0).unwrap();
            // let e = e.with_bounds(inner_rect);
            // if let Some(ee) = e.filter(&l) { //TODO API OOF not filtering breaks for_child mechanism
            //     passed |= l.event_direct(&ee);
            // }

            self.inner.with_widget(
                AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    let stack = WithCurrentBounds {
                        inner: for_child_widget(&stack,widget),
                        bounds: inner_rect,
                        viewport: *rect,
                    };
        
                    passed |= widget.event_direct(&stack, event, &mut cache.inner_cache, root,ctx);
                }),
                root.fork(),ctx
            )
        }

        if !passed && event_mode.receive_self { //TODO passed stack doof
            if let Some(ee) = event.query_variant::<KbdPress<E>,_>(&stack) {
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
                        if let Some(t) = self.scroll_updater.boxed(su) {
                            ctx.mutate_closure(t);
                            passed = true;
                        }
                    }
                }
            }
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
        self.size.clone()
    }

    fn childs(&self) -> usize {
        1
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
        //if i != 0 {return Err(());} //TODO fix callback
        self.inner.with_widget(
            AsWidgetClosure::new(|widget: &<W as AsWidget<E>>::Widget<'_>,_,ctx: &mut E::Context<'_>|
                (callback)(Ok(widget.erase()),ctx)
            ),
            root,ctx
        )
    }
    
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        todo!() // TODO complete inner bounds or just view
    }
    fn focusable(&self) -> bool {
        false //TODO
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn AtomState<E,ScrollOff> => |s| &s.scroll;
    );
}

impl<'z,E,W,Scroll,MutFn> AsWidget<'z,E> for Area<'z,E,W,Scroll,MutFn> where Self: Widget<E>, E: Env {
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
