use super::*;
use util::{state::AtomState};
use crate::text::stor::*;
use crate::text::layout::TxtLayout;
use std::{sync::Arc, ops::{Range, RangeInclusive}};

pub fn max_off<E>(g: &E::TextBoxor, b: &Bounds) -> Offset where E: Env {
    let size = g.size();
    Offset {
        x: size.w.saturating_sub( b.w() ) as i32,
        y: size.h.saturating_sub( b.h() ) as i32,
    }
}

/*pub struct TBState<E> where E: Env {
    pub off: (u32,u32), //TODO fix pub
    pub max_off: (u32,u32),
    pub cursor: Cursor,
    pub glyphs: Arc<ESGlyphs<E>>,
}

impl<E> TBState<E> where E: Env {
    pub fn retrieve<'a,S,P,C>(TextStor: &S, glyphs: Arc<E::TextBoxor>, p: &P, c: &C, ctx: &mut E::Context, b: &Bounds) -> Self where S: TextStor<E>+'a, P: AtomState<E,(u32,u32)>, C: AtomState<E,Cursor> {
        let off = p.get(ctx);
        //assert_eq!(glyphs.chars() as usize,TextStor.len()+1);
        let siz = glyphs.size();
        let max_off = (
            siz.w.saturating_sub( b.w() ),
            siz.h.saturating_sub( b.h() ),
        );
        let cursor = c.get(ctx);
        let num_glyphs = TextStor.len() as u32;
        let cursor = cursor.min(num_glyphs);
        Self{
            off,
            max_off,
            cursor,
            glyphs,
        }
    }
    pub fn bound_off(&self, o: (u32,u32)) -> (u32,u32) {
        (
            o.0.min(self.max_off.0),
            o.1.min(self.max_off.1),
        )
    }
    pub fn bound_off2(&self, o: (u32,u32)) -> Offset {
        Offset{
            x: o.0.min(self.max_off.0) as i32,
            y: o.1.min(self.max_off.1) as i32,
        }
    }
    pub fn off2(&self) -> Offset {
        Offset{
            x: self.off.0 as i32,
            y: self.off.1 as i32,
        }
    }

    pub fn cursor_display_pos(&self, i: u32) -> Option<(u32,u32)> {
        self.glyphs.char_at(i)
        .map(|i| 
            (
                i.offset().x as u32,
                i.offset().y as u32,
            )
        )
    }
    
    pub fn cpl(&self, i: u32) -> Option<((u32,u32),Bounds,usize)> {
        let mut j = 0;
        for (k,l) in self.glyphs.lines().enumerate() {
            for c in l.0 {
                if i == j {
                    return Some((
                        (
                            c.offset().x as u32,
                            (c.offset().y as u32).saturating_sub(self.glyphs.line_ascent()),
                        ),
                        l.1,
                        k
                    ));
                }
                j+=1;
            }
        }
        None
    }

    pub fn selection_box(&self) -> Vec<Bounds> {
        let sel = self.cursor.range();

        if sel.len() == 0 {
            return Vec::new();
        }

        let start = self.cpl(sel.start).unwrap();
        let end = self.cpl(sel.end).unwrap();

        assert!(end.2 >= start.2);

        let mut dest = Vec::new();

        if start.2 == end.2 {
            return vec![
                Bounds::from_xyxy(
                    start.0 .0 as i32,
                    start.1 .y(),
                    end.0 .0 as i32,
                    start.1 .y1(),
                )
            ];
        }

        dest.push(Bounds::from_xyxy(
            start.0 .0 as i32,
            start.1 .y(),
            start.1 .x1(),
            start.1 .y1(),
        ));

        for i in start.2+1 .. end.2 {
            let b = self.glyphs.lines().skip(i).next().unwrap().1; //TODO OPTI
            dest.push(b);
        }

        dest.push(Bounds::from_xyxy(
            end.1 .x(),
            end.1 .y(),
            end.0 .0 as i32,
            end.1 .y1(),
        ));

        dest
    }

    pub fn cursor_pos_reverse(&self, pos: Offset) -> u32 {
        self.glyphs.glyphs()
            .enumerate()
            .filter(|(_,b)| b.offset().y <= pos.y && b.offset().x <= pos.x )
            .map(|(i,_)| i as u32 )
            .last() // TODO PERF all this seekery will be slow on huge texts
            .unwrap_or(0)
    }
    pub fn cursor_pos_reverse_line_centric(&self, line: u32, x: i32) -> Option<u32> {
        fn try_centerx<E>(p: &ESGlyph<E>) -> i32 where E: Env {
            if let Some(o) = p.bounds() {
                o.center().x
            }else{
                p.offset().x
            }
        }

        self.glyphs.lines()
            .skip(line as usize)
            .next()
            .map(|(l,_)| {
                let mut last = 0;

                for (i,b) in l.enumerate() {
                    if try_centerx::<E>(&b) >= x {
                        return i as u32;
                    }
                    last = i;
                }

                return last as u32;
            })
    }
}*/

#[derive(Copy,Clone,Default)]
pub struct Cursor {
    pub select: u32,
    pub caret: u32,
}

impl Cursor {
    pub fn min(&self, min: u32) -> Self {
        Self{
            select: self.select.min(min),
            caret: self.caret.min(min),
        }
    }
    pub fn range(&self) -> Range<u32> {
        self.select.min(self.caret) .. self.select.max(self.caret)
    }
    pub fn range_usize(&self) -> Range<usize> {
        self.select.min(self.caret) as usize .. self.select.max(self.caret) as usize
    }
    pub fn range_incl(&self) -> RangeInclusive<u32> {
        self.select.min(self.caret) ..= self.select.max(self.caret)
    }
    pub fn start_len(&self) -> (u32,u32) {
        let r = self.range();
        (r.start,r.end-r.start)
    }
    pub fn is_selection(&self) -> bool {
        self.select != self.caret
    }
    pub fn unselect(&mut self) {
        self.select = self.caret;
    }
    pub fn unselect_add(&mut self, o: u32, skip_unselect: bool) {
        self.caret += o;
        if !skip_unselect {
            self.select = self.caret;
        }
    }
    pub fn unselect_sub(&mut self, o: u32, skip_unselect: bool) {
        self.caret = self.caret.saturating_sub(o);
        if !skip_unselect {
            self.select = self.caret;
        }
    }
    pub fn unselect_addi(&mut self, o: i32, skip_unselect: bool) {
        self.caret = (self.caret as i32 +o).max(0) as u32;
        if !skip_unselect {
            self.select = self.caret;
        }
    }
    pub fn limit(&mut self, min: u32) {
        *self = self.min(min);
    }
    pub fn del_selection<'a,S,E>(&mut self, c: &mut S) where S: TextStorMut<E>+'a, E: Env {
        let (start,len) = self.start_len();
        c.remove_chars(self.range_usize());
        self.caret = start;
        self.unselect();
    }
    pub fn fix_boundaries<E>(&mut self, t: &impl TxtLayout<E>) where E: Env {
        self.caret = t.fix_boundary(self.caret as usize) as u32;
        self.select = t.fix_boundary(self.select as usize) as u32;
    }
}
