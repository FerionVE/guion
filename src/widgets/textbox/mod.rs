use super::*;
use std::marker::PhantomData;
use util::{LocalGlyphCache, caption::Caption, remote_state::RemoteState};
use state::Cursor;

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache,Stil> where
    E: Env,
    Text: 'w,
    Scroll: 'w,
    Curs: 'w,
    CursorStickX: 'w,
    GlyphCache: 'w,
    Stil: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub text: Text,
    pub scroll: Scroll,
    pub cursor: Curs,
    pub cursor_stick_x: CursorStickX,
    pub glyph_cache: GlyphCache,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),Cursor,Option<u32>,LocalGlyphCache<E>,()> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Gonstraints::empty(),
            style: (),
            text: "".to_owned(),
            scroll: (0,0),
            cursor: Cursor{select: 0, caret: 0}, //TODO default trait
            cursor_stick_x: None,
            glyph_cache: None,
            p: PhantomData,
        }
    }
}
impl<'w,E,Text> TextBox<'w,E,Text,RemoteState<E,(u32,u32)>,RemoteState<E,Cursor>,RemoteState<E,Option<u32>>,RemoteState<E,LocalGlyphCache<E>>,()> where
    E: Env,
    E::Context: DynState<E>,
    Text: 'w,
{
    #[inline]
    pub fn immediate(id: E::WidgetID, text: Text) -> Self {
        Self{
            size: Gonstraints::empty(),
            style: (),
            text,
            scroll: RemoteState::for_widget(id.clone()),
            cursor: RemoteState::for_widget(id.clone()), //TODO default trait
            cursor_stick_x: RemoteState::for_widget(id.clone()),
            glyph_cache: RemoteState::for_widget(id.clone()),
            id,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache,Stil> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache,Stil> where
    E: Env,
    Text: 'w,
    Scroll: 'w,
    Curs: 'w,
    CursorStickX: 'w,
    GlyphCache: 'w,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,Scroll,Curs,CursorStickX,GlyphCache,Stil> where T: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            cursor_stick_x: self.cursor_stick_x,
            glyph_cache: self.glyph_cache,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    #[inline]
    pub fn with_states<PScroll,CCurs,XCursorStickX>(self, scroll: PScroll, cursor: CCurs, cursor_stick_x: XCursorStickX) -> TextBox<'w,E,Text,PScroll,CCurs,XCursorStickX,GlyphCache,Stil> where PScroll: 'w, CCurs: 'w, XCursorStickX: 'w {
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
    pub fn with_style<SStil>(self, style: SStil) -> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,GlyphCache,SStil> where SStil: 'w, ESVariant<E>: for<'z> StyleVariantSupport<&'z Stil> {
        TextBox{
            id: self.id,
            size: self.size,
            style,
            text: self.text,
            cursor: self.cursor,
            cursor_stick_x: self.cursor_stick_x,
            scroll: self.scroll,
            glyph_cache: self.glyph_cache,
            p: PhantomData,
        }
    }
}
