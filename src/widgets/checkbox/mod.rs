use super::*;
use crate::event::key::Key;
use std::marker::PhantomData;
use util::{state::*, caption::Caption};

pub mod widget;
pub mod imp;

pub struct CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    State: 'w,
    Text: 'w,
    Stil: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>,bool),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    pub state: State,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,State,E> CheckBox<'w,E,State,&'static str,()> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID, state: State) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: (),
            trigger: |_,_|{},
            locked: false,
            text: "",
            state,
            p: PhantomData,
        }
    }
}

impl<'w,E,State,Text,Stil> CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    Text: 'w,
{
    

    #[inline]
    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>,bool)) -> Self {
        self.trigger = fun;
        self
    }
    #[inline]
    pub fn with_text<T>(self, text: T) -> CheckBox<'w,E,State,T,Stil> where T: Caption<'w>+StatizeSized<E> {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            locked: self.locked,
            text,
            state: self.state,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    #[inline]
    pub fn with_style<SStil>(self, style: SStil) -> CheckBox<'w,E,State,Text,SStil> where SStil: 'w {
        CheckBox{
            trigger: self.trigger,
            id: self.id,
            size: self.size,
            style,
            locked: self.locked,
            text: self.text,
            state: self.state,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,State,Text,Stil> Statize<E> for CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    State: StatizeSized<E>+'w,
    Text: StatizeSized<E>+'w,
    Stil: StatizeSized<E>+'w,
{
    type Statur = CheckBox<'static,E,State::StaturSized,Text::StaturSized,Stil::StaturSized>;
}
