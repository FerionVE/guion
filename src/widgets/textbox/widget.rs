use std::any::Any;
use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;

use crate::aliases::{ETCurSel, EEvent, ETextLayout, ERenderer, ETCurSelCachor, ESize};
use crate::ctx::Context;
use crate::ctx::clipboard::CtxClipboardAccess;
use crate::dispatchor::AsWidgetDispatch;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::event::key::MatchKeyCode;
use crate::event::standard::variants::{TextInput, KbdPress, MouseScroll, MouseDown};
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::text::cursel::{Direction, TxtCurSelBytePos};
use crate::util::bounds::Offset;
use crate::widget::cache::{WidgetCache, StdRenderCachors};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::{event_new, impl_traitcast, EventResp};
use crate::newpath::{PathStack, PathResolvusDyn, PathResolvus};
use crate::queron::Queron;
use crate::render::{TestStyleColorType, StdRenderProps, TestStyleBorderType, QueryTestStyle, with_inside_spacing_border};
use crate::render::widgets::RenderStdWidgets;
use crate::state::{CtxStdState, StdState};
use crate::style::standard::cursor::StdCursor;
use crate::text::cursel::TxtCurSel;
use crate::text::layout::{TxtLayoutFromStor, TxtLayout};
use crate::text::stor::TextStor;
use crate::util::tabulate::{TabulateOrigin, TabulateDirection, TabulateResponse};
use crate::validation::Validation;
use crate::view::mutor_trait::MutorEndBuilder;
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::as_widget::AsWidget;
use crate::widget::stack::QueryCurrentBounds;
use crate::widgets::util::state::AtomState;

use super::{TextBox, TextBoxUpdate};
use super::imp::ITextBox;
use super::state::max_off;

