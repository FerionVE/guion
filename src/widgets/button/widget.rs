use crate::dispatchor::AsWidgetClosure;
use crate::event_new::filter::QueryStdEventMode;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::style::standard::cursor::StdCursor;
use crate::widget::cache::{WidgetCache, StdRenderCachors};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::for_child_widget;

use super::*;
use super::imp::*;

impl<'w,E,Text,Tr,TrMut> Widget<E> for Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    Text: AsWidget<'w,E>,
    Tr: Trigger<E>,
    TrMut: TriggerMut<E>,
{
    type Cache = ButtonCache<Text::WidgetCache,E>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        need_render |= StdRenderCachors::current(stack).validate(&mut cache.std_render_cachors);
        
        let render_props = StdRenderProps::new(stack)
            .inside_spacing_border();

        let vartypes = (
            ctx.state().is_hovered(&self.id),
            ctx.state().is_focused(&self.id),
            self.pressed(ctx).is_some(),
            self.locked,
        );

        let (hovered,selected,activated,disabled) = vartypes;

        if cache.vartype_cachors != Some(vartypes) {
            need_render = true;
            cache.vartype_cachors = Some(vartypes);
        }

        if hovered {
            renderer.set_cursor_specific(&StdCursor::Hand.into(),ctx);
        }

        if need_render {
            renderer.fill_rect(
                &render_props
                    .with_style_color_type(TestStyleColorType::Fg)
                    .with_vartype(
                        ctx.state().is_hovered(&self.id),
                        ctx.state().is_focused(&self.id),
                        self.pressed(ctx).is_some(),
                        self.locked,
                    ),
                ctx
            );
            renderer.fill_border_inner(
                &render_props
                    .with_style_border_type(TestStyleBorderType::Component)
                    .with_style_color_type(TestStyleColorType::Border)
                    .with_vartype(
                        ctx.state().is_hovered(&self.id),
                        ctx.state().is_focused(&self.id),
                        self.pressed(ctx).is_some(),
                        self.locked,
                    ),
                ctx
            );
        }

        self.text.with_widget(
            AsWidgetClosure::new(|widget: &<Text as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                let render_props = render_props
                    .inside_border_of_type(TestStyleBorderType::Component)
                    .with_vartype(
                        hovered, selected, activated, disabled
                    );

                widget.render(
                    &for_child_widget(render_props,widget),
                    renderer,
                    need_render,
                    &mut cache.label_cache,
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
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        if !event_mode.receive_self {return false;}

        if let Some(ee) = event.query_variant::<MouseUp<E>,_>(&stack) {
            if ee.key == MatchKeyCode::MouseLeft && ee.down_widget.is(self.id()) && ctx.state().is_hovered(&self.id) && !self.locked {
                self.trigger(root,ctx);
                return true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>,_>(&stack) {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && ee.down_widget.is(self.id()) {
                self.trigger(root,ctx);
                return true;
            }
        }
        false
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
            |stack| widget_size_inside_border_type(
                stack, TestStyleBorderType::Component,
                |stack|
                    self.text.with_widget(AsWidgetClosure::new(
                        |widget: &<Text as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>|
                            widget.size(&for_child_widget(&stack,widget), &mut cache.label_cache, root,ctx)
                    ),root,ctx)
            )
        );

        size.max( &self.size )
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
        self.text.with_widget(
            AsWidgetClosure::new(|widget: &<Text as AsWidget<E>>::Widget<'_>,_,ctx: &mut E::Context<'_>|
                (callback)(Ok(widget.erase()),ctx)
            ),
            root,ctx
        )
    }
    
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        todo!();
        Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    }
    fn focusable(&self) -> bool { true }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn IButton<E> => |s| s;
        dyn Trigger<E> => |s| &s.trigger;
    );
}

impl<'w,E,S,Tr,TrMut> Button<'w,E,S,Tr,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    S: AsWidget<'w,E>,
    Tr: Trigger<E>,
    TrMut: TriggerMut<E>,
{
    pub fn pressed<'l:'s,'cc: 'l,'s>(&self, ctx: &'l mut E::Context<'cc>) -> Option<&'s EPressedKey<'cc,E>> {
        ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,self.id.clone())
            .or_else(||
                ctx.state().is_pressed_and_id(MatchKeyCode::KbdReturn,self.id.clone())
            )
            .or_else(||
                ctx.state().is_pressed_and_id(MatchKeyCode::KbdSpace,self.id.clone())
            )
    }
}

impl<'z,E,Text,Tr,TrMut> AsWidget<'z,E> for Button<'z,E,Text,Tr,TrMut> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct ButtonCache<LabelCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LabelCache: WidgetCache<E> {
    label_cache: LabelCache,
    std_render_cachors: Option<StdRenderCachors<E>>,
    vartype_cachors: Option<(bool,bool,bool,bool)>,
    _p: PhantomData<E>,
    //TODO cachor borders and colors
}

impl<LabelCache,E> WidgetCache<E> for ButtonCache<LabelCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LabelCache: WidgetCache<E> {
    fn reset_current(&mut self) {}
}
