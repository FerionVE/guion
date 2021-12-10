use crate::text::layout::{Direction, TxtLayoutFromStor};
use crate::text::stor::{TextStor, TextStorMut};
use crate::text::layout::TxtLayout;

use super::*;
use util::{state::{AtomState, AtomStateMut}, LocalGlyphCache};
use std::sync::Arc;
use validation::*;

pub trait ITextBox<E> where E: Env {

}

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> ITextBox<E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    Text: 'w,
    Scroll: 'w,
    Curs: 'w,
    CursorStickX: 'w,
    GlyphCache: 'w,
{

}

pub trait ITextBoxMut<E>: ITextBox<E> where E: Env {
    fn insert_text(&mut self, t: &str, ctx: &mut E::Context<'_>);
    fn remove_selection_or_n(&mut self, n: u32, ctx: &mut E::Context<'_>);
    fn remove_selection(&mut self, ctx: &mut E::Context<'_>) -> bool;
    fn move_cursor_x(&mut self, o: Direction, skip_unselect: bool, ctx: &mut E::Context<'_>);
    fn move_cursor_y(&mut self, o: Direction, skip_unselect: bool, ctx: &mut E::Context<'_>, widget_bounds: &Bounds);
    fn _m(&mut self, mouse_down: Option<MouseDown<E>>, mouse_pressed: bool, mouse: Offset, b: Bounds, ctx: &mut E::Context<'_>);
    fn scroll_to_cursor(&mut self, ctx: &mut E::Context<'_>, b: &Bounds);
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> ITextBoxMut<E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E> + CtxClipboardAccess<E>,
    Text: TextStorMut<E>+ValidationMut<E>+'w,
    ETextLayout<E>: TxtLayoutFromStor<E,Text>,
    Scroll: AtomStateMut<E,(u32,u32)>,
    Curs: AtomStateMut<E,Cursor>,
    CursorStickX: AtomStateMut<E,Option<u32>>,
    GlyphCache: AtomStateMut<E,LocalGlyphCache<E>>+Clone,
{
    fn insert_text(&mut self, s: &str, ctx: &mut E::Context<'_>) {
        let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        if cursor.is_selection() {
            cursor.del_selection(&mut self.text);
        }
        self.text.push_chars(cursor.caret as usize,&s);
        cursor.unselect_add(s.len() as u32,false);
        cursor.limit(self.text.len() as u32);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn remove_selection_or_n(&mut self, n: u32, ctx: &mut E::Context<'_>) {
        let g = self.glyphs2(ctx);
        if self.remove_selection(ctx) {return;}
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        let to_remove = g.char_len_l(cursor.caret as usize, n as usize);
        self.text.remove_chars_old(cursor.caret as usize,to_remove);
        cursor.unselect_sub(to_remove as u32,false);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn remove_selection(&mut self, ctx: &mut E::Context<'_>) -> bool {
        let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        if cursor.is_selection() {
            cursor.del_selection(&mut self.text);
            self.cursor.set(cursor,ctx);
            self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
            true
        }else{
            false
        }
    }
    fn move_cursor_x(&mut self, o: Direction, skip_unselect: bool, ctx: &mut E::Context<'_>) {
        let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        cursor.caret = g.move_cursor(o,cursor.caret as usize) as u32;
        if !skip_unselect {cursor.unselect();}
        cursor.limit(self.text.len() as u32);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn move_cursor_y(&mut self, o: Direction, skip_unselect: bool, ctx: &mut E::Context<'_>, b: &Bounds) {
        /*let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);

        if g.line_count() != 0 {
            let (_,mut line) = g.coord_of(cursor.caret).unwrap();
            let mut dx = g.display_of_char(cursor.caret as usize).x() as u32;

            let mut new_stick_x = self.cursor_stick_x.get(ctx);
            if new_stick_x.is_none() {
                new_stick_x = Some(dx);
            }
            dx = dx.max(new_stick_x.unwrap());

            line = (line as i32 +o).max(0).min(g.line_count() as i32 -1) as u32;

            let x = g.cursor_pos_reverse_line_centric(line,dx as i32).unwrap();

            let i = g.at_coord((x,line)).unwrap();

            cursor.caret = i;
            if !ctx.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                cursor.select = cursor.caret;
            }

            //cursor = cursor.min(self.text.len() as u32);

            self.cursor.set(cursor,ctx);
            self.cursor_stick_x.set(new_stick_x,ctx);
        }*/
        let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        cursor.caret = g.move_cursor(o,cursor.caret as usize) as u32;
        if !skip_unselect {cursor.unselect();}
        cursor.limit(self.text.len() as u32);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn _m(&mut self, mouse_down: Option<MouseDown<E>>, mouse_pressed: bool, mouse: Offset, b: Bounds, ctx: &mut E::Context<'_>) {
        let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        let off = self.scroll.get(ctx);

        let mut tpos = mouse - b.off + Offset::from(off);
        //tpos.y += g.line_ascent() as i32; //TODO FIX boundary precision all over the place
                    
        if let Some(ee) = mouse_down {
            cursor.select = g.char_at_display(tpos) as u32;
            cursor.caret = cursor.select;
            //cursor.unselect();
        } else if mouse_pressed {
            cursor.caret = g.char_at_display(tpos) as u32;
            //cursor.unselect();
        }
        assert!(cursor.caret <= g.len() as u32); //TODO FIXME äöü crash. the whole unicode char handling is borked.
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn scroll_to_cursor(&mut self, ctx: &mut E::Context<'_>, b: &Bounds) {
        let g = self.glyphs2(ctx);
        let mut cursor = self.cursor.get(ctx);
        cursor.fix_boundaries(&*g);
        let off = self.scroll.get(ctx);
        
        let cb = g.display_of_char(cursor.caret as usize); //TODO fix as it should work if cursor is at end
            
        let mut vb = Bounds{
            off: off.into(),
            size: b.size,
        };

        vb.shift_to_fit(&cb);

        let off = (vb.off.x as u32, vb.off.y as u32);
        self.scroll.set(off,ctx);
    }
}

traitcast_for!(ITextBox<E>;ITextBoxMut<E>);

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: TextStor<E>+Validation<E>+'w,
    ETextLayout<E>: TxtLayoutFromStor<E,Text>,
    Scroll: AtomState<E,(u32,u32)>,
    Curs: AtomState<E,Cursor>,
    CursorStickX: AtomState<E,Option<u32>>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    pub(crate) fn glyphs(&self, mut l: Link<E>) -> Arc<ETextLayout<E>> { //TODO FIX style mutation invalidating glyphs
        if let Some((v,c)) = self.glyph_cache.get(l.ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let glyphs: Arc<ETextLayout<E>> = Arc::new(
            TxtLayoutFromStor::<E,Text>::from(&self.text,l.ctx)
        );

        let g = glyphs.refc();
        l.mutate_closure(Box::new(move |mut w,ctx,_| {
            let vali = w.traitcast_mut::<dyn ValidationMut<E>>().unwrap();
            let key = vali.validate();
            let cache = w.traitcast_mut::<dyn AtomStateMut<E,LocalGlyphCache<E>>>().unwrap();
            cache.set( Some((g,key)) ,ctx);
        }));

        glyphs
    }
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E> + CtxClipboardAccess<E>,
    Text: TextStorMut<E>+ValidationMut<E>+'w,
    ETextLayout<E>: TxtLayoutFromStor<E,Text>,
    Scroll: AtomStateMut<E,(u32,u32)>,
    Curs: AtomStateMut<E,Cursor>,
    CursorStickX: AtomStateMut<E,Option<u32>>,
    GlyphCache: AtomStateMut<E,LocalGlyphCache<E>>+Clone,
{
    pub(crate) fn glyphs2(&mut self, ctx: &mut E::Context<'_>) -> Arc<ETextLayout<E>> {
        if let Some((v,c)) = self.glyph_cache.get(ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let glyphs: Arc<ETextLayout<E>> = Arc::new(
            TxtLayoutFromStor::<E,Text>::from(&self.text,ctx)
        );

        let key = self.text.validate();
        self.glyph_cache.set( Some((glyphs.refc(),key)),ctx);

        glyphs
    }
}
