use std::marker::PhantomData;

use crate::aliases::{ESize, EStyle};
use crate::cachor::AsCachor;
use crate::traitcast::WQuery;
use crate::view::mut_target::MuTarget;
use crate::view::mutor_trait::{MutorEndBuilder, MutorToBuilder, MutorToBuilderExt};
use crate::constraint;
use crate::env::Env;
use crate::text::stor::TextStor;

use super::label::Label;

pub mod widget;
pub mod imp;

pub struct Button<E,Text,Tr,TrMut> where
    E: Env,
{
    pub trigger: Tr,
    pub trigger_mut: TrMut,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    p: PhantomData<()>,
}

impl<E> Button<E,Label<E,&'static str>,(),()> where
    E: Env,
{
    #[inline]
    pub fn new() -> Self {
        Button{
            size: constraint!(0|0).into(),
            style: Default::default(),
            trigger: (),
            trigger_mut: (),
            locked: false,
            text: Label::new(),
            p: PhantomData,
        }
    }
}

impl<E,Text> Button<E,Text,(),()> where
    E: Env,
{
    #[inline]
    pub fn of_text(text: Text) -> Self {
        Self{
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

impl<E,T> Button<E,Label<E,T>,(),()> where
    E: Env, //TODO WidgetWithCaption with_text replace
    T: TextStor<E> + AsCachor<E>,
{
    #[inline]
    pub fn of_label_text(text: T) -> Self {
        Button::of_text(Label::of_text(text))
    }
}

impl<E,Text,Tr,TrMut> Button<E,Text,Tr,TrMut> where
    E: Env,
{
    #[inline]
    pub fn with_trigger<T>(self, mutor: T) -> Button<E,Text,T,TrMut> where T: Fn(E::RootRef<'_>,&mut E::Context<'_>) {
        Button{
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
    pub fn with_trigger_mut<T>(self, mutor: T) -> Button<E,Text,Tr,T> where T: MutorEndBuilder<(),E> {
        Button{
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
    pub fn with_trigger_mut_if<LeftMutor,LeftArgs,LeftTarget,RightFn>(self, left_mutor: LeftMutor, left_arg: LeftArgs, right_fn: RightFn) -> Button<E,Text,Tr,impl MutorEndBuilder<(),E>>
    where 
        LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
        LeftTarget: MuTarget<E> + ?Sized,
        LeftArgs: Clone + Sized + Send + Sync + 'static,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut LeftTarget::Mutable<'ss>,
            (),
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        self.with_trigger_mut(
            left_mutor.mutor_end_if(left_arg, right_fn)
        )
    }
    #[inline]
    pub fn with_caption<T>(self, text: T) -> Button<E,T,Tr,TrMut> {
        Button{
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

impl<E,T,Tr,TrMut> Button<E,Label<E,T>,Tr,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> Button<E,Label<E,TT>,Tr,TrMut> where TT: TextStor<E> + AsCachor<E> {
        Button{
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

impl<E> WQuery<E> for dyn Trigger<E> where E: Env {
    type Result<'a> = &'a (dyn Trigger<E> + 'a);
}
