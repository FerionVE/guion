use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::style::standard::cursor::StdCursor;
use crate::text::cursel::Direction;
use crate::text::cursel::TxtCurSel;
use crate::text::cursel::TxtCurSelBytePos;
use crate::text::layout::TxtLayout;
use crate::text::layout::TxtLayoutFromStor;
use crate::text::stor::*;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::stack::QueryCurrentBounds;

use super::*;
use state::max_off;
use util::{state::*, LocalGlyphCache};
use super::imp::*;
use validation::*;

impl<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> Widget<E> for TextBox<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
    Scroll: AtomState<E,(u32,u32)>,
    Curs: AtomState<E,ETCurSel<E>>,
    TBUpd: TBMut<E>,
    TBScr: TBSM<E>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
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
        let render_props = render_props.inside_border_of_type_mul(TestStyleBorderType::Component,2);

        let g = self.glyphs(root,ctx);
        //let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,r.bounds());
        let mut cursor = self.cursor.get(ctx);
        //cursor.fix_boundaries(&*g);
        let off: Offset = self.scroll.get(ctx).into();

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

        if ctx.state().is_hovered(&self.id) {
            renderer.set_cursor_specific(&StdCursor::IBeam.into(),ctx);
        }

        renderer.render_preprocessed_text(
            &g,
            off,
            &render_props
                .with_style_color_type(TestStyleColorType::Fg),
            ctx
        )
    }
    
    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        let stack = with_inside_spacing_border(stack);

        let style = QueryTestStyle.query_in(&stack).unwrap();
        let bounds = QueryCurrentBounds.query_in(&stack).unwrap();
        let event_mode = event.query_std_event_mode(&stack).unwrap();

        if !event_mode.receive_self {return false;}

        //e.0._debug_type_name();
        let g = self.glyphs(root.fork(),ctx);

        let mut cursor = self.cursor.get(ctx);
        g.fix_cursor_boundaries(&mut cursor);

        let border = style.border_of_type(TestStyleBorderType::Component)*2;
        let b = bounds.bounds.inside_border(&border);

        let mut passed = false;

        if let Some(ee) = event.query_variant::<TextInput,_>(&stack)  {
            if !ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                let s = ee.text.clone();
                
                self.insert_text(&s,root.fork(),ctx);
                self.scroll_to_cursor(&b,root,ctx);

                passed = true;
            }
        } else if let Some(ee) = event.query_variant::<KbdPress<E>,_>(&stack) {
            if
                ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdBackspace ||
                ee.key == MatchKeyCode::KbdLeft || ee.key == MatchKeyCode::KbdRight
            {
                let ctrl = ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some();

                
                if ee.key == MatchKeyCode::KbdBackspace {
                    self.remove_selection_or_n(1,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdReturn {
                    self.insert_text("\n",root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdLeft {
                    self.move_cursor_x(Direction::Left,ctrl,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdRight {
                    self.move_cursor_x(Direction::Right,ctrl,root.fork(),ctx);
                }
                self.scroll_to_cursor(&b,root,ctx);

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
                    self.insert_text(&text,root.fork(),ctx);
                    self.scroll_to_cursor(&b,root,ctx);
                }

                passed = true;
            }else if (ee.key == MatchKeyCode::KbdC || ee.key == MatchKeyCode::KbdX) && ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                if let TxtCurSelBytePos::Selection(range) = cursor.typ() {
                    let text = self.text.caption();
                    let text = &text.as_ref()[range];
                    ctx.clipboard_set_text(text);

                    if ee.key == MatchKeyCode::KbdX {
                        self.remove_selection(root.fork(),ctx);
                        self.scroll_to_cursor(&b,root,ctx);
                    }
                }
                passed = true;
            }else if ee.key == MatchKeyCode::KbdUp || ee.key == MatchKeyCode::KbdDown {
                let ctrl = ctx.state().is_pressed(MatchKeyCode::KbdCtrl).is_some();

                let b = b.clone();
                
                if ee.key == MatchKeyCode::KbdUp {
                    self.move_cursor_y(Direction::Up,ctrl,&b,root.fork(),ctx);
                }
                if ee.key == MatchKeyCode::KbdDown {
                    self.move_cursor_y(Direction::Down,ctrl,&b,root.fork(),ctx);
                }
                self.scroll_to_cursor(&b,root,ctx);

                passed = true;
            }
        } else if let Some(ee) = event.query_variant::<MouseScroll,_>(&stack) {
            //let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,&b);
            let off = self.scroll.get(ctx);
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

            if let Some(t) = self.scroll_update.boxed(off) {
                ctx.mutate_closure(t);
            }
            passed = true;
        } else {
            if let Some(mouse) = ctx.state().cursor_pos() { //TODO strange event handling

                let mouse_down = event.query_variant::<MouseDown<E>,_>(&stack).cloned();
                let mouse_pressed = ctx.state().is_hovered(&self.id()) && ctx.state().is_pressed_and_id(MatchKeyCode::MouseLeft,self.id.clone()).is_some();
                let b = b.clone();

                self._m(mouse_down,mouse_pressed,mouse,b,root.fork(),ctx);
                if mouse_pressed {
                    self.scroll_to_cursor(&b,root,ctx);
                }

                passed |= mouse_pressed;
            }
        }
        passed
    }

    fn _size<P>(
        &self,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
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

    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        true
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn TextStor<E> => |s| &s.text;
        dyn AtomState<E,(u32,u32)> => |s| &s.scroll;
        dyn AtomState<E,ETCurSel<E>> => |s| &s.cursor;
        dyn ITextBox<E> => |s| s;
        dyn AtomState<E,LocalGlyphCache<E>> => |s| &s.glyph_cache;
        dyn Validation<E> => |s| &s.text;
    );
}

impl<'z,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> AsWidget<'z,E> for TextBox<'z,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,Ret>(&'w self, f: F, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> Ret
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,Ret,E>
    {
        f.call(self, root, ctx)
    }
}
