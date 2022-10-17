use std::marker::PhantomData;

use crate::aliases::{ERenderer, EEvent, ESize};
use crate::dispatchor::{AsWidgetClosure, AsWidgetDispatch};
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::{MouseDown, KbdPress, MouseUp};
use crate::layout::Gonstraints;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::widget::cache::{StdRenderCachors, WidgetCache};
use crate::{impl_traitcast, EventResp, event_new};
use crate::newpath::{PathStack, PathResolvusDyn, FwdCompareStat, SimpleId, PathResolvus, PathFragment};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, TestStyleBorderType, with_inside_spacing_border, widget_size_inside_border_type, widget_size_inside_border};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::style::standard::cursor::StdCursor;
use crate::util::border::Border;
use crate::util::bounds::Bounds;
use crate::util::tabulate::{TabulateDirection, TabulateOrigin, TabulateResponse};
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widget::stack::QueryCurrentBounds;
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::as_widget::AsWidget;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widgets::util::state::AtomState;

use super::CheckBox;
use super::imp::ICheckBox;

impl<'w,E,State,Text,TrMut> Widget<E> for CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    State: AtomState<E,bool>,
    Text: AsWidget<E>,
    TrMut: MutorEndBuilder<bool,E>,
{
    type Cache = CheckBoxCache<Text::WidgetCache,E>;

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

        let vartypes = (
            ctx.state().is_hovered(path._erase()),
            ctx.state().is_focused(path._erase()),
            self.state.get(ctx),
            false, //TODO
        );

        let (hovered,selected,activated,disabled) = vartypes;

        if cache.vartype_cachors != Some(vartypes) {
            need_render = true;
            cache.vartype_cachors = Some(vartypes);
        }

        if ctx.state().is_hovered(path._erase()) {
            renderer.set_cursor_specific(&StdCursor::Hand.into(),ctx);
        }

        let size = render_props.absolute_bounds.size.h;

        if need_render {
            renderer.fill_rect(
                &render_props
                .slice(Bounds::from_wh(size+4/*TODO fix border impl*/*2,size))
                .with_style_color_type(TestStyleColorType::Bg),
                ctx
            );

            let rect = Bounds::from_wh(size,size);
            let render_props = render_props.slice(&rect);

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
                        hovered,
                        selected,
                        activated,
                        disabled,
                    ),
                ctx
            );

            renderer.fill_border_inner(
                &render_props
                    .with_style_border_type(TestStyleBorderType::Component)
                    .with_style_color_type(TestStyleColorType::Border)
                    .with_vartype(
                        hovered,
                        selected,
                        activated,
                        disabled,
                    ),
                ctx
            );
        }
        {
            let text_border = Border::new(size+4/*TODO fix border impl*/*2,0,0,0);

            self.text.with_widget(
                &mut AsWidgetClosure::new(|widget: &<Text as AsWidget<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>| {
                    widget.render(
                        &SimpleId(CheckBoxChild).push_on_stack(path),
                        &render_props
                            .inside_border(text_border)
                            .with_vartype(
                                hovered,
                                selected,
                                false, //self.pressed(ctx).is_some(), TODO this wasn't set previously
                                disabled,
                            ),
                        renderer,
                        force_render,
                        &mut cache.label_cache,
                        root,ctx
                    )
                }),
                root,ctx
            );
        }
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
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self && route_to_widget.map_or(true, |i| i.inner().is_none() );

        if !receive_self {return false;}

        if let Some(ee) = event.query_variant::<MouseUp<E>,_,_>(path,&stack) {
            if ee.key == MatchKeyCode::MouseLeft && path.fwd_compare(&*ee.down_widget) == FwdCompareStat::Equal && ctx.state().is_hovered(path._erase()) && !self.locked { //TODO down_widgets checks are redundand as event is sent?
                let new = !self.state.get(ctx);
                self.set(new,ctx);
                return true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>,_,_>(path,&stack) {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && path.fwd_compare(&*ee.down_widget) == FwdCompareStat::Equal {
                let new = !self.state.get(ctx);
                self.set(new,ctx);
                return true;
            }
        }
        event.query_variant::<MouseDown<E>,_,_>(path,&stack).is_some() //TODO tf, also what is this EventResp? useless?
    }

    fn _size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let check_size = QueryCurrentBounds.query_in(stack).unwrap().bounds.size.h;
        let text_border = Border::new(check_size+4/*TODO fix border impl*/*2,0,0,0);

        let mut size = widget_size_inside_border_type(
            stack, TestStyleBorderType::Spacing,
            |stack| widget_size_inside_border(
                stack, text_border,
                |stack|
                    self.text.with_widget(&mut AsWidgetClosure::new(|widget: &<Text as AsWidget<E>>::Widget<'_,'_>,root,ctx: &mut E::Context<'_>|
                        widget.size(&SimpleId(CheckBoxChild).push_on_stack(path), &stack, &mut cache.label_cache, root,ctx)
                    ),root,ctx)
            )
        );

        size.add_x( &self.size );

        size
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
        
        self.text.with_widget(
            &mut AsWidgetClosure::new(|widget: &<Text as AsWidget<E>>::Widget<'_,'_>,_,ctx: &mut E::Context<'_>|
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
        if sub_path.try_fragment::<SimpleId<CheckBoxChild>>().is_some() {
            self.text.with_widget(
                &mut AsWidgetClosure::new(move |widget: &<Text as AsWidget<E>>::Widget<'_,'_>,_,ctx: &mut E::Context<'_>|
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

        self.text.with_widget(
            &mut AsWidgetClosure::new(move |widget: &<Text as AsWidget<E>>::Widget<'_,'_>,_,ctx: &mut E::Context<'_>|
                widget._tabulate(&SimpleId(CheckBoxChild).push_on_stack(path), stack, op.clone(), dir, root.fork(), ctx)
            ),
            rootf,ctx
        )
    }
    
    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     todo!();
    //     Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    // }
    
    fn focusable(&self) -> bool { true }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn ICheckBox<E> => |s| s;
        dyn AtomState<E,bool> => |s| &s.state;
    );
}

impl<E,State,Text,TrMut> AsWidget<E> for CheckBox<'_,E,State,Text,TrMut> where Self: Widget<E>, E: Env {
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
pub struct CheckBoxCache<LabelCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LabelCache: WidgetCache<E> {
    label_cache: LabelCache,
    std_render_cachors: Option<StdRenderCachors<E>>,
    vartype_cachors: Option<(bool,bool,bool,bool)>,
    _p: PhantomData<E>,
    //TODO cachor borders and colors
}

impl<LabelCache,E> WidgetCache<E> for CheckBoxCache<LabelCache,E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, LabelCache: WidgetCache<E> {
    fn reset_current(&mut self) {
        self.label_cache.reset_current()
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct CheckBoxChild;
