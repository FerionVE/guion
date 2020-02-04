use super::*;

pub trait Font<E>: Sized where EStyle<E>: Style<E,Font=Self>, E: Env {
    
}

/// Text in a optimized form for faster frequent rendering and iterating
pub trait PreprocessedText<E>: Sized where EStyle<E>: Style<E,PreprocessedText=Self>, E: Env {
    fn size(&self) -> Dims;
    fn style(&self) -> &EStyle<E>; //TODO TextCache validates invalidate state and compares style
    fn chars(&self) -> &[ESPPChar<E>];
    fn back(&self) -> String;
}

pub trait PreprocessedChar {
    fn offset(&self) -> Offset;
    fn char(&self) -> char;
}