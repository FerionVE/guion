use super::*;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod imp;

pub struct Label<'w,E,S> where
    E: Env,
    S: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdTag>,
    pub border: Option<Border>,
    pub text: S,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> Label<'w,E,&'static str> where
    E: Env,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
            text: "",
            p: PhantomData,
        }
    }
}

impl<'w,E,S> Label<'w,E,S> where
    E: Env,
    S: 'w,
{
    pub fn with_text<T>(self, text: T) -> Label<'w,E,T> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        Label{
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

unsafe impl<'w,E,S> Statize<E> for Label<'w,E,S> where
    E: Env,
    S: StatizeSized<E>,
{
    type Statur = Label<'static,E,S::StaturSized>;
}
