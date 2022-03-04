use super::*;
use util::{state::AtomState};
use crate::text::stor::*;
use crate::text::layout::TxtLayout;
use std::{sync::Arc, ops::{Range, RangeInclusive}};

pub fn max_off<E>(g: &ETextLayout<E>, b: &Bounds) -> Offset where E: Env {
    let size = g.display_size();
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
    pub fn retrieve<'a,S,P,C>(TextStor: &S, glyphs: Arc<ETextLayout<E>>, p: &P, c: &C, ctx: &mut E::Context<'_>, b: &Bounds) -> Self where S: TextStor<E>+'a, P: AtomState<E,(u32,u32)>, C: AtomState<E,Cursor> {
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
