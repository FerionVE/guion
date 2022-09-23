use super::*;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Range;
use util::{LocalGlyphCache, remote_state::RemoteState};

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where
    E: Env,
    Self: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub scroll: Scroll,
    pub cursor: Curs,
    pub glyph_cache: GlyphCache,
    pub update: TBUpd,
    pub scroll_update: TBScr,
    p: PhantomData<&'w (Text,Scroll,Curs,TBUpd,TBScr,GlyphCache)>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),ETCurSel<E>,(),(),LocalGlyphCache<E>> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Gonstraints::empty(),
            style: Default::default(),
            text: "".to_owned(),
            scroll: (0,0),
            cursor: Default::default(), //TODO default trait
            glyph_cache: None,
            update: (),
            scroll_update: (),
            p: PhantomData,
        }
    }
}
impl<'w,E,Text> TextBox<'w,E,Text,RemoteState<E,(u32,u32)>,RemoteState<E,ETCurSel<E>>,(),(),RemoteState<E,LocalGlyphCache<E>>> where
    E: Env,
{
    #[inline]
    pub fn immediate(id: E::WidgetID, text: Text) -> Self {
        Self{
            size: Gonstraints::empty(),
            style: Default::default(),
            text,
            scroll: RemoteState::for_widget(id.clone()),
            cursor: RemoteState::for_widget(id.clone()), //TODO default trait
            glyph_cache: RemoteState::for_widget(id.clone()),
            update: (),
            scroll_update: (),
            id,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs,TBUpd,TBScr> TextBox<'w,E,Text,Scroll,Curs,TBUpd,TBScr,LocalGlyphCache<E>> where
    E: Env,
    TBUpd: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<E>>) + Clone + Send + Sync + 'static,
    TBScr: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,(u32,u32)) + Clone + Send + Sync + 'static,
{
    #[inline]
    pub fn immediate_test(id: E::WidgetID, text: Text, scroll: Scroll, cursor: Curs, tbupd: TBUpd, tbscr: TBScr) -> Self {
        Self{
            size: Gonstraints::empty(),
            style: Default::default(),
            text,
            scroll: scroll,
            cursor: cursor, //TODO default trait
            glyph_cache: None, //TODO fix caching
            update: tbupd,
            scroll_update: tbscr,
            id,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> TextBox<'w,E,Text,Scroll,Curs,TBUpd,TBScr,GlyphCache> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,Scroll,Curs,TBUpd,TBScr,GlyphCache> where T: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            glyph_cache: self.glyph_cache,
            scroll_update: self.scroll_update,
            update: self.update,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    #[inline]
    pub fn with_states<PScroll,CCurs>(self, scroll: PScroll, cursor: CCurs) -> TextBox<'w,E,Text,PScroll,CCurs,TBUpd,TBScr,GlyphCache> where PScroll: 'w, CCurs: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text: self.text,
            scroll,
            cursor,
            glyph_cache: self.glyph_cache,
            update: self.update,
            scroll_update: self.scroll_update,
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

/// blanket-implemented on all `FnMut(&mut E::Context<'_>)`
pub trait TBMut<E> where E: Env {
    fn boxed(&self, tu: Option<(Range<usize>,Cow<'static,str>)>, nc: Option<ETCurSel<E>>) -> Option<BoxMutEvent<E>>;
}

impl<E> TBMut<E> for () where E: Env {
    #[inline]
    fn boxed(&self, _: Option<(Range<usize>,Cow<'static,str>)>, _: Option<ETCurSel<E>>) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TBMut<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<E>>) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self, tu: Option<(Range<usize>,Cow<'static,str>)>, nc: Option<ETCurSel<E>>) -> Option<BoxMutEvent<E>> {
        let s = self.clone();
        Some(Box::new(move |r,x,c| s(r,x,c,tu,nc) ))
    }
}


/// blanket-implemented on all `FnMut(&mut E::Context<'_>)`
pub trait TBSM<E> where E: Env {
    fn boxed(&self, value: (u32,u32)) -> Option<BoxMutEvent<E>>;
}

impl<E> TBSM<E> for () where E: Env {
    #[inline]
    fn boxed(&self, _: (u32,u32)) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TBSM<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,(u32,u32)) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self, value: (u32,u32)) -> Option<BoxMutEvent<E>> {
        let s = self.clone();
        Some(Box::new(move |r,x,c| s(r,x,c,value) ))
    }
}
