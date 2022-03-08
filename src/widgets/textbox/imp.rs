use crate::text::cursel::{Direction, TxtCurSel};
use crate::text::layout::TxtLayoutFromStor;
use crate::text::stor::TextStor;
use crate::text::layout::TxtLayout;

use super::*;
use util::{state::AtomState, LocalGlyphCache};
use std::borrow::Cow;
use std::sync::Arc;
use validation::*;

pub trait ITextBox<E> where E: Env {
    fn insert_text(&self, l: Link<E>, t: &str);
    fn remove_selection_or_n(&self, l: Link<E>, n: u32);
    fn remove_selection(&self, l: Link<E>) -> bool;
    fn move_cursor_x(&self, l: Link<E>, o: Direction, skip_unselect: bool);
    fn move_cursor_y(&self, l: Link<E>, o: Direction, skip_unselect: bool, widget_bounds: &Bounds);
    fn _m(&self, l: Link<E>, mouse_down: Option<MouseDown<E>>, mouse_pressed: bool, mouse: Offset, b: Bounds);
    fn scroll_to_cursor(&self, l: Link<E>, b: &Bounds);
    fn update(&self, l: &mut Link<E>, tu: Option<(Range<usize>,Cow<'static,str>)>, nc: Option<ETCurSel<E>>);
}

impl<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> ITextBox<E> for TextBox<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where
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
    fn insert_text(&self, mut l: Link<E>, s: &str) {
        let g = self.glyphs(l.reference());
        let mut cursor = self.cursor.get(l.ctx);
        g.fix_cursor_boundaries(&mut cursor);
        if cursor.is_selection() {
            let (del_range,new_cursor) = 
                cursor.attempt_replace_text(s.len(), self.text.len());
            self.update(&mut l,Some((del_range,Cow::Owned(s.to_owned()))),Some(new_cursor));
        } else {
            let (ins_off,new_cursor) = 
                cursor.attempt_insert_text(s.len(), self.text.len());
            self.update(&mut l,Some((ins_off..ins_off,Cow::Owned(s.to_owned()))),Some(new_cursor));
        }
    }
    fn remove_selection_or_n(&self, mut l: Link<E>, n: u32) {
        if self.remove_selection(l.reference()) {return;}

        let g = self.glyphs(l.reference());
        let mut cursor = self.cursor.get(l.ctx);
        g.fix_cursor_boundaries(&mut cursor);

        let to_remove = g.char_len_l(cursor.caret(), n as usize);

        let (del_range,new_cursor) = 
            cursor.attempt_backspace(to_remove, self.text.len());
        self.update(&mut l,Some((del_range,Default::default())),Some(new_cursor));
    }
    fn remove_selection(&self, mut l: Link<E>) -> bool {
        let g = self.glyphs(l.reference());
        let mut cursor = self.cursor.get(l.ctx);
        g.fix_cursor_boundaries(&mut cursor);

        if cursor.is_selection() {
            let (del_range,new_cursor) = 
                cursor.attempt_replace_text(0, self.text.len());
            self.update(&mut l,Some((del_range,Default::default())),Some(new_cursor));
            true
        }else{
            false
        }
    }
    fn move_cursor_x(&self, mut l: Link<E>, o: Direction, skip_unselect: bool) {
        let g = self.glyphs(l.reference());
        let cursor = self.cursor.get(l.ctx);

        let new_cursor = g.move_cursor_direction(cursor,o,skip_unselect);

        self.update(&mut l,None,Some(new_cursor))
    }
    fn move_cursor_y(&self, mut l: Link<E>, o: Direction, skip_unselect: bool, b: &Bounds) {
        let g = self.glyphs(l.reference());
        let cursor = self.cursor.get(l.ctx);

        let new_cursor = g.move_cursor_direction(cursor,o,skip_unselect);

        self.update(&mut l,None,Some(new_cursor))
    }
    fn _m(&self, mut l: Link<E>, mouse_down: Option<MouseDown<E>>, mouse_pressed: bool, mouse: Offset, b: Bounds) {
        let g = self.glyphs(l.reference());
        let mut cursor = self.cursor.get(l.ctx);
        g.fix_cursor_boundaries(&mut cursor);

        let off = self.scroll.get(l.ctx);

        let tpos = mouse - b.off + Offset::from(off);
        //tpos.y += g.line_ascent() as i32; //TODO FIX boundary precision all over the place
                    
        if let Some(ee) = mouse_down {
            let new_cursor = 
                g.move_cursor_display(cursor, tpos, false);
            self.update(&mut l,None,Some(new_cursor))
        } else if mouse_pressed {
            let new_cursor = 
                g.move_cursor_display(cursor, tpos, true);
            self.update(&mut l,None,Some(new_cursor))
        }
    }
    fn scroll_to_cursor(&self, mut l: Link<E>, b: &Bounds) {
        let g = self.glyphs(l.reference());
        let mut cursor = self.cursor.get(l.ctx);
        g.fix_cursor_boundaries(&mut cursor);

        let off = self.scroll.get(l.ctx);
        
        let cb = g.cursor_bounds(cursor); //TODO fix as it should work if cursor is at end
            
        let mut vb = Bounds{
            off: off.into(),
            size: b.size,
        };

        vb.shift_to_fit(&cb);

        let off = (vb.off.x as u32, vb.off.y as u32);
        
        if let Some(t) = self.scroll_update.boxed(off) {
            l.mutate_closure(t);
        }
    }

    fn update(&self, l: &mut Link<E>, tu: Option<(Range<usize>,Cow<'static,str>)>, nc: Option<ETCurSel<E>>) {
        if tu.is_some() || nc.is_some() {
            if let Some(t) = self.update.boxed(tu,nc) {
                l.mutate_closure(t);
            }
        }
    }
}

traitcast_for_from_widget!(ITextBox<E>);

impl<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> TextBox<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: TextStor<E>+Validation<E>+'w,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
    Scroll: AtomState<E,(u32,u32)>,
    Curs: AtomState<E,ETCurSel<E>>,
    TBUpd: TBMut<E>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    pub(crate) fn glyphs(&self, mut l: Link<E>) -> Arc<ETextLayout<E>> { //TODO FIX style mutation invalidating glyphs
        if let Some((v,c)) = self.glyph_cache.get(l.ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let glyphs: Arc<ETextLayout<E>> = Arc::new(
            TxtLayoutFromStor::<Text,E>::from(&self.text,l.ctx)
        );

        // let g = glyphs.refc();
        // l.mutate_closure(Box::new(move |mut w,ctx,_| {
        //     let vali = w.traitcast_mut::<dyn ValidationMut<E>>().unwrap();
        //     let key = vali.validate();
        //     let cache = w.traitcast_mut::<dyn AtomStateMut<E,LocalGlyphCache<E>>>().unwrap();
        //     cache.set( Some((g,key)) ,ctx);
        // }));

        glyphs
    }
}
