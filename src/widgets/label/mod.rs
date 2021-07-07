use crate::text::stor::TextStor;
use crate::validation::Validator;
use crate::validation::imp::MirrorValidated;

use super::*;
use super::util::state::AtomState;
use std::marker::PhantomData;
use util::{LocalGlyphCache, remote_state::RemoteState};

pub mod widget;

pub struct Label<'w,E,Text,GlyphCache,TextValidator> where
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
    p: PhantomData<&'w mut &'w TextValidator>,
}

impl<'w,E> Label<'w,E,&'static str,LocalGlyphCache<E,String>,MirrorValidated> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self { //TODO: MirrorValidated is not a generic default
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

impl<'w,E,Text> Label<'w,E,Text,RemoteState<E,LocalGlyphCache<E,Text::Owned>>,MirrorValidated> where
    E: Env,
    E::Context: DynState<E>,
    Text: TextStor<E>+ToOwned+'w,
    Text::Owned: Clone,
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

impl<'w,E,Text,GlyphCache,TextValidator> Label<'w,E,Text,GlyphCache,TextValidator> where
    E: Env,
    Text: 'w,
    GlyphCache: 'w,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> Label<'w,E,T,GlyphCache,TextValidator> where T: TextStor<E>+'w {
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

    pub fn with_text_validator<TV,GC>(self) -> Label<'w,E,Text,GlyphCache,TV> where TV: Validator<Text,E>, GC: AtomState<E,LocalGlyphCache<E,TV::Cache>>+Clone {
        Label{
            id: self.id,
            size: self.size,
            style: self.style,
            text: self.text,
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
