use super::*;

pub trait Font<E>: Sized where EStyle<E>: Style<E,Font=Self>, E: Env {
    
}

/// Text in a optimized form for faster frequent rendering and iterating
pub trait PreprocessedText<E>: Sized where EStyle<E>: Style<E,PreprocessedText=Self>, E: Env {
    //type LineIter: Iterator<Item=(Self::CharIter,Bounds)>;
    //type CharIter: Iterator<Item=Bounds>;

    fn size(&self) -> Dims;
    fn lines<'s>(&'s self) -> CrazyWorkaroundPPIter<'s>;

    fn generate(s: &str, size: (f32,f32), ctx: &mut E::Context) -> Self;

    fn chars(&self) -> u32 {
        let mut i = 0u32; //TODO more optimal fn

        for (l,_) in self.lines() {
            for c in l {
                i+=1;
            }
        }

        i
    }

    fn char_at(&self, i: u32) -> Option<PPChar> {
        self.lines()
            .flat_map(|(c,_)| c )
            .skip(i as usize)
            .next()
    }
}

pub struct PPChar {
    pub bounds: Option<Bounds>,
    pub offset: Offset,
}

pub type CrazyWorkaroundPPIter<'a> = Box<dyn Iterator<Item=(Box<dyn Iterator<Item=PPChar>+'a>,Bounds)>+'a>;
