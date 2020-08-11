use super::*;
use std::marker::PhantomData;
use util::{LocalGlyphCache, caption::Caption};

pub mod widget;

pub struct Label<'w,E,Text,Stil,GlyphCache> where
    E: Env,
    Text: 'w,
    Stil: 'w,
    GlyphCache: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub text: Text,
    pub align: (f32,f32),
    pub glyph_cache: GlyphCache,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> Label<'w,E,&'static str,(),LocalGlyphCache<E>> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: (),
            text: "",
            align: (0.5,0.5),
            glyph_cache: None,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Stil,GlyphCache> Label<'w,E,Text,Stil,GlyphCache> where
    E: Env,
    Text: 'w,
    GlyphCache: 'w,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> Label<'w,E,T,Stil,GlyphCache> where T: 'w {
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
    pub fn with_style<SStil>(self, style: SStil) -> Label<'w,E,Text,SStil,GlyphCache> where SStil: 'w {
        Label{
            id: self.id,
            size: self.size,
            style,
            text: self.text,
            align: self.align,
            glyph_cache: self.glyph_cache,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,Text,Stil,GlyphCache> Statize<E> for Label<'w,E,Text,Stil,GlyphCache> where
    E: Env,
    Text: StatizeSized<E>,
    Stil: StatizeSized<E>,
    GlyphCache: StatizeSized<E>,
{
    type Statur = Label<'static,E,Text::StaturSized,Stil::StaturSized,GlyphCache::StaturSized>;
}
