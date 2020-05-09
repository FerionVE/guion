use super::*;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod imp;
pub mod state;

pub struct TextBox<'w,E,S,P,C> where
    E: Env,
    S: 'w,
    P: 'w,
    C: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub text: S,
    pub scroll: P,
    pub cursor: C,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),u32> where
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
            cursor: 0,
            p: PhantomData,
        }
    }
}

impl<'w,E,S,P,C> TextBox<'w,E,S,P,C> where
    E: Env,
    S: 'w,
    P: 'w,
    C: 'w,
{
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,P,C> where T: Caption<'w>+Statize, T::Statur: Sized {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,S,P,C> Statize for TextBox<'w,E,S,P,C> where
    E: Env,
    S: Statize, S::Statur: Sized,
    P: Statize, P::Statur: Sized,
    C: Statize, C::Statur: Sized,
{
    type Statur = TextBox<'static,E,S::Statur,P::Statur,C::Statur>;
}
