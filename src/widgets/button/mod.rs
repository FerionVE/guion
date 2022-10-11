use super::*;
use crate::text::stor::TextStor;
use crate::view::mutor_trait::MutorEndBuilder;
use crate::{event::key::Key, validation::Validation};
use std::marker::PhantomData;
use util::LocalGlyphCache;
use label::Label;

pub mod widget;
pub mod imp;

pub struct Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Self: 'w,
{
    pub trigger: Tr,
    pub trigger_mut: TrMut,
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    p: PhantomData<&'w (Text,Tr,TrMut)>,
}

impl<'w,E> Button<'w,E,Label<'w,E,&'static str>,(),()> where
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
{
    #[inline]
    pub fn with_trigger<T>(self, mutor: T) -> Button<'w,E,Text,T,TrMut> where T: Fn(E::RootRef<'_>,&mut E::Context<'_>) {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: mutor,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text: self.text,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_trigger_mut<T>(self, mutor: T) -> Button<'w,E,Text,Tr,T> where T: MutorEndBuilder<(),E> {
        Button{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            trigger_mut: mutor,
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
    pub fn with_size(mut self, size: ESize<E>) -> Self {
        self.size = size;
        self
    }
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }
}

impl<'w,E,T,Tr,TrMut> Button<'w,E,Label<'w,E,T>,Tr,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> Button<'w,E,Label<'w,E,TT>,Tr,TrMut> where TT: TextStor<E>+Validation<E>+'w {
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
    fn trigger(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>);
}

impl<E> Trigger<E> for () where E: Env {
    #[inline]
    fn trigger(&self, _: E::RootRef<'_>, _: &mut E::Context<'_>) {}
}

impl<T,E> Trigger<E> for T where T: Fn(E::RootRef<'_>,&mut E::Context<'_>), E: Env {
    #[inline]
    fn trigger(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        (self)(root,ctx)
    }
}

traitcast_for_from_widget!(Trigger<E>);
