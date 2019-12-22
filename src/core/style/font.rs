use crate::core::*;
use util::bounds::Offset;
use util::bounds::Dims;
use super::*;

pub trait Font<S,C>: Sized where S: Style<C,Font=Self>, C: Context<Style=S> {
    
}

pub trait PreprocessedText<S,C>: Sized where S: Style<C,PreprocessedText=Self>, C: Context<Style=S> {
    fn size(&self) -> Dims;
    fn style(&self) -> &S; //TODO TextCache validates invalidate state and compares style
    fn chars(&self) -> &[S::PreprocessedChar];
    fn back(&self) -> String;
}

pub trait PreprocessedChar {
    fn offset(&self) -> Offset;
    fn char(&self) -> char;
}