use super::*;
use crate::text::stor::TextStor;
use crate::{event::key::Key, validation::Validation};
use std::marker::PhantomData;
use util::LocalGlyphCache;
use label::Label;

pub mod widget;
pub mod imp;

pub struct Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Text: 'w,
{
    pub trigger: Tr,
    pub trigger_mut: TrMut,
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> Button<'w,E,Label<'w,E,&'static str,LocalGlyphCache<E>>,(),()> where
    E: Env,
    E::WidgetID: WidgetIDAlloc,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Button{
            id,
            size: constraint!(0|0).into(),
            style: Default::default(),
            trigger: (),
            trigger_mut: (),
            locked: false,
            text: Label::new(E::WidgetID::new_id()),
            p: PhantomData,
        }
    }
}

impl<'w,E,Text> Button<'w,E,Text,(),()> where
    E: Env,
    Text: 'w,
{
    #[inline]
    pub fn immediate(id: E::WidgetID, text: Text) -> Self {
        Self{
            id,
            size: constraint!(0|0).into(),
            style: Default::default(),
            trigger: (),
            trigger_mut: (),
            locked: false,
            text,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Tr,TrMut> Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Text: 'w,
{
    #[inline]
    pub fn with_trigger<T>(self, fun: T) -> Button<'w,E,Text,T,TrMut> where T: Fn(Link<E>) {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: fun,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text: self.text,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_trigger_mut<T>(self, fun: T) -> Button<'w,E,Text,Tr,T> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>) + Clone + Send + Sync + 'static {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            trigger_mut: fun,
            locked: self.locked,
            text: self.text,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_caption<T>(self, text: T) -> Button<'w,E,T,Tr,TrMut> where T: 'w {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text,
            p: PhantomData,
        }
    }
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
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }
}

impl<'w,E,T,LC,Tr,TrMut> Button<'w,E,Label<'w,E,T,LC>,Tr,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> Button<'w,E,Label<'w,E,TT,LC>,Tr,TrMut> where TT: TextStor<E>+Validation<E>+'w {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text: self.text.with_text(text),
            p: PhantomData,
        }
    }
}

/// blanket-implemented on all `Fn(Link<E>)`
pub trait Trigger<E> where E: Env {
    fn trigger(&self, l: Link<E>);
}

impl<E> Trigger<E> for () where E: Env {
    #[inline]
    fn trigger(&self, _: Link<E>) {}
}

impl<T,E> Trigger<E> for T where T: Fn(Link<E>), E: Env {
    #[inline]
    fn trigger(&self, l: Link<E>) {
        (self)(l)
    }
}

/// blanket-implemented on all `FnMut(&mut E::Context<'_>)`
pub trait TriggerMut<E> where E: Env {
    fn boxed(&self) -> Option<BoxMutEvent<E>>;
}

impl<E> TriggerMut<E> for () where E: Env {
    #[inline]
    fn boxed(&self) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TriggerMut<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self) -> Option<BoxMutEvent<E>> {
        Some(Box::new(self.clone()))
    }
}

traitcast_for_from_widget!(Trigger<E>);
