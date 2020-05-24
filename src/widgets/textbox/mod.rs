use super::*;
use std::marker::PhantomData;
use util::caption::Caption;
use state::Cursor;

pub mod imp;
pub mod state;

pub struct TextBox<'w,E,S,P,C,V> where
    E: Env,
    S: 'w,
    P: 'w,
    C: 'w,
    V: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub text: S,
    pub scroll: P,
    pub cursor: C,
    pub validation: V,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),Cursor,bool> where
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
            validation: false, //would work perfectly on owned, for immediate state-stored AtomStateX can be used
            p: PhantomData,
        }
    }
}

impl<'w,E,S,P,C,V> TextBox<'w,E,S,P,C,V> where
    E: Env,
    S: 'w,
    P: 'w,
    C: 'w,
    V: 'w,
{
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,P,C,V> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            validation: self.validation,
            p: PhantomData,
        }
    }

    pub fn with_states<PP,CC>(self, scroll: PP, cursor: CC) -> TextBox<'w,E,S,PP,CC,V> where PP: Statize<E>+'w, CC: Statize<E>+'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text: self.text,
            scroll,
            cursor,
            validation: self.validation,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,S,P,C,V> Statize<E> for TextBox<'w,E,S,P,C,V> where
    E: Env,
    S: Statize<E>, S::Statur: Sized,
    P: Statize<E>, P::Statur: Sized,
    C: Statize<E>, C::Statur: Sized,
    V: Statize<E>, V::Statur: Sized,
{
    type Statur = TextBox<'static,E,S::Statur,P::Statur,C::Statur,V::Statur>;
}
