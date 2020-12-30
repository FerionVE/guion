use super::*;
use crate::{event::key::Key, validation::Validation};
use std::marker::PhantomData;
use util::{LocalGlyphCache, caption::Caption};
use label::Label;

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
    pub text: Text,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> Button<'w,E,Label<'w,E,&'static str,(),LocalGlyphCache<E>>,()> where
    E: Env,
    E::WidgetID: WidgetIDAlloc,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: constraint!(0|0).into(),
            style: (),
            trigger: |_|{},
            locked: false,
            text: Label::new(E::WidgetID::new_id()),
            p: PhantomData,
        }
    }
}

impl<'w,E,Text> Button<'w,E,Text,()> where
    E: Env,
    Text: 'w,
{
    #[inline]
    pub fn immediate(id: E::WidgetID, text: Text) -> Self {
        Self{
            id,
            size: constraint!(0|0).into(),
            style: (),
            trigger: |_|{},
            locked: false,
            text,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Stil> Button<'w,E,Text,Stil> where
    E: Env,
    Text: 'w,
    Stil: 'w
{
    

    #[inline]
    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>)) -> Self {
        self.trigger = fun;
        self
    }
    /*#[inline]
    pub fn with_text<T>(self, text: T) -> Button<'w,E,T,Stil> where T: 'w {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            locked: self.locked,
            text,
            p: PhantomData,
        }
    }*/
    #[inline]
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    #[inline]
    pub fn with_style<SStil>(self, style: SStil) -> Button<'w,E,Text,SStil> where SStil: 'w, ESVariant<E>: for<'z> StyleVariantSupport<&'z Stil> {
        Button{
            trigger: self.trigger,
            id: self.id,
            size: self.size,
            style: style,
            locked: self.locked,
            text: self.text,
            p: PhantomData,
        }
    }
}

impl<'w,E,T,LS,BS,LC> Button<'w,E,Label<'w,E,T,LS,LC>,BS> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> Button<'w,E,Label<'w,E,TT,LS,LC>,BS> where TT: Caption<E>+Validation<E>+'w {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            locked: self.locked,
            text: self.text.with_text(text),
            p: PhantomData,
        }
    }
}

pub trait TriggerFn<E,W> where E: Env {
    fn call(&mut self, w: &W, l: Link<E>);
}
impl<T,E,W> TriggerFn<E,W> for T where T: FnMut(&W,Link<E>), E: Env {
    #[inline]
    fn call(&mut self, w: &W, l: Link<E>) {
        self(w,l)
    }
}
impl<E,W> TriggerFn<E,W> for () where E: Env {
    #[inline]
    fn call(&mut self, _: &W, _: Link<E>) {}
}
