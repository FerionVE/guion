use std::marker::PhantomData;
use std::ops::Range;

use crate::aliases::{ERenderer, EEvent, EPressedKey, ESize, EStyle};
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::{MouseUp, KbdPress, MouseEnter, MouseLeave, MouseDown, KbdDown, KbdUp};
use crate::invalidation::Invalidation;
use crate::layout::Gonstraints;
use crate::root::RootRef;
use crate::traitcast::WQueryResponder;
use crate::util::bounds::Dims;
use crate::util::tabulate::{TabulateResponse, TabulateDirection, TabulateOrigin};
use crate::widget::cache::StdRenderCachors;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::id::WidgetID;
use crate::{event_new, EventResp};
use crate::newpath::{PathStack, PathResolvusDyn, SimpleId, PathStackDyn, FwdCompareStat, PathFragment, PathResolvus};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, TestStyleBorderType, with_inside_spacing_border, widget_size_inside_border_type, TestStyleCurrent, TestStyleVariant};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::style::standard::cursor::StdCursor;
use crate::widget_decl::mutor_trait::MutorEndBuilder;
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

    fn _render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let render_props = StdRenderProps::new(stack);

        force_render |= self.rendered_dims != Some(render_props.absolute_bounds.size);

        let mut need_render = force_render;

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

        let render_props = render_props.inside_spacing_border();

        let vartypes = TestStyleVariant {
            hovered: ctx.state().is_hovered(path._erase()),
            selected: ctx.state().is_focused(path._erase()),
            pressed: self.pressed(path._erase(),ctx).is_some(),
            disabled: self.locked,
            ..Default::default()
        };

        if vartypes.hovered {
            renderer.set_cursor_specific(&StdCursor::Hand.into(),ctx);
        }

        let fill_inner_color = &render_props
            .with_style_color_type(TestStyleColorType::Fg)
            .with_style_type(vartypes);

        if need_render {
            // renderer.fill_rect(
            //     &fill_inner_color,
            //     ctx
            // );
            renderer.fill_border_inner(
                &render_props
                    .with_style_border_type(TestStyleBorderType::Component)
                    .with_style_color_type(TestStyleColorType::Border)
                    .with_style_type(vartypes),
                ctx
            );
        }

        let render_props = render_props
            .inside_border_of_type(TestStyleBorderType::Component)
            .fork_with(|p| p.style.bg_color = fill_inner_color.style.current_color() )
            .with_style_type(vartypes);

        self.text.render(
            &SimpleId(ButtonChild).push_on_stack(path), &render_props,
            renderer,
            force_render,
            cache,
            root,ctx
        );

        self.rendered_dims = Some(render_props.absolute_bounds.size);
    }

    fn _event_direct<P,Ph,Evt>(
        &mut self,
        path: &Ph,
        stack: &P,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self && route_to_widget.map_or(true, |i| i.inner().is_none() );

        let mut vali = Invalidation::valid();

        if !receive_self {return vali;}

        if 
            event.query_variant::<MouseEnter>(path,&stack).is_some() | event.query_variant::<MouseLeave>(path,&stack).is_some() |
            event.query_variant::<MouseDown<E>>(path,&stack).is_some() | event.query_variant::<KbdDown<E>>(path,&stack).is_some() |
            event.query_variant::<KbdUp<E>>(path,&stack).is_some()
        {
            vali.render = true;
        } else if let Some(ee) = event.query_variant::<MouseUp<E>>(path,&stack) {
            if ee.key == MatchKeyCode::MouseLeft && path.fwd_compare(&*ee.down_widget) == FwdCompareStat::Equal && ctx.state().is_hovered(path._erase()) && !self.locked {
                self.trigger(path._erase(), root, ctx);
                vali.render = true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>>(path,&stack) {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && path.fwd_compare(&*ee.down_widget) == FwdCompareStat::Equal {
                self.trigger(path._erase(), root, ctx);
            }
        }
        
        if vali.render {
            self.rendered_dims = None;
        }

        self.invalidate_recursive(vali);
        
        vali
    }

    fn _size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| widget_size_inside_border_type(
                stack, TestStyleBorderType::Component,
                |stack|
                    self.text.size(&SimpleId(ButtonChild).push_on_stack(path), &stack, root,ctx)
            )
        );

        size.max( &self.size )
    }

    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: crate::widget_decl::route::UpdateRoute<'_,E>,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        self.text.update(&SimpleId(ButtonChild).push_on_stack(path), route.for_child_1(), root, ctx)
    }

    fn childs(&self) -> Range<isize> {
        0..1
    }
    
    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        if idx != 0 { return None; }

        Some(WidgetChildDynResult {
            idx,
            widget_id: self.text.id(),
            widget: &self.text,
        })
    }

    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        if idx != 0 { return None; }

        Some(WidgetChildDynResultMut {
            idx,
            widget_id: self.text.id(),
            widget: &mut self.text,
        })
    }

    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        if range.start <= 0 && range.end >= 1 {
            (callback)(WidgetChildDynResult {
                idx: 0,
                widget_id: self.text.id(),
                widget: &self.text,
            })
        }
    }

    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        if range.start <= 0 && range.end >= 1 {
            (callback)(WidgetChildDynResultMut {
                idx: 0,
                widget_id: self.text.id(),
                widget: &mut self.text,
            })
        }
    }

    fn resolve_child_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        path.try_fragment::<SimpleId<ButtonChild>>().map(|_|
            WidgetChildResolveDynResult {
                idx: 0,
                widget_id: self.text.id(),
                widget: &self.text,
                sub_path: path.inner().unwrap(),
            }
        )
    }

    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        path.try_fragment::<SimpleId<ButtonChild>>().map(|_|
            WidgetChildResolveDynResultMut {
                idx: 0,
                widget_id: self.text.id(),
                widget: &mut self.text,
                sub_path: path.inner().unwrap(),
            }
        )
    }

    fn send_mutation<Ph>(
        &mut self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn std::any::Any,
        root: <E as Env>::RootRef<'_>,
        ctx: &mut <E as Env>::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        //todo!()
    }

    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     todo!();
    //     Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    // }
    fn focusable(&self) -> bool { true }

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        idx: isize,
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

        self.text._tabulate(&SimpleId(ButtonChild).push_on_stack(path), stack, op.clone(), dir, root, ctx)
    }

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
    fn end<Ph>(
        &mut self,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        self.text.end(&SimpleId(ButtonChild).push_on_stack(path), root, ctx)
    }
}

impl<E,S,Tr> Button<E,S,Tr> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    S: Widget<E>,
    Tr: Trigger<E>,
{
    pub fn pressed<'l:'s,'cc: 'l,'s>(&self, path: &(dyn PathStackDyn<E>+'_), ctx: &'l mut E::Context<'cc>) -> Option<&'s EPressedKey<'cc,E>> {
        ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,path)
            .or_else(||
                ctx.state().is_pressed_and_id(MatchKeyCode::KbdReturn,path)
            )
            .or_else(||
                ctx.state().is_pressed_and_id(MatchKeyCode::KbdSpace,path)
            )
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct ButtonChild;
