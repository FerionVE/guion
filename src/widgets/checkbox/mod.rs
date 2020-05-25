use super::*;
use crate::event::key::Key;
use std::marker::PhantomData;
use util::{state::*, caption::Caption};

pub mod imp;
pub mod trayt;

pub struct CheckBox<'w,E,State,Text> where
    E: Env,
    State: 'w,
    Text: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>,bool),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub border: Option<Border>,
    pub text: Text,
    pub state: State,
    p: PhantomData<&'w mut ()>,
}

impl<'w,State,E> CheckBox<'w,E,State,&'static str> where
    E: Env,
{
    pub fn new(id: E::WidgetID, state: State) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: vec![],
            trigger: |_,_|{},
            locked: false,
            border: None,
            text: "",
            state,
            p: PhantomData,
        }
    }
}

impl<'w,E,State,Text> CheckBox<'w,E,State,Text> where
    E: Env,
    Text: 'w,
{
    

    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>,bool)) -> Self {
        self.trigger = fun;
        self
    }
    pub fn with_text<T>(self, text: T) -> CheckBox<'w,E,State,T> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            locked: self.locked,
            border: self.border,
            text,
            state: self.state,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,State,Text> Statize<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: Statize<E>+'w, State::Statur: Sized,
    Text: Statize<E>+'w, Text::Statur: Sized,
{
    type Statur = CheckBox<'static,E,State::Statur,Text::Statur>;
}
