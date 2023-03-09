use std::marker::PhantomData;

use crate::aliases::{ESize, EStyle, ETextLayout};
use crate::cachor::AsCachor;
use crate::env::Env;
use crate::layout::Gonstraints;
use crate::text::stor::TextStor;

pub mod widget;
pub mod decl;



impl<E> decl::Label<E,&'static str> where
    E: Env,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            size: None,
            style: None,
            text: "",
            align: None,
        }
    }
}

impl<E,Text> decl::Label<E,Text> where
    E: Env,
    Text: TextStor<E> + AsCachor<E>,
{
    #[inline]
    pub fn of_text(text: Text) -> Self {
        Self{
            size: None,
            style: None,
            text,
            align: None,
        }
    }
}

impl<E,Text> decl::Label<E,Text> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> decl::Label<E,T> where T: TextStor<E> + AsCachor<E> {
        decl::Label {
            size: self.size,
            style: self.style,
            text,
            align: self.align,
        }
    }

    #[inline]
    pub fn with_align(mut self, align: (f32,f32)) -> Self {
        self.align = Some(align);
        self
    }
    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = Some(s);
        self
    }
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self where EStyle<E>: PartialEq {
        self.style = Some(style);
        self
    }
}
