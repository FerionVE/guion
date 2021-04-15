use super::*;
use super::util::LocalGlyphCache;
use super::label::Label;
use crate::text::stor::TextStor;
use crate::{event::key::Key, validation::Validation};
use std::marker::PhantomData;
use util::{state::*, caption::Caption};

pub mod widget;
pub mod imp;

pub struct CheckBox<'w,E,State,Text> where
    E: Env,
    State: 'w,
    Text: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>,bool),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub text: Text,
    pub state: State,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,State,E> CheckBox<'w,E,State,Label<'w,E,&'static str,LocalGlyphCache<E>>> where
    E: Env,
    E::WidgetID: WidgetIDAlloc,
{
    #[inline]
    pub fn new(id: E::WidgetID, state: State) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: Default::default(),
            trigger: |_,_|{},
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
    Text: 'w,
{
    

    #[inline]
    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>,bool)) -> Self {
        self.trigger = fun;
        self
    }
    #[inline]
    pub fn with_caption<T>(self, text: T) -> CheckBox<'w,E,State,T> where T: AsWidget<E> {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
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

impl<'w,E,State,T,LC> CheckBox<'w,E,State,Label<'w,E,T,LC>> where
    E: Env, //TODO WidgetWithCaption with_text replace
{
    #[inline]
    pub fn with_text<TT>(self, text: TT) -> CheckBox<'w,E,State,Label<'w,E,TT,LC>> where TT: TextStor<E>+Validation<E>+'w {
        CheckBox{
            trigger: self.trigger,
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
