use std::marker::PhantomData;

use crate::aliases::{ESize, EStyle};
use crate::cachor::AsCachor;
use crate::newpath::PathStackDyn;
use crate::traitcast::WQuery;
use crate::widget_decl::mut_target::MuTarget;
use crate::widget_decl::mutor_trait::{MutorEndBuilder, MutorToBuilder, MutorToBuilderExt};
use crate::constraint;
use crate::env::Env;
use crate::text::stor::TextStor;

use self::decl::send_mutation_trigger_ty;

use super::label::decl::Label;

pub mod widget;
pub mod imp;
pub mod decl;

impl<E> decl::Button<E,Label<E,&'static str>,(),(),()> where
    E: Env,
{
    #[inline]
    pub fn new() -> Self {
        decl::Button {
            size: None,//constraint!(0|0).into(),
            style: None,//Default::default(),
            trigger: (),
            trigger_im: (),
            trigger_mut: (),
            locked: false,
            text: Label::new(),
        }
    }
}

impl<E,Text> decl::Button<E,Text,(),(),()> where
    E: Env,
{
    #[inline]
    pub fn of_text(text: Text) -> Self {
        Self{
            size: None,
            style: None,
            trigger: (),
            trigger_im: (),
            trigger_mut: (),
            locked: false,
            text,
        }
    }
}

impl<E,T> decl::Button<E,Label<E,T>,(),(),()> where
    E: Env, //TODO WidgetWithCaption with_text replace
    T: TextStor<E> + AsCachor<E>,
{
    #[inline]
    pub fn of_label_text(text: T) -> Self {
        decl::Button::of_text(Label::of_text(text))
    }
}

impl<E,Text,Tr,TrIm,TrMut> decl::Button<E,Text,Tr,TrIm,TrMut> where
    E: Env,
{
    #[inline]
    pub fn with_trigger<T>(self, trigger: T) -> decl::Button<E,Text,T,TrIm,TrMut> where T: Fn(&(dyn PathStackDyn<E>+'_),E::RootRef<'_>,&mut E::Context<'_>) + Clone + 'static {
        decl::Button {
            size: self.size,
            style: self.style,
            trigger,
            trigger_im: self.trigger_im,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text: self.text,
        }
    }
    #[inline]
    pub fn with_trigger_im<T>(self, immutor: T) -> decl::Button<E,Text,send_mutation_trigger_ty<E>,T,TrMut> where T: Fn(&(dyn PathStackDyn<E>+'_),E::RootRef<'_>,&mut E::Context<'_>) {
        decl::Button {
            size: self.size,
            style: self.style,
            trigger: decl::send_mutation_trigger::<E>,
            trigger_im: immutor,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text: self.text,
        }
    }
    #[inline]
    pub fn with_trigger_mut<T>(self, mutor: T) -> decl::Button<E,Text,send_mutation_trigger_ty<E>,TrIm,T> where T: MutorEndBuilder<(),E> {
        decl::Button {
            size: self.size,
            style: self.style,
            trigger: decl::send_mutation_trigger::<E>,
            trigger_im: self.trigger_im,
            trigger_mut: mutor,
            locked: self.locked,
            text: self.text,
        }
    }
    #[inline]
    pub fn with_trigger_mut_if<LeftMutor,LeftArgs,LeftTarget,RightFn>(self, left_mutor: LeftMutor, left_arg: LeftArgs, right_fn: RightFn) -> decl::Button<E,Text,send_mutation_trigger_ty<E>,TrIm,impl MutorEndBuilder<(),E>>
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
    pub fn with_caption<T>(self, text: T) -> decl::Button<E,T,Tr,TrIm,TrMut> {
        decl::Button {
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            trigger_im: self.trigger_im,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text,
        }
    }
    #[inline]
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    #[inline]
    pub fn with_size(mut self, size: ESize<E>) -> Self {
        self.size = Some(size);
        self
    }
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = Some(style);
        self
    }
}

impl<E,T,Tr,TrIm,TrMut> decl::Button<E,Label<E,T>,Tr,TrIm,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> decl::Button<E,Label<E,TT>,Tr,TrIm,TrMut> where TT: TextStor<E> + AsCachor<E> {
        decl::Button {
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            trigger_im: self.trigger_im,
            trigger_mut: self.trigger_mut,
            locked: self.locked,
            text: self.text.with_text(text),
        }
    }
}

/// blanket-implemented on all `Fn(Link<E>)`
pub trait Trigger<E> where E: Env {
    fn trigger(&self, path: &(dyn PathStackDyn<E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>);
}

impl<E> Trigger<E> for () where E: Env {
    #[inline]
    fn trigger(&self, _: &(dyn PathStackDyn<E>+'_), _: E::RootRef<'_>, _: &mut E::Context<'_>) {}
}

impl<T,E> Trigger<E> for T where T: Fn(&(dyn PathStackDyn<E>+'_),E::RootRef<'_>,&mut E::Context<'_>), E: Env {
    #[inline]
    fn trigger(&self, path: &(dyn PathStackDyn<E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        (self)(path, root, ctx)
    }
}

impl<E> WQuery<E> for dyn Trigger<E> where E: Env {
    type Result<'a> = &'a (dyn Trigger<E> + 'a);
}
