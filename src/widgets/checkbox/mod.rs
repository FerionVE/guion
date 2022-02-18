use super::*;
use super::util::LocalGlyphCache;
use super::label::Label;
use crate::text::stor::TextStor;
use crate::{event::key::Key, validation::Validation};
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

impl<'w,State,E> CheckBox<'w,E,State,Label<'w,E,&'static str,LocalGlyphCache<E>>,()> where
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
    pub fn with_update<T>(self, fun: T) -> CheckBox<'w,E,State,Text,T> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,bool) + Clone + Send + Sync + 'static {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            updater: fun,
            locked: self.locked,
            text: self.text,
            state: self.state,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn with_atomstate<T>(self, fun: T) -> CheckBox<'w,E,State,Text,impl TriggerMut<E>> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>) -> &'r mut (dyn AtomStateMut<E,bool>) + Clone + Send + Sync + 'static {
        self.with_update(move |r,x,c,v| fun(r,x,c).set(v,c) )
    }
    #[inline]
    pub fn with_caption<T>(self, text: T) -> CheckBox<'w,E,State,T,TrMut> where T: AsWidget<E> {
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

impl<'w,E,State,T,LC,TrMut> CheckBox<'w,E,State,Label<'w,E,T,LC>,TrMut> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> CheckBox<'w,E,State,Label<'w,E,TT,LC>,TrMut> where TT: TextStor<E>+Validation<E>+'w {
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

/// blanket-implemented on all `FnMut(&mut E::Context<'_>)`
pub trait TriggerMut<E> where E: Env {
    fn boxed(&self, value: bool) -> Option<BoxMutEvent<E>>;
}

impl<E> TriggerMut<E> for () where E: Env {
    #[inline]
    fn boxed(&self, _: bool) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TriggerMut<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,bool) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self, value: bool) -> Option<BoxMutEvent<E>> {
        let s = self.clone();
        Some(Box::new(move |r,x,c| s(r,x,c,value) ))
    }
}
