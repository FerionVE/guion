use super::*;
use util::{caption::CaptionMut, state::AtomStateX};
use std::ops::{Range, RangeInclusive};

pub struct State<E> where E: Env {
    pub off: (u32,u32), //TODO fix pub
    pub max_off: (u32,u32),
    pub cursor: Cursor,
    pub glyphs: ESPPText<E>,
}

impl<E> State<E> where E: Env {
    pub fn retrieve<'a,S,P,C>(s: &S, p: &P, c: &C, ctx: &mut E::Context, b: &Bounds) -> Self where S: Caption<'a>, P: AtomStateX<E,(u32,u32)>, C: AtomStateX<E,Cursor> {
        let off = p.get(ctx);
        let caption = s.caption();
        let glyphs = ESPPText::<E>::generate(caption.as_ref(),(20.0,20.0),ctx);
        assert_eq!(glyphs.chars() as usize,caption.len()+1);
        let siz = glyphs.size();
        let max_off = (
            siz.w.saturating_sub( b.w() ),
            siz.h.saturating_sub( b.h() ),
        );
        let cursor = c.get(ctx);
        let num_glyphs = caption.len() as u32;
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
                i.offset.x as u32,
                i.offset.y as u32,
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
                            c.offset.x as u32,
                            (c.offset.y as u32).saturating_sub(self.glyphs.line_ascent()),
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

        if ExactSizeIterator::len(&sel) == 0 {
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
            .filter(|(_,b)| b.offset.y <= pos.y && b.offset.x <= pos.x )
            .map(|(i,_)| i as u32 )
            .last() // TODO PERF all this seekery will be slow on huge texts
            .unwrap_or(0)
    }
}

#[derive(Copy,Clone)]
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
    pub fn unselect_add(&mut self, o: u32) {
        self.caret += o;
        self.select = self.caret;
    }
    pub fn unselect_sub(&mut self, o: u32) {
        self.caret = self.caret.saturating_sub(o);
        self.select = self.caret;
    }
    pub fn limit(&mut self, min: u32) {
        *self = self.min(min);
    }
    pub fn del_selection<'a,S>(&mut self, c: &mut S) where S: CaptionMut<'a> {
        let (start,len) = self.start_len();
        c.pop_left((start+len) as usize, len as usize);
        self.caret = start;
        self.unselect();
    }
}