use crate::text::stor::TextStor;
use crate::validation::Validation;

use super::*;
use std::marker::PhantomData;
use util::{LocalGlyphCache, remote_state::RemoteState};

pub mod widget;

pub struct Label<'w,E,Text,GlyphCache> where
    E: Env,
    Text: 'w,
    GlyphCache: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub align: (f32,f32),
    pub glyph_cache: GlyphCache,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> Label<'w,E,&'static str,LocalGlyphCache<E>> where
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
            glyph_cache: None,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text> Label<'w,E,Text,RemoteState<E,LocalGlyphCache<E>>> where
    E: Env,
    Text: TextStor<E>+Validation<E>+'w,
{
    #[inline]
    pub fn immediate(id: E::WidgetID, text: Text) -> Self {
        Self{
            id: id.clone(),
            size: ESize::<E>::empty(),
            style: Default::default(),
            text,
            align: (0.5,0.5),
            glyph_cache: RemoteState::for_widget(id),
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,GlyphCache> Label<'w,E,Text,GlyphCache> where
    E: Env,
    Text: 'w,
    GlyphCache: 'w,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> Label<'w,E,T,GlyphCache> where T: TextStor<E>+Validation<E>+'w {
        Label{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            align: self.align,
            glyph_cache: self.glyph_cache,
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
