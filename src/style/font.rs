use super::*;

pub trait Font<E>: Sized where EStyle<E>: Style<E,Font=Self>, E: Env {
    
}

/// Text in a optimized form for faster frequent rendering and iterating
pub trait PreprocessedText<E>: Sized where EStyle<E>: Style<E,PreprocessedText=Self>, E: Env {
    //type LineIter: Iterator<Item=(Self::CharIter,Bounds)>;
    //type CharIter: Iterator<Item=Bounds>;

    fn size(&self) -> Dims;
    fn lines<'s>(&'s self) -> CrazyWorkaroundPPIter<'s>;

    fn generate(s: &str, size: (f32,f32)) -> Self; 
}

pub type CrazyWorkaroundPPIter<'a> = Box<dyn Iterator<Item=(Box<dyn Iterator<Item=Option<Bounds>>+'a>,Bounds)>+'a>;
