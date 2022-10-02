use crate::text::stor::TextStor;
use crate::validation::Validation;

use super::*;
use std::marker::PhantomData;
use util::{LocalGlyphCache, remote_state::RemoteState};

pub mod widget;

pub struct Label<'w,E,Text> where
    E: Env,
    Self: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub align: (f32,f32),
    p: PhantomData<&'w Text>,
}

impl<'w,E> Label<'w,E,&'static str> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: Default::default(),
            text: "",
            align: (0.5,0.5),
            p: PhantomData,
        }
    }
}

impl<'w,E,Text> Label<'w,E,Text> where
    E: Env,
    Text: TextStor<E>+Validation<E>,
{
    #[inline]
    pub fn immediate(id: E::WidgetID, text: Text) -> Self {
        Self{
            id: id.clone(),
            size: ESize::<E>::empty(),
            style: Default::default(),
            text,
            align: (0.5,0.5),
            p: PhantomData,
        }
    }
}

impl<'w,E,Text> Label<'w,E,Text> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> Label<'w,E,T> where T: TextStor<E>+Validation<E>+'w {
        Label{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            align: self.align,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_align(mut self, align: (f32,f32)) -> Self {
        self.align = align;
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
