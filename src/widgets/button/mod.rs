use super::*;
use crate::core::event::key::Key;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod imp;

pub struct Button<'w,E,S> where
    E: Env,
    S: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub border: Option<Border>,
    pub text: S,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E> Button<'w,E,&'static str> where
    E: Env,
{
    pub fn new(id: E::WidgetID, size: ESize<E>) -> Self {
        Self{
            id,
            size,
            style: vec![],
            trigger: |_|{},
            locked: false,
            border: None,
            text: "",
            p: PhantomData,
        }
    }
}

impl<'w,E,S> Button<'w,E,S> where
    E: Env,
    S: 'w,
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

unsafe impl<'w,E,S> Statize<E> for Button<'w,E,S> where
    E: Env,
    S: Caption<'w>+Statize<E>,
    S::Statur: Sized,
{
    type Statur = Button<'static,E,S::Statur>;
}
