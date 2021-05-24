use crate::style::standard::cursor::StdCursor;

use super::*;
use util::{state::*, caption::CaptionMut, LocalGlyphCache};
use state::{Cursor, TBState};
use super::imp::*;
use validation::*;

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> Widget<E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: Caption<E>+Validation<E>+'w,
    Scroll: AtomState<E,(u32,u32)>,
    Curs: AtomState<E,Cursor>,
    CursorStickX: AtomState<E,Option<u32>>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
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
        let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,r.bounds());
        for b in s.selection_box() {
            let b = b - s.off2();
            r.slice(&b)
                .with(StdSelectag::ObjForeground)
                .fill_rect(l.ctx);
        }
        if let Some(c) = s.cursor_display_pos(s.cursor.caret) { //TODO fix as it should work if cursor is at end
            let b = Bounds::from_xywh(c.0 as i32, c.1 as i32 - s.glyphs.line_ascent() as i32, 2, s.glyphs.line_height());
            let b = b - s.off2();
            r.slice(&b)
                .with(StdSelectag::ObjActive)
                .fill_rect(l.ctx);
        }
        if l.state().is_hovered(&self.id) {
            r.set_cursor_specific(&StdCursor::IBeam.into(),l.ctx);
        }

        r.with(&[
                StdSelectag::ObjForeground,
                StdSelectag::ObjText,
            ][..])
                .render_preprocessed_text(&s.glyphs, s.off2(), &mut l.ctx);
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = e.with_style(&self.style);
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));

        //e.0._debug_type_name();
        let mut cursor = self.cursor.get(l.ctx);
        let border = e.style.border(&StdSelectag::BorderVisual.into_selector(),l.ctx)*2;
        let b = e.bounds.inside_border(&border);

        let mut passed = false;

        if let Some(ee) = e.event.is_text_input() {
            let s = ee.text;
            l.mutate_closure(Box::new(move |mut w,ctx,_| {
                let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                w.insert_text(&s,ctx);
                w.scroll_to_cursor(ctx,&b);
            }));
            passed = true;
        } else if let Some(ee) = e.event.is_kbd_press() {
            if
                ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::BACKSPACE ||
                ee.key == EEKey::<E>::LEFT || ee.key == EEKey::<E>::RIGHT
            {
                let ctrl = l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some();

                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                    if ee.key == EEKey::<E>::BACKSPACE {
                        w.remove_selection_or_n(1,ctx);
                    }
                    if ee.key == EEKey::<E>::ENTER {
                        w.insert_text("\n",ctx);
                    }
                    if ee.key == EEKey::<E>::LEFT {
                        w.move_cursor_x(-1,ctrl,ctx);
                    }
                    if ee.key == EEKey::<E>::RIGHT {
                        w.move_cursor_x(1,ctrl,ctx);
                    }
                    w.scroll_to_cursor(ctx,&b);
                }));
                passed = true;
            }else if ee.key == EEKey::<E>::A && l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let wc = w.traitcast_mut::<dyn CaptionMut<E>>().unwrap();
                    cursor.select = 0;
                    cursor.caret = wc.len() as u32;
                    w.traitcast_mut::<dyn AtomStateMut<E,Cursor>>().unwrap().set(cursor,ctx);
                    w.traitcast_mut::<dyn AtomStateMut<E,Option<u32>>>().unwrap().set(None,ctx);
                }));
                passed = true;
            }else if ee.key == EEKey::<E>::V && l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                if let Some(text) = l.clipboard_get_text() {
                    self._event_direct(
                        l,
                        &EventCompound{
                            event: Event::from(TextInput{text}),
                            bounds: e.bounds,
                            ts: e.ts,
                            filter: Default::default(),
                            style: e.style.clone(),
                            flag: e.flag,
                        },
                    );
                }
                passed = true;
            }else if (ee.key == EEKey::<E>::C || ee.key == EEKey::<E>::X) && l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                if cursor.is_selection() {
                    let range = cursor.range_usize();
                    let text = self.text.caption();
                    let text = &text.as_ref()[range];
                    l.clipboard_set_text(text);

                    if ee.key == EEKey::<E>::X {
                        l.mutate_closure(Box::new(move |mut w,ctx,_| {
                            let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                            w.remove_selection(ctx);
                            w.scroll_to_cursor(ctx,&b);
                        }));
                    }
                }
                passed = true;
            }else if ee.key == EEKey::<E>::UP || ee.key == EEKey::<E>::DOWN {
                let b = b.clone();
                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();

                    if ee.key == EEKey::<E>::UP {
                        w.move_cursor_y(-1,false,ctx,&b);
                    }
                    if ee.key == EEKey::<E>::DOWN {
                        w.move_cursor_y(1,false,ctx,&b);
                    }
                    w.scroll_to_cursor(ctx,&b);
                }));
                passed = true;
            }
        } else if let Some(ee) = e.event.is_mouse_scroll() {
            let s = TBState::<E>::retrieve(&self.text,self.glyphs(l.reference()),&self.scroll,&self.cursor,&mut l.ctx,&b);
            
            let off = (
                s.off.0 as i32 + ee.x,
                s.off.1 as i32 + ee.y,
            );
            let off = s.bound_off((off.0.max(0) as u32, off.1.max(0) as u32));
            l.mutate_closure(Box::new(move |mut w,ctx,_| {
                let w = w.traitcast_mut::<dyn AtomStateMut<E,(u32,u32)>>().unwrap();
                w.set(off,ctx);
            }));
            passed = true;
        } else {
            if let Some(mouse) = l.state().cursor_pos() { //TODO strange event handling

                let mouse_down = e.event.is_mouse_down();
                let mouse_pressed = l.is_hovered() && l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],self.id.clone()).is_some();
                let b = b.clone();

                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                    w._m(mouse_down,mouse_pressed,mouse,b,ctx);
                    if mouse_pressed {
                        w.scroll_to_cursor(ctx,&b);
                    }
                }));
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
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        vec![]
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        vec![]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        true
    }
    fn child(&self, _: usize) -> Result<Resolvable<E>,()> {
        Err(())
    }
    fn into_child<'a>(self: Box<Self>, _: usize) -> Result<Resolvable<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast!(
        dyn Caption<E> => |s| &s.text;
        dyn AtomState<E,(u32,u32)> => |s| &s.scroll;
        dyn AtomState<E,Cursor> => |s| &s.cursor;
        dyn AtomState<E,Option<u32>> => |s| &s.cursor_stick_x;
        dyn ITextBox<E> => |s| s;
        dyn AtomState<E,LocalGlyphCache<E>> => |s| &s.glyph_cache;
        dyn Validation<E> => |s| &s.text;
    );
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> WidgetMut<E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>,
    Text: CaptionMut<E>+ValidationMut<E>+'w,
    Scroll: AtomStateMut<E,(u32,u32)>,
    Curs: AtomStateMut<E,Cursor>,
    CursorStickX: AtomStateMut<E,Option<u32>>,
    GlyphCache: AtomStateMut<E,LocalGlyphCache<E>>+Clone,
{
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        vec![]
    }
    fn into_childs_mut<'a>(self: Box<Self>) -> Vec<ResolvableMut<'a,E>> where Self: 'a {
        vec![]
    }
    fn child_mut(&mut self, _: usize) -> Result<ResolvableMut<E>,()> {
        Err(())
    }
    fn into_child_mut<'a>(self: Box<Self>, _: usize) -> Result<ResolvableMut<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast_mut!(
        dyn Caption<E> => |s| &mut s.text;
        dyn CaptionMut<E> => |s| &mut s.text;
        dyn AtomState<E,(u32,u32)> => |s| &mut s.scroll;
        dyn AtomState<E,Cursor> => |s| &mut s.cursor;
        dyn AtomState<E,Option<u32>> => |s| &mut s.cursor_stick_x;
        dyn AtomStateMut<E,(u32,u32)> => |s| &mut s.scroll;
        dyn AtomStateMut<E,Cursor> => |s| &mut s.cursor;
        dyn AtomStateMut<E,Option<u32>> => |s| &mut s.cursor_stick_x;
        dyn ITextBox<E> => |s| s;
        dyn ITextBoxMut<E> => |s| s;
        dyn AtomState<E,LocalGlyphCache<E>> => |s| &mut s.glyph_cache;
        dyn AtomStateMut<E,LocalGlyphCache<E>> => |s| &mut s.glyph_cache;
        dyn Validation<E> => |s| &mut s.text;
        dyn ValidationMut<E> => |s| &mut s.text;
    );
}
