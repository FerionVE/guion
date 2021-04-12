use super::*;
use util::{state::{AtomState, AtomStateMut}, caption::CaptionMut, LocalGlyphCache};
use state::TBState;
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
    fn insert_text(&mut self, t: &str, ctx: &mut E::Context);
    fn remove_selection_or_n(&mut self, n: u32, ctx: &mut E::Context);
    fn remove_selection(&mut self, ctx: &mut E::Context) -> bool;
    fn move_cursor_x(&mut self, o: i32, skip_unselect: bool, ctx: &mut E::Context);
    fn move_cursor_y(&mut self, o: i32, skip_unselect: bool, ctx: &mut E::Context, widget_bounds: &Bounds);
    fn _m(&mut self, mouse_down: Option<MouseDown<E>>, mouse_pressed: bool, mouse: Offset, b: Bounds, ctx: &mut E::Context);
    fn scroll_to_cursor(&mut self, ctx: &mut E::Context, b: &Bounds);
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> ITextBoxMut<E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>,
    Text: CaptionMut<E>+ValidationMut<E>+'w,
    Scroll: AtomStateMut<E,(u32,u32)>,
    Curs: AtomStateMut<E,Cursor>,
    CursorStickX: AtomStateMut<E,Option<u32>>,
    GlyphCache: AtomStateMut<E,LocalGlyphCache<E>>+Clone,
{
    fn insert_text(&mut self, s: &str, ctx: &mut E::Context) {
        let mut cursor = self.cursor.get(ctx);
        if cursor.is_selection() {
            cursor.del_selection(&mut self.text);
        }
        self.text.push(cursor.caret as usize,&s);
        cursor.unselect_add(s.chars().count() as u32,false);
        cursor.limit(self.text.len() as u32);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn remove_selection_or_n(&mut self, n: u32, ctx: &mut E::Context) {
        if self.remove_selection(ctx) {return;}
        let mut cursor = self.cursor.get(ctx);
        self.text.pop_left(cursor.caret as usize,n as usize);
        cursor.unselect_sub(n,false);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn remove_selection(&mut self, ctx: &mut E::Context) -> bool {
        let mut cursor = self.cursor.get(ctx);
        if cursor.is_selection() {
            cursor.del_selection(&mut self.text);
            self.cursor.set(cursor,ctx);
            self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
            true
        }else{
            false
        }
    }
    fn move_cursor_x(&mut self, o: i32, skip_unselect: bool, ctx: &mut E::Context) {
        let mut cursor = self.cursor.get(ctx);
        cursor.unselect_addi(o,skip_unselect);
        self.cursor.set(cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn move_cursor_y(&mut self, o: i32, skip_unselect: bool, ctx: &mut E::Context, b: &Bounds) {
        let g = self.glyphs2(ctx);
        let mut s = TBState::<E>::retrieve(&self.text,g,&self.scroll,&self.cursor,ctx,&b);

        if s.glyphs.line_count() != 0 {
            let (_,mut line) = s.glyphs.coord_of(s.cursor.caret).unwrap();
            let (mut dx,_) = s.cursor_display_pos(s.cursor.caret).unwrap();

            let mut new_stick_x = self.cursor_stick_x.get(ctx);
            if new_stick_x.is_none() {
                new_stick_x = Some(dx);
            }
            dx = dx.max(new_stick_x.unwrap());

            line = (line as i32 +o).max(0).min(s.glyphs.line_count() as i32 -1) as u32;

            let x = s.cursor_pos_reverse_line_centric(line,dx as i32).unwrap();

            let i = s.glyphs.at_coord((x,line)).unwrap();

            s.cursor.caret = i;
            if !ctx.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                s.cursor.select = s.cursor.caret;
            }

            //cursor = cursor.min(self.text.len() as u32);

            self.cursor.set(s.cursor,ctx);
            self.cursor_stick_x.set(new_stick_x,ctx);
        }
    }
    fn _m(&mut self, mouse_down: Option<MouseDown<E>>, mouse_pressed: bool, mouse: Offset, b: Bounds, ctx: &mut E::Context) {
        let g = self.glyphs2(ctx);
        let mut s = TBState::<E>::retrieve(&self.text,g,&self.scroll,&self.cursor,ctx,&b);

        let mut tpos = mouse - b.off + Offset::from(s.off);
        tpos.y += s.glyphs.line_ascent() as i32; //TODO FIX boundary precision all over the place
                    
        if let Some(ee) = mouse_down {
            s.cursor.select = s.cursor_pos_reverse(tpos);
            s.cursor.caret = s.cursor.select;
            //cursor.unselect();
        } else if mouse_pressed {
            s.cursor.caret = s.cursor_pos_reverse(tpos);
            //cursor.unselect();
        }
        assert!(s.cursor.caret < s.glyphs.chars()); //TODO FIXME äöü crash. the whole unicode char handling is borked.
        self.cursor.set(s.cursor,ctx);
        self.cursor_stick_x.set(None,ctx); //TODO this constant unsetting is garbage and breaks is string is mutated externally, rather we should update by cursor move
    }
    fn scroll_to_cursor(&mut self, ctx: &mut E::Context, b: &Bounds) {
        let g = self.glyphs2(ctx);
        let s = TBState::<E>::retrieve(&self.text,g,&self.scroll,&self.cursor,ctx,&b);
        
        if let Some(c) = s.cursor_display_pos(s.cursor.caret) { //TODO fix as it should work if cursor is at end
            let cb = Bounds::from_xywh(c.0 as i32, c.1 as i32 - s.glyphs.line_ascent() as i32, 2, s.glyphs.line_height());
            
            let mut vb = Bounds{
                off: s.off.into(),
                size: b.size,
            };

            vb.shift_to_fit(&cb);

            let off = (vb.off.x as u32, vb.off.y as u32);
            self.scroll.set(off,ctx);
        }
    }
}

traitcast_for!(ITextBox<E>;ITextBoxMut<E>);

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: Caption<E>+Validation<E>+'w,
    Scroll: AtomState<E,(u32,u32)>,
    Curs: AtomState<E,Cursor>,
    CursorStickX: AtomState<E,Option<u32>>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    pub(crate) fn glyphs(&self, mut l: Link<E>) -> Arc<ESGlyphs<E>> { //TODO FIX style mutation invalidating glyphs
        if let Some((v,c)) = self.glyph_cache.get(l.ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let text = self.text.caption();
        let glyphs = Arc::new(ESGlyphs::<E>::generate(text.as_ref(),(20.0,20.0),l.ctx));

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
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>,
    Text: CaptionMut<E>+ValidationMut<E>+'w,
    Scroll: AtomStateMut<E,(u32,u32)>,
    Curs: AtomStateMut<E,Cursor>,
    CursorStickX: AtomStateMut<E,Option<u32>>,
    GlyphCache: AtomStateMut<E,LocalGlyphCache<E>>+Clone,
{
    pub(crate) fn glyphs2(&mut self, ctx: &mut E::Context) -> Arc<ESGlyphs<E>> {
        if let Some((v,c)) = self.glyph_cache.get(ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let text = self.text.caption();
        let glyphs = Arc::new(ESGlyphs::<E>::generate(text.as_ref(),(20.0,20.0),ctx));

        let key = self.text.validate();
        self.glyph_cache.set( Some((glyphs.refc(),key)),ctx);

        glyphs
    }
}
