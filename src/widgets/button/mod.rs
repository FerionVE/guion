use super::*;
use crate::event::key::Key;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod imp;

pub struct Button<'w,E,Text> where
    E: Env,
    Text: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdTag>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub border: Option<Border>,
    pub text: Text,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> Button<'w,E,&'static str> where
    E: Env,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: vec![],
            trigger: |_|{},
            locked: false,
            border: None,
            text: "",
            p: PhantomData,
        }
    }
}

impl<'w,E,Text> Button<'w,E,Text> where
    E: Env,
    Text: 'w,
{
    

    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>)) -> Self {
        self.trigger = fun;
        self
    }
    pub fn with_text<T>(self, text: T) -> Button<'w,E,T> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            locked: self.locked,
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

unsafe impl<'w,E,Text> Statize<E> for Button<'w,E,Text> where
    E: Env,
    Text: Caption<'w>+StatizeSized<E>
{
    type Statur = Button<'static,E,Text::StaturSized>;
}
