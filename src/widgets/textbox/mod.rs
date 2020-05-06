use super::*;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod imp;

pub struct TextBox<'w,E,S> where
    E: Env,
    S: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub text: S,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> TextBox<'w,E,String> where
    E: Env,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
            text: "".to_owned(),
            p: PhantomData,
        }
    }
}

impl<'w,E,S> TextBox<'w,E,S> where
    E: Env,
    S: 'w,
{
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T> where T: Caption<'w>+Statize, T::Statur: Sized {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,S> Statize for TextBox<'w,E,S> where
    E: Env,
    S: Statize, S::Statur: Sized,
{
    type Statur = TextBox<'static,E,S::Statur>;
}
