use crate::dispatchor::AsWidgetClosure;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::style::standard::cursor::StdCursor;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::QueryCurrentBounds;

use super::*;
use imp::ICheckBox;

impl<'w,E,State,Text,TrMut> Widget<E> for CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    State: AtomState<E,bool>,
    for<'a> Text: AsWidget<'a,E>,
    TrMut: TriggerMut<E>,
{
    fn child_paths(&self, _: E::WidgetPath, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        let render_props = StdRenderProps::new(stack)
            .inside_spacing_border();

        if ctx.state().is_hovered(&self.id) {
            renderer.set_cursor_specific(&StdCursor::Hand.into(),ctx);
        }

        let size = render_props.absolute_bounds.size.h;
        {
            let rect = Bounds::from_wh(size,size);
            let mut render_props = render_props.slice(&rect);

            renderer.fill_rect(
                &render_props
                .with_style_color_type(TestStyleColorType::Fg),
                ctx
            );

            renderer.fill_rect(
                &render_props
                    .inside_border_of_type_mul(TestStyleBorderType::Component,3)
                    .with_style_color_type(TestStyleColorType::Fg)
                    .with_vartype(
                        ctx.state().is_hovered(&self.id),
                        ctx.state().is_focused(&self.id),
                        self.state.get(ctx),
                        false, //TODO
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
                        false, //self.pressed(ctx).is_some(),
                        self.locked,
                    ),
                ctx
            );
        }
        {
            let text_border = Border::new(size+4/*TODO fix border impl*/*2,0,0,0);

            self.text.with_widget(
                AsWidgetClosure::new(|widget: &<Text as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| {
                    widget.render(
                        &render_props
                            .inside_border(text_border)
                            .with_vartype(
                                ctx.state().is_hovered(&self.id),
                                ctx.state().is_focused(&self.id),
                                false, //self.pressed(ctx).is_some(), TODO this wasn't set previously
                                self.locked,
                            ),
                        renderer,
                        root,ctx
                    )
                }),
                root,ctx
            );
        }
    }

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        if !event_mode.receive_self {return false;}

        if let Some(ee) = event.query_variant::<MouseUp<E>,_>(&stack) {
            if ee.key == MatchKeyCode::MouseLeft && ee.down_widget.is(self.id()) && ctx.state().is_hovered(&self.id) && !self.locked { //TODO down_widgets checks are redundand as event is sent?
                let new = !self.state.get(ctx);
                self.set(new,ctx);
                return true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>,_>(&stack) {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && ee.down_widget.is(self.id()) {
                let new = !self.state.get(ctx);
                self.set(new,ctx);
                return true;
            }
        }
        event.query_variant::<MouseDown<E>,_>(&stack).is_some() //TODO tf, also what is this EventResp? useless?
    }

    fn _size<P>(
        &self,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        let check_size = QueryCurrentBounds.query_in(stack).unwrap().bounds.size.h;
        let text_border = Border::new(check_size+4/*TODO fix border impl*/*2,0,0,0);

        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| widget_size_inside_border(
                stack, text_border,
                |stack|
                    self.text.with_widget(AsWidgetClosure::new(
                        |widget: &<Text as AsWidget<E>>::Widget<'_>,root,ctx: &mut E::Context<'_>| widget.size(&stack,root,ctx)
                    ),root,ctx)
            )
        );

        size
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
        dyn ICheckBox<E> => |s| s;
        dyn AtomState<E,bool> => |s| &s.state;
    );
}

impl<'z,E,State,Text,TrMut> AsWidget<'z,E> for CheckBox<'z,E,State,Text,TrMut> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}
