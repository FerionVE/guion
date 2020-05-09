use super::*;
use util::state::AtomStateX;

pub struct State<E> where E: Env {
    pub off: (u32,u32), //TODO fix pub
    pub max_off: (u32,u32),
    pub cursor: u32,
    pub glyphs: ESPPText<E>,
}

impl<E> State<E> where E: Env {
    pub fn retrieve<'a,S,P,C>(s: &S, p: &P, c: &C, ctx: &mut E::Context, b: &Bounds) -> Self where S: Caption<'a>, P: AtomStateX<E,(u32,u32)>, C: AtomStateX<E,u32> {
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
}