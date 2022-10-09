use super::*;
use super::util::LocalGlyphCache;
use super::label::Label;
use crate::error::ResolveResult;
use crate::text::stor::TextStor;
use crate::validation::Validation;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorEnd, MutorTo, MutorEndBuilderDyn, MutorToBuilderDyn, MutorEndBuilder, MutorToBuilder, MutorToBuilderExt, MutorEndBuilderExt};
use std::marker::PhantomData;
use util::state::*;

pub mod widget;
pub mod imp;

pub struct CheckBox<'w,E,State,Text> where
    E: Env,
    Self: 'w,
{
    pub updater: CheckBoxUpdater<'w,E>,
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    pub state: State,
    p: PhantomData<&'w (State,Text)>,
}

impl<'w,State,E> CheckBox<'w,E,State,Label<'w,E,&'static str>,> where
    E: Env,
    E::WidgetID: WidgetIDAlloc,
{
    #[inline]
    pub fn new(id: E::WidgetID, state: State) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: Default::default(),
            updater: CheckBoxUpdater::None,
            locked: false,
            text: Label::new(E::WidgetID::new_id())
                .with_align((0.,0.5)),
            state,
            p: PhantomData,
        }
    }
}

impl<'w,E,State,Text> CheckBox<'w,E,State,Text> where
    E: Env,
{
    

    #[inline]
    pub fn with_update<T>(self, mutor: &'w T) -> CheckBox<'w,E,State,Text> where T: MutorEndBuilder<bool,E> {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            updater: CheckBoxUpdater::Apply(mutor.erase()),
            locked: self.locked,
            text: self.text,
            state: self.state,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_atomstate<T>(self, mutor: &'w T) -> CheckBox<'w,E,State,Text>
    where
        T: MutorToBuilder<(),DynAtomStateMutTarget<bool>,E>
    {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            updater: CheckBoxUpdater::Atomstate(mutor.erase()),
            locked: self.locked,
            text: self.text,
            state: self.state,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_caption<T>(self, text: T) -> CheckBox<'w,E,State,T> {
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

impl<'w,E,State,T> CheckBox<'w,E,State,Label<'w,E,T>> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> CheckBox<'w,E,State,Label<'w,E,TT>> where TT: TextStor<E>+Validation<E>+'w {
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

pub enum CheckBoxUpdater<'w,E> where E: Env {
    None,
    Apply(&'w (dyn MutorEndBuilderDyn<bool,E>+'w)),
    Atomstate(&'w (dyn MutorToBuilderDyn<(),DynAtomStateMutTarget<bool>,E>+'w)),
}

impl<'w,E> CheckBoxUpdater<'w,E> where E: Env {
    fn submit_update(&self, update: bool, ctx: &mut E::Context<'_>) -> bool {
        match self {
            CheckBoxUpdater::None => {return false;},
            &CheckBoxUpdater::Apply(x) => ctx.mutate_closure(x.build_box_mut_event(update)),
            &CheckBoxUpdater::Atomstate(x) => ctx.mutate_closure(
                x.mutor_end_if((), |state,_,v,ctx| {
                    state.set(v,ctx);
                }).build_box_mut_event(update)
            ),
        }
        true
    }
}
