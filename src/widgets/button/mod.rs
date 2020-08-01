use super::*;
use crate::event::key::Key;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod widget;

pub struct Button<'w,E,Text,Stil> where
    E: Env,
    Text: 'w,
    Stil: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub border: Option<Border>,
    pub text: Text,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> Button<'w,E,&'static str,()> where
    E: Env,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: (),
            trigger: |_|{},
            locked: false,
            border: None,
            text: "",
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Stil> Button<'w,E,Text,Stil> where
    E: Env,
    Text: 'w,
{
    

    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>)) -> Self {
        self.trigger = fun;
        self
    }
    pub fn with_text<T>(self, text: T) -> Button<'w,E,T,Stil> where T: 'w {
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

unsafe impl<'w,E,Text,Stil> Statize<E> for Button<'w,E,Text,Stil> where
    E: Env,
    Text: StatizeSized<E>+'w,
    Stil: StatizeSized<E>+'w,
{
    type Statur = Button<'static,E,Text::StaturSized,Stil::StaturSized>;
}
