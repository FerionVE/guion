use crate::style::standard::cursor::StdCursor;
use crate::text::cursel::Direction;
use crate::text::cursel::TxtCurSel;
use crate::text::cursel::TxtCurSelBytePos;
use crate::text::layout::TxtLayout;
use crate::text::layout::TxtLayoutFromStor;
use crate::text::stor::*;

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
    Text: TextStor<E>+Validation<E>+'w,
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
    fn _render(&self, mut l: Link<E>, r: &mut ERenderer<'_,E>) {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        r.with(&[
            StdSelectag::ObjBorder,
            StdSelectag::Focused(l.is_focused()),
            StdSelectag::BorderVisual,
        ][..])
            .fill_border_inner(l.ctx);
        let mut r = r.inside_border_by_mul(StdSelectag::BorderVisual,2,l.ctx);

        let g = self.glyphs(l.reference());
        //let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,r.bounds());
        let mut cursor = self.cursor.get(l.ctx);
        //cursor.fix_boundaries(&*g);
        let off: Offset = self.scroll.get(l.ctx).into();

        for b in g.selection_bounds(cursor.clone()) {
            let b = b - off;
            r.slice(&b)
                .with(StdSelectag::ObjForeground)
                .fill_rect(l.ctx);
        }
        let mut b = g.cursor_bounds(cursor); //TODO fix as it should work if cursor is at end
        b.size.w = 2;
        //let b = Bounds::from_xywh(c.0 as i32, c.1 as i32 - s.glyphs.line_ascent() as i32, 2, s.glyphs.line_height());
        let b = b - off;
        r.slice(&b)
            .with(StdSelectag::ObjActive)
            .fill_rect(l.ctx);

        if l.state().is_hovered(&self.id) {
            r.set_cursor_specific(&StdCursor::IBeam.into(),l.ctx);
        }

        r.with(&[
                StdSelectag::ObjForeground,
                StdSelectag::ObjText,
            ][..])
                .render_preprocessed_text(&g, off, &mut l.ctx);
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = e.with_style(&self.style);
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));

        //e.0._debug_type_name();
        let g = self.glyphs(l.reference());

        let mut cursor = self.cursor.get(l.ctx);
        g.fix_cursor_boundaries(&mut cursor);

        let border = e.style.border(&StdSelectag::BorderVisual.into_selector(),l.ctx)*2;
        let b = e.bounds.inside_border(&border);

        let mut passed = false;

        if let Some(ee) = e.event.is_text_input() {
            if !l.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                let s = ee.text;
                
                self.insert_text(l.reference(),&s);
                self.scroll_to_cursor(l.reference(),&b);

                passed = true;
            }
        } else if let Some(ee) = e.event.is_kbd_press() {
            if
                ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdBackspace ||
                ee.key == MatchKeyCode::KbdLeft || ee.key == MatchKeyCode::KbdRight
            {
                let ctrl = l.state().is_pressed(MatchKeyCode::KbdCtrl).is_some();

                
                if ee.key == MatchKeyCode::KbdBackspace {
                    self.remove_selection_or_n(l.reference(),1);
                }
                if ee.key == MatchKeyCode::KbdReturn {
                    self.insert_text(l.reference(),"\n");
                }
                if ee.key == MatchKeyCode::KbdLeft {
                    self.move_cursor_x(l.reference(),Direction::Left,ctrl);
                }
                if ee.key == MatchKeyCode::KbdRight {
                    self.move_cursor_x(l.reference(),Direction::Right,ctrl);
                }
                self.scroll_to_cursor(l.reference(),&b);

                passed = true;
            }else if ee.key == MatchKeyCode::KbdA && l.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                // l.mutate_closure(Box::new(move |mut w,ctx,_| { TODO
                //     let wc = w.traitcast_mut::<dyn TextStorMut<E>>().unwrap();
                //     cursor.select = 0;
                //     cursor.caret = wc.len() as u32;
                //     w.traitcast_mut::<dyn AtomStateMut<E,Cursor>>().unwrap().set(cursor,ctx);
                //     w.traitcast_mut::<dyn AtomStateMut<E,Option<u32>>>().unwrap().set(None,ctx);
                // }));
                passed = true;
            }else if ee.key == MatchKeyCode::KbdV && l.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                if let Some(text) = l.clipboard_get_text() {
                    self.insert_text(l.reference(),&text);
                    self.scroll_to_cursor(l.reference(),&b);
                }

                passed = true;
            }else if (ee.key == MatchKeyCode::KbdC || ee.key == MatchKeyCode::KbdX) && l.state().is_pressed(MatchKeyCode::KbdCtrl).is_some() {
                if let TxtCurSelBytePos::Selection(range) = cursor.typ() {
                    let text = self.text.caption();
                    let text = &text.as_ref()[range];
                    l.clipboard_set_text(text);

                    if ee.key == MatchKeyCode::KbdX {
                        self.remove_selection(l.reference());
                        self.scroll_to_cursor(l.reference(),&b);
                    }
                }
                passed = true;
            }else if ee.key == MatchKeyCode::KbdUp || ee.key == MatchKeyCode::KbdDown {
                let ctrl = l.state().is_pressed(MatchKeyCode::KbdCtrl).is_some();

                let b = b.clone();
                
                if ee.key == MatchKeyCode::KbdUp {
                    self.move_cursor_y(l.reference(),Direction::Up,ctrl,&b);
                }
                if ee.key == MatchKeyCode::KbdDown {
                    self.move_cursor_y(l.reference(),Direction::Down,ctrl,&b);
                }
                self.scroll_to_cursor(l.reference(),&b);

                passed = true;
            }
        } else if let Some(ee) = e.event.is_mouse_scroll() {
            //let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,&b);
            let off = self.scroll.get(l.ctx);
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
                l.mutate_closure(t);
            }
            passed = true;
        } else {
            if let Some(mouse) = l.state().cursor_pos() { //TODO strange event handling

                let mouse_down = e.event.is_mouse_down();
                let mouse_pressed = l.is_hovered() && l.state().is_pressed_and_id(MatchKeyCode::MouseLeft,self.id.clone()).is_some();
                let b = b.clone();

                self._m(l.reference(),mouse_down,mouse_pressed,mouse,b);
                if mouse_pressed {
                    self.scroll_to_cursor(l.reference(),&b);
                }

                passed |= mouse_pressed;
            }
        }
        passed
    }
    fn _size(&self, _: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let e = e.and(&self.style);
        self.size.clone()
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref<'s>(&'s self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        vec![]
    }
    fn into_childs<'s>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        vec![]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        true
    }
    fn child<'s>(&'s self, _: usize, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        Err(())
    }
    fn into_child<'s>(self: Box<Self>, _: usize, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        Err(())
    }

    impl_traitcast!( dyn Widget<E>:
        dyn TextStor<E> => |s| &s.text;
        dyn AtomState<E,(u32,u32)> => |s| &s.scroll;
        dyn AtomState<E,ETCurSel<E>> => |s| &s.cursor;
        dyn ITextBox<E> => |s| s;
        dyn AtomState<E,LocalGlyphCache<E>> => |s| &s.glyph_cache;
        dyn Validation<E> => |s| &s.text;
    );
}

impl<'l,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> AsWidget<E> for TextBox<'l,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where Self: Widget<E>, E: Env {
    type Widget = Self;
    type WidgetOwned = Self;

    #[inline]
    fn as_widget<'w>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget<'w>(self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(self)
    }
    #[inline]
    fn box_into_widget<'w>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned(*self)
    }
    #[inline]
    fn as_widget_dyn<'w,'s>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget_dyn<'w,'s>(self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    #[inline]
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
    }
}
