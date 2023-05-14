use std::marker::PhantomData;
use std::ops::Range;

use crate::aliases::{ERenderer, EEvent, EPressedKey, ESize, EStyle};
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::{MouseUp, KbdPress, MouseEnter, MouseLeave, MouseDown, KbdDown, KbdUp};
use crate::event_new::Event;
use crate::invalidation::Invalidation;
use crate::layout::Gonstraints;
use crate::pathslice::{NewPathStack, PathSliceRef, PathSliceMatch};
use crate::queron::dyn_tunnel::QueronDyn;
use crate::root::RootRef;
use crate::traitcast::WQueryResponder;
use crate::util::bounds::Dims;
use crate::util::tabulate::{TabulateResponse, TabulateDirection, TabulateOrigin};
use crate::widget::cache::StdRenderCachors;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::id::WidgetID;
use crate::{event_new, EventResp, widget_childs_macro};
use crate::newpath::{PathStack, PathResolvusDyn, SimpleId, PathStackDyn, FwdCompareStat, PathFragment, PathResolvus};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, TestStyleBorderType, with_inside_spacing_border, widget_size_inside_border_type, TestStyleCurrent, TestStyleVariant};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::style::standard::cursor::StdCursor;
use crate::widget::{Widget, WidgetChildDynResult, WidgetChildDynResultMut, WidgetChildResolveDynResult, WidgetChildResolveDynResultMut};

use super::decl::WQueryButtonRestore;
use super::imp::IButton;
use super::Trigger;

pub struct Button<E,Text,Tr> where
    E: Env,
{
    pub(super) id: WidgetID,
    pub(super) trigger: Tr,
    pub(super) size: ESize<E>,
    pub(super) style: EStyle<E>,
    pub(super) locked: bool,
    pub(super) rendered_dims: Option<Dims>,
    //pressed: Option<EEKey<E>>,
    pub(super) text: Text,
}

impl<E,Text,Tr> Widget<E> for Button<E,Text,Tr> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    Text: Widget<E>,
    Tr: Trigger<E> + Clone + 'static,
{
    type Cache = Text::Cache;

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

        force_render |= self.rendered_dims != Some(render_props.absolute_bounds.size);

        let mut need_render = true;

        // render_props.current_std_render_cachors()
        //     .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

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

        let inner_render_props = render_props.inside_spacing_border();

        let vartypes = TestStyleVariant {
            hovered: ctx.state().is_hovered(self.id),
            selected: ctx.state().is_focused(self.id),
            pressed: self.pressed(ctx).is_some(),
            disabled: self.locked,
            ..Default::default()
        };

        if vartypes.hovered {
            renderer.set_cursor_specific(&StdCursor::Hand.into(),ctx);
        }

        let fill_inner_color = &inner_render_props
            .with_style_color_type(TestStyleColorType::Fg)
            .with_style_type(vartypes);

        if need_render {
            // renderer.fill_rect(
            //     &fill_inner_color,
            //     ctx
            // );
            renderer.fill_border_inner(
                &inner_render_props
                    .with_style_border_type(TestStyleBorderType::Component)
                    .with_style_color_type(TestStyleColorType::Border)
                    .with_style_type(vartypes),
                ctx
            );
        }

        let inner_render_props = inner_render_props
            .inside_border_of_type(TestStyleBorderType::Component)
            .fork_with(|p| p.style.bg_color = fill_inner_color.style.current_color() )
            .with_style_type(vartypes);

        self.text.render(
            &mut path.with(ButtonChild), inner_render_props,
            renderer,
            force_render,
            cache,
            root,ctx
        );

        self.rendered_dims = Some(render_props.absolute_bounds.size);
    }

    fn _event_direct(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        event: &(dyn event_new::EventDyn<E>+'_),
        route_to_widget: Option<PathSliceRef>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        let stack = with_inside_spacing_border(stack);
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self && route_to_widget.map_or(true, |i| i.fetch().is_empty() );

        let mut vali = Invalidation::valid();

        if !receive_self {return vali;}

        if 
            event.query_variant::<MouseEnter>(path,&stack).is_some() | event.query_variant::<MouseLeave>(path,&stack).is_some() |
            event.query_variant::<MouseDown<E>>(path,&stack).is_some() | event.query_variant::<KbdDown<E>>(path,&stack).is_some() |
            event.query_variant::<KbdUp<E>>(path,&stack).is_some()
        {
            vali.render = true;
        } else if let Some(ee) = event.query_variant::<MouseUp<E>>(path,&stack) {
            if ee.key == MatchKeyCode::MouseLeft && ee.down_widget.1 == self.id && ctx.state().is_hovered(self.id) && !self.locked {
                self.trigger(path, root, ctx);
                vali.render = true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>>(path,&stack) {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && ee.down_widget.1 == self.id {
                self.trigger(path, root, ctx);
            }
        }
        
        if vali.render {
            self.rendered_dims = None;
        }

        self.invalidate_recursive(vali);
        
        vali
    }

    fn _size(
        &mut self,
        path: &mut NewPathStack,
        stack: &(dyn QueronDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> {
        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| widget_size_inside_border_type(
                stack, TestStyleBorderType::Component,
                |stack|
                    self.text.size(&mut path.with(SimpleId(ButtonChild)), &stack, root,ctx)
            )
        );

        size.max( &self.size )
    }

    fn update(
        &mut self,
        path: &mut NewPathStack,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation {
        self.text.update(&mut path.with(SimpleId(ButtonChild)), route.for_child_1::<SimpleId<ButtonChild>>(), root, ctx)
    }

    fn send_mutation(
        &mut self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) {
        //todo!()
    }

    // fn child_bounds<P,Ph>(&self, path: &mut NewPathStack,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> {
    //     todo!();
    //     Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    // }
    fn focusable(&self) -> bool { true }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        if vali.render {
            self.rendered_dims = None;
        }
        self.text.invalidate_recursive(vali);
    }

    #[inline]
    fn respond_query<'a>(&'a self, mut r: WQueryResponder<'_,'a,E>) {
        r.try_respond::<dyn IButton<E>>(#[inline] || self) ||
        r.try_respond::<dyn Trigger<E>>(#[inline] || &self.trigger);
    }

    #[inline]
    fn respond_query_mut<'a>(&'a mut self, mut responder: WQueryResponder<'_,'a,E>) {
        if let Some(h) = responder.try_downcast::<WQueryButtonRestore>() {
            *h = Some(self);
        }
    }

    #[inline]
    fn end(
        &mut self,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.text.end(&mut path.with(ButtonChild), root, ctx)
    }

    widget_childs_macro!(
        ButtonChild |i| ButtonChild;
        |s|
        0 =>  s.text;
    );
}

impl<E,S,Tr> Button<E,S,Tr> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    S: Widget<E>,
    Tr: Trigger<E>,
{
    pub fn pressed<'l:'s,'cc: 'l,'s>(&self, ctx: &'l mut E::Context<'cc>) -> Option<&'s EPressedKey<'cc,E>> {
        ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft, self.id)
            .or_else(||
                ctx.state().is_pressed_and_id(MatchKeyCode::KbdReturn, self.id)
            )
            .or_else(||
                ctx.state().is_pressed_and_id(MatchKeyCode::KbdSpace, self.id)
            )
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct ButtonChild;
