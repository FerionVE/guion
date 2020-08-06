use super::*;

pub trait Font<E>: Sized where E: Env {
    
}

/// Text in a optimized form for faster frequent rendering and iterating
pub trait Glyphs<E>: Sized where E: Env {
    type Glyph: Glyph;

    //type LineIter: Iterator<Item=(Self::CharIter,Bounds)>;
    //type CharIter: Iterator<Item=Bounds>;

    fn size(&self) -> Dims;
    fn lines<'s>(&'s self) -> CrazyWorkaroundPPIter<'s,Self::Glyph>;

    fn generate(s: &str, size: (f32,f32), ctx: &mut E::Context) -> Self;

    fn chars(&self) -> u32 {
        let mut i = 0u32; //TODO more optimal fn

        for (l,_) in self.lines() {
            for _ in l {
                i+=1;
            }
        }

        i
    }

    fn glyphs<'s>(&'s self) -> Box<dyn Iterator<Item=Self::Glyph>+'s> {
        Box::new( //TODO OPTI use ext trait to avoid boxing
            self.lines()
            .flat_map(|(c,_)| c )
        )
    }

    fn char_at(&self, i: u32) -> Option<Self::Glyph> {
        self.lines()
            .flat_map(|(c,_)| c )
            .skip(i as usize)
            .next()
    }

    fn coord_of(&self, i: u32) -> Option<(u32,u32)> {
        let mut j = 0;
        
        for (y,(line,_)) in self.lines().enumerate() {
            for (x,_) in line.enumerate() {
                if j == i {
                    return Some((x as u32,y as u32));
                }
                j+=1;
            }
        }

        None
    }

    fn at_coord(&self, xy: (u32,u32)) -> Option<u32> {
        let mut i = 0;

        for (y,(line,_)) in self.lines().enumerate() {
            for (x,_) in line.enumerate() {
                if x == xy.0 as usize && y == xy.1 as usize {
                    return Some(i as u32);
                }
                i+=1;
            }
        }

        None
    }

    fn line_ascent(&self) -> u32;
    fn line_height(&self) -> u32;
    fn line_distance(&self) -> u32;
    fn line_count(&self) -> u32 {
        self.lines().count() as u32
    }
}

/// Provides informations over a glyph
pub trait Glyph {
    fn bounds(&self) -> Option<Bounds>;
    fn offset(&self) -> Offset;
    fn str_pos(&self) -> usize;
}

pub struct GlyphInfo {
    pub bounds: Option<Bounds>,
    pub offset: Offset,
    pub str_pos: usize,
}

impl Glyph for GlyphInfo {
    #[inline]
    fn bounds(&self) -> Option<Bounds> {
        self.bounds.clone()
    }
    #[inline]
    fn offset(&self) -> Offset {
        self.offset.clone()
    }
    #[inline]
    fn str_pos(&self) -> usize {
        self.str_pos
    }
}

pub type CrazyWorkaroundPPIter<'a,G: Glyph> = Box<dyn Iterator<Item=(Box<dyn Iterator<Item=G>+'a>,Bounds)>+'a>;
