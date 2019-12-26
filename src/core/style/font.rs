use super::*;

pub trait Font<S,E>: Sized where S: Style<E,Font=Self>, E: Env, E::Backend: Backend<E,Style=S> {
    
}

pub trait PreprocessedText<S,E>: Sized where S: Style<E,PreprocessedText=Self>, E: Env, E::Backend: Backend<E,Style=S> {
    fn size(&self) -> Dims;
    fn style(&self) -> &S; //TODO TextCache validates invalidate state and compares style
    fn chars(&self) -> &[S::PreprocessedChar];
    fn back(&self) -> String;
}

pub trait PreprocessedChar {
    fn offset(&self) -> Offset;
    fn char(&self) -> char;
}