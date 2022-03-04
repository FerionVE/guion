use super::*;
use std::marker::PhantomData;
use util::{LocalGlyphCache, remote_state::RemoteState};
use state::Cursor;

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<'w,E,Text,Scroll,Curs,TBUpd,GlyphCache> where
    E: Env,
    Text: 'w,
    Scroll: 'w,
    Curs: 'w,
    CursorStickX: 'w,
    GlyphCache: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub scroll: Scroll,
    pub cursor: Curs,
    pub glyph_cache: GlyphCache,
    pub update: TBUpd,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),Cursor,(),LocalGlyphCache<E>> where
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
            cursor: Cursor{select: 0, caret: 0}, //TODO default trait
            glyph_cache: None,
            update: (),
            p: PhantomData,
        }
    }
}
impl<'w,E,Text> TextBox<'w,E,Text,RemoteState<E,(u32,u32)>,RemoteState<E,Cursor>,(),RemoteState<E,LocalGlyphCache<E>>> where
    E: Env,
    Text: 'w,
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
            id,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs,TBUpd,GlyphCache> TextBox<'w,E,Text,Scroll,Curs,TBUpd,GlyphCache> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,Scroll,Curs,GlyphCache> where T: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            glyph_cache: self.glyph_cache,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    #[inline]
    pub fn with_states<PScroll,CCurs>(self, scroll: PScroll, cursor: CCurs, cursor_stick_x: XCursorStickX) -> TextBox<'w,E,Text,PScroll,CCurs,GlyphCache> where PScroll: 'w, CCurs: 'w, XCursorStickX: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text: self.text,
            scroll,
            cursor,
            cursor_stick_x,
            glyph_cache: self.glyph_cache,
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
    fn boxed(&self, tu: TextUpdate<'static>, nc: ETCursor<E>) -> Option<BoxMutEvent<E>>;
}

impl<E> TBMut<E> for () where E: Env {
    #[inline]
    fn boxed(&self, tu: TextUpdate<'static>, nc: ETCursor<E>) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TBMut<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,TextUpdate<'static>,ETCursor<E>) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self, tu: TextUpdate<'static>, nc: ETCursor<E>) -> Option<BoxMutEvent<E>> {
        let s = self.clone();
        Some(Box::new(move |r,x,c| s(r,x,c,tu,nc) ))
    }
}
