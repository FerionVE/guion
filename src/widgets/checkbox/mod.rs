use super::*;
use super::util::LocalGlyphCache;
use super::label::Label;
use crate::error::ResolveResult;
use crate::text::stor::TextStor;
use crate::validation::Validation;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorEnd, MutorTo};
use std::marker::PhantomData;
use util::state::*;

pub mod widget;
pub mod imp;

pub struct CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
    Self: 'w,
{
    pub updater: TrMut,
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    pub state: State,
    p: PhantomData<&'w (State,Text,TrMut)>,
}

impl<'w,State,E> CheckBox<'w,E,State,Label<'w,E,&'static str>,()> where
    E: Env,
    E::WidgetID: WidgetIDAlloc,
{
    #[inline]
    pub fn new(id: E::WidgetID, state: State) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: Default::default(),
            updater: (),
            locked: false,
            text: Label::new(E::WidgetID::new_id())
                .with_align((0.,0.5)),
            state,
            p: PhantomData,
        }
    }
}

impl<'w,E,State,Text,TrMut> CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
{
    

    #[inline]
    pub fn with_update<T>(self, mutor: T) -> CheckBox<'w,E,State,Text,T> where T: MutorEnd<bool,E> {
        CheckBox{
            id: self.id,
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
    pub fn with_atomstate<T>(self, mutor: T) -> CheckBox<'w,E,State,Text,impl MutorEnd<bool,E>>
    where
        T: MutorTo<(),E,Target=DynAtomStateMutTarget<bool>>
    {
        self.with_update(
            mutor.mutor_end_if((), |state,_,value,ctx| {
                //TODO ResolveResult handling
                state.set(value,ctx);
            })
        )
    }

    #[inline]
    pub fn with_caption<T>(self, text: T) -> CheckBox<'w,E,State,T,TrMut> {
        CheckBox{
            id: self.id,
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

impl<'w,E,State,T,TrMut> CheckBox<'w,E,State,Label<'w,E,T>,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> CheckBox<'w,E,State,Label<'w,E,TT>,TrMut> where TT: TextStor<E>+Validation<E>+'w {
        CheckBox{
            updater: self.updater,
            id: self.id,
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
