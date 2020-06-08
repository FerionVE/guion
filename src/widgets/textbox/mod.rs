use super::*;
use std::marker::PhantomData;
use util::caption::Caption;
use state::Cursor;

pub mod imp;
pub mod state;

pub struct TextBox<'w,E,S,P,C,X,V> where
    E: Env,
    S: 'w,
    P: 'w,
    C: 'w,
    X: 'w,
    V: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub text: S,
    pub scroll: P,
    pub cursor: C,
    pub cursor_stick_x: X,
    pub validation: V,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),Cursor,Option<u32>,bool> where
    E: Env,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
            text: "".to_owned(),
            scroll: (0,0),
            cursor: Cursor{select: 0, caret: 0}, //TODO default trait
            cursor_stick_x: None,
            validation: false, //would work perfectly on owned, for immediate state-stored AtomStateX can be used
            p: PhantomData,
        }
    }
}

impl<'w,E,S,P,C,X,V> TextBox<'w,E,S,P,C,X,V> where
    E: Env,
    S: 'w,
    P: 'w,
    C: 'w,
    X: 'w,
    V: 'w,
{
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,P,C,X,V> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            cursor_stick_x: self.cursor_stick_x,
            validation: self.validation,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    pub fn with_states<PP,CC,XX>(self, scroll: PP, cursor: CC, cursor_stick_x: XX) -> TextBox<'w,E,S,PP,CC,XX,V> where PP: Statize<E>+'w, CC: Statize<E>+'w, XX: Statize<E>+'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text: self.text,
            scroll,
            cursor,
            cursor_stick_x,
            validation: self.validation,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,S,P,C,X,V> Statize<E> for TextBox<'w,E,S,P,C,X,V> where
    E: Env,
    S: Statize<E>, S::Statur: Sized,
    P: Statize<E>, P::Statur: Sized,
    C: Statize<E>, C::Statur: Sized,
    X: Statize<E>, X::Statur: Sized,
    V: Statize<E>, V::Statur: Sized,
{
    type Statur = TextBox<'static,E,S::Statur,P::Statur,C::Statur,X::Statur,V::Statur>;
}
