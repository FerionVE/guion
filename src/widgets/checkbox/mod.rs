use std::marker::PhantomData;

use crate::aliases::{ESize, EStyle};
use crate::env::Env;
use crate::layout::Gonstraints;
use crate::text::stor::TextStor;
use crate::validation::Validation;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorToBuilder, MutorEndBuilder, MutorToBuilderExt};

use super::label::Label;

pub mod widget;
pub mod imp;

pub struct CheckBox<E,State,Text,TrMut> where
    E: Env,
{
    pub updater: TrMut,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    pub state: State,
    p: PhantomData<()>,
}

impl<State,E> CheckBox<E,State,Label<E,&'static str>,()> where
    E: Env,
{
    #[inline]
    pub fn new(state: State) -> Self {
        Self{
            size: ESize::<E>::empty(),
            style: Default::default(),
            updater: (),
            locked: false,
            text: Label::new()
                .with_align((0.,0.5)),
            state,
            p: PhantomData,
        }
    }
}

impl<E,State,Text,TrMut> CheckBox<E,State,Text,TrMut> where
    E: Env,
{
    #[inline]
    pub fn with_update<T>(self, mutor: T) -> CheckBox<E,State,Text,T> where T: MutorEndBuilder<bool,E> {
        CheckBox{
            size: self.size,
            style: self.style,
            updater: mutor,
            locked: self.locked,
            text: self.text,
            state: self.state,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_atomstate<T>(self, mutor: T) -> CheckBox<E,State,Text,impl MutorEndBuilder<bool,E>>
    where
        T: MutorToBuilder<(),DynAtomStateMutTarget<bool>,E>
    {
        self.with_update(
            mutor.mutor_end_if((), |state,_,value,ctx| {
                //TODO ResolveResult handling
                state.set(value,ctx);
            })
        )
    }

    #[inline]
    pub fn with_caption<T>(self, text: T) -> CheckBox<E,State,T,TrMut> {
        CheckBox{
            size: self.size,
            style: self.style,
            updater: self.updater,
            locked: self.locked,
            text,
            state: self.state,
            p: PhantomData,
        }
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

impl<E,State,T,TrMut> CheckBox<E,State,Label<E,T>,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> CheckBox<E,State,Label<E,TT>,TrMut> where TT: TextStor<E>+Validation<E> {
        CheckBox{
            updater: self.updater,
            size: self.size,
            style: self.style,
            locked: self.locked,
            text: self.text.with_text(text),
            state: self.state,
            p: PhantomData,
        }
    }
}

//TODO bring the immutable trigger like in Button
