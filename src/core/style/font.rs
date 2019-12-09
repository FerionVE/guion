use crate::core::util::bounds::Offset;
use crate::core::util::bounds::Dims;
use super::*;

pub trait Font<S>: Sized where S: Style<Font=Self> {
    
}

pub trait PreprocessedText<S>: Sized where S: Style<PreprocessedText=Self> {
    fn size(&self, s: &str) -> Dims;
    fn style(&self) -> &S; //TODO TextCache validates invalidate state and compares style
    fn chars(&self) -> [S::PreprocessedChar];
    fn back(&self) -> String;
}

pub trait PreprocessedChar {
    fn offset(&self) -> Offset;
    fn char(&self) -> char;
}