impl<E,Text,Scroll,Curs,TBUpd> Widget<E> for TextBox<'_,E,Text,Scroll,Curs,TBUpd> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
    Scroll: AtomState<E,(u32,u32)>,
    Curs: AtomState<E,ETCurSel<E>>,
    TBUpd: MutorEndBuilder<TextBoxUpdate<E>,E>,
{
    type Cache = TextBoxCache<E>;
    
    fn _render<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        _root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        let render_props = StdRenderProps::new(stack);

        render_props.current_std_render_cachors()
            .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

        let render_props = render_props.inside_spacing_border();

        //TODO cachor align and style stuff e.g. bg color
        //TODO text layout cachors
        need_render |= self.glyphs(stack, cache, ctx);

        let g = cache.text_cache.as_ref().unwrap();
        //let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,r.bounds());
        let cursor = self.get_cursor(ctx);
        //cursor.fix_boundaries(&*g);
        let off: Offset = self.get_scroll(ctx).into();

        let selected = ctx.state().is_focused(path._erase());

        if cache.scroll_curs_cachor != Some((cursor.cachor(),off,selected)) {
            need_render = true;
            cache.scroll_curs_cachor = Some((cursor.cachor(),off,selected));
        }

        if ctx.state().is_hovered(path._erase()) {
            renderer.set_cursor_specific(&StdCursor::IBeam.into(),ctx);
        }

        if !need_render {return;}

        renderer.fill_rect(
            &render_props
                .with_style_color_type(TestStyleColorType::Bg),
            ctx
        );

        let render_props = render_props.inside_spacing_border();

        renderer.fill_border_inner(
            &render_props
                .with_style_border_type(TestStyleBorderType::Component)
                .with_style_color_type(TestStyleColorType::Border)
                .with_vartype(
                    false, //ctx.state().is_hovered(&self.id),
                    selected,
                    false, //self.pressed(ctx).is_some(),
                    false, //self.locked,
                ),
            ctx
        );
        let render_props = render_props.inside_border_of_type_mul(TestStyleBorderType::Component,2);

        for b in g.selection_bounds(cursor.clone()) {
            let b = b - off;

            renderer.fill_rect(
                &render_props
                    .slice(b)
                    .with_style_color_type(TestStyleColorType::Fg),
                ctx
            );
        }
        let mut b = g.cursor_bounds(cursor); //TODO fix as it should work if cursor is at end
        b.size.w = 2;
        //let b = Bounds::from_xywh(c.0 as i32, c.1 as i32 - s.glyphs.line_ascent() as i32, 2, s.glyphs.line_height());
        let b = b - off;
        renderer.fill_rect(
            &render_props
                .slice(b)
                .with_style_color_type(TestStyleColorType::Border), //Active
            ctx
        );

        renderer.render_preprocessed_text(
            &g,
            off,
            &render_props
                .with_style_color_type(TestStyleColorType::Fg),
            ctx
        );

        cache.text_rendered = true;
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

        let style = QueryTestStyle.query_in(&stack).unwrap();
        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(path,&stack).unwrap();

        let receive_self = event_mode.receive_self && route_to_widget.map_or(true, |i| i.inner().is_none() );

        if !receive_self {return false;}

        self.glyphs(&stack, cache, ctx);

        //e.0._debug_type_name();
        let g = cache.text_cache.as_ref().unwrap();

        let mut cursor = self.get_cursor(ctx);
        g.fix_cursor_boundaries(&mut cursor);

        let border = style.border_of_type(TestStyleBorderType::Component)*2;
        let b = bounds.bounds.inside_border(&border);

        let mut passed = false;

        if let Some(ee) = event.query_variant::<TextInput>(path,&stack)  {
            if !ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                let s = ee.text.clone();
                
                self.insert_text(&s,g,root.fork(),ctx);
                self.scroll_to_cursor(&b,g,root,ctx);

                passed = true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>>(path,&stack) {
            if
                ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdBackspace ||
                ee.key == MatchKeyCode::KbdLeft || ee.key == MatchKeyCode::KbdRight
            {
                let ctrl = ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some();
                
                if ee.key == MatchKeyCode::KbdBackspace {
                    self.remove_selection_or_n(1,g,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdReturn {
                    self.insert_text("\n",g,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdLeft {
                    self.move_cursor_x(Direction::Left,ctrl,g,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdRight {
                    self.move_cursor_x(Direction::Right,ctrl,g,root.fork(),ctx);
                }
                self.scroll_to_cursor(&b,g,root,ctx);

                passed = true;
            }else if ee.key == MatchKeyCode::KbdA && ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                // l.mutate_closure(Box::new(move |mut w,ctx,_| { TODO
                //     let wc = w.traitcast_mut::<dyn TextStorMut<E>>().unwrap();
                //     cursor.select = 0;
                //     cursor.caret = wc.len() as u32;
                //     w.traitcast_mut::<dyn AtomStateMut<E,Cursor>>().unwrap().set(cursor,ctx);
                //     w.traitcast_mut::<dyn AtomStateMut<E,Option<u32>>>().unwrap().set(None,ctx);
                // }));
                passed = true;
            }else if ee.key == MatchKeyCode::KbdV && ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                if let Some(text) = ctx.clipboard_get_text() {
                    self.insert_text(&text,g,root.fork(),ctx);
                    self.scroll_to_cursor(&b,g,root,ctx);
                }

                passed = true;
            }else if (ee.key == MatchKeyCode::KbdC || ee.key == MatchKeyCode::KbdX) && ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                if let TxtCurSelBytePos::Selection(range) = cursor.typ() {
                    let text = self.text.caption();
                    let text = &text.as_ref()[range];
                    ctx.clipboard_set_text(text);

                    if ee.key == MatchKeyCode::KbdX {
                        self.remove_selection(g,root.fork(),ctx);
                        self.scroll_to_cursor(&b,g,root,ctx);
                    }
                }
                passed = true;
            }else if ee.key == MatchKeyCode::KbdUp || ee.key == MatchKeyCode::KbdDown {
                let ctrl = ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some();

                let b = b.clone();
                
                if ee.key == MatchKeyCode::KbdUp {
                    self.move_cursor_y(Direction::Up,ctrl,&b,g,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdDown {
                    self.move_cursor_y(Direction::Down,ctrl,&b,g,root.fork(),ctx);
                }
                self.scroll_to_cursor(&b,g,root,ctx);

                passed = true;
            }
        } else if let Some(ee) = event.query_variant::<MouseScroll>(path,&stack) {
            //let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,&b);
            let off = self.get_scroll(ctx);
            let max_off = max_off::<E>(&g,&b);

            let off = (
                off.0 as i32 + ee.x,
                off.1 as i32 + ee.y,
            );
            //let off = s.bound_off((off.0.max(0) as u32, off.1.max(0) as u32));
            let off = (
                off.0.max(0).min(max_off.x) as u32,
                off.1.max(0).min(max_off.y) as u32,
            );

            if let Some(t) = self.update.build_box_mut_event(TextBoxUpdate { update_text: None, update_cursor: None, update_scroll_pos: Some(off) }) {
                ctx.mutate_closure(t);
            }
            passed = true;
        } else {
            if let Some(mouse) = ctx.state().cursor_pos() { //TODO strange event handling

                let mouse_down = event.query_variant::<MouseDown<E>>(path,&stack).cloned();
                let mouse_pressed = ctx.state().is_hovered(path._erase()) && ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,path._erase()).is_some();
                let b = b.clone();

                self._m(mouse_down,mouse_pressed,mouse,b,g,root.fork(),ctx);
                if mouse_pressed {
                    self.scroll_to_cursor(&b,g,root,ctx);
                }

                passed |= mouse_pressed;
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
        0
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
        (callback)(Err(()),ctx)
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
        (callback)(Err(todo!()),ctx)
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
        Err(todo!())
    }

    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     Ok(vec![])
    // }
    fn focusable(&self) -> bool {
        true
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn TextStor<E> => |s| &s.text;
        // dyn AtomState<E,(u32,u32)> => |s| &s.scroll;
        // dyn AtomState<E,ETCurSel<E>> => |s| &s.cursor;
        dyn ITextBox<E> => |s| s;
        dyn Validation<E> => |s| &s.text;
    );
}

impl<E,Text,Scroll,Curs,TBUpd> AsWidget<E> for TextBox<'_,E,Text,Scroll,Curs,TBUpd> where Self: Widget<E>, E: Env {
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
pub struct TextBoxCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E> {
    pub(super) text_cache: Option<ETextLayout<E>>,
    pub(super) text_cachor: Option<Arc<dyn Any>>,
    pub(super) scroll_curs_cachor: Option<(ETCurSelCachor<E>,Offset,bool)>,
    pub(super) text_rendered: bool,
    pub(super) std_render_cachors: Option<StdRenderCachors<E>>,
    //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
}

impl<E> WidgetCache<E> for TextBoxCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E> {
    fn reset_current(&mut self) {}
}
