use crate::view::mutor_trait::{MutorEnd, MutorEndBuilder, MutorEndBuilderDyn};

use super::*;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Range;
use util::{LocalGlyphCache, remote_state::RemoteState};

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<'w,E,Text,Scroll,Curs> where
    E: Env,
    Self: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub scroll: Scroll,
    pub cursor: Curs,
    pub update: Option<&'w (dyn MutorEndBuilderDyn<(Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<E>>),E>+'w)>,
    pub scroll_update: Option<&'w (dyn MutorEndBuilderDyn<(u32,u32),E>+'w)>,
    p: PhantomData<&'w (Text,Scroll,Curs)>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),ETCurSel<E>> where
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
            update: None,
            scroll_update: None,
            p: PhantomData,
        }
    }
}
impl<'w,E,Text> TextBox<'w,E,Text,RemoteState<E,(u32,u32)>,RemoteState<E,ETCurSel<E>>> where
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
            update: None,
            scroll_update: None,
            id,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs> TextBox<'w,E,Text,Scroll,Curs> where
    E: Env,
    //TBUpd: MutorEndBuilder<(Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<E>>),E>,
    //TBScr: MutorEndBuilder<(u32,u32),E>,
{
    #[inline]
    pub fn immediate_test<TBUpd,TBScr>(id: E::WidgetID, text: Text, scroll: Scroll, cursor: Curs, tbupd: &'w TBUpd, tbscr: &'w TBScr) -> Self 
    where
        TBUpd: MutorEndBuilder<(Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<E>>),E>,
        TBScr: MutorEndBuilder<(u32,u32),E>,
    {
        Self{
            size: Gonstraints::empty(),
            style: Default::default(),
            text,
            scroll: scroll,
            cursor: cursor, //TODO default trait
            update: Some(tbupd.erase()),
            scroll_update: Some(tbscr.erase()),
            id,
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs> TextBox<'w,E,Text,Scroll,Curs> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,Scroll,Curs> where T: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            scroll_update: self.scroll_update,
            update: self.update,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    // #[inline]
    // pub fn with_states<PScroll,CCurs>(self, scroll: PScroll, cursor: CCurs) -> TextBox<'w,E,Text,PScroll,CCurs,TBUpd,TBScr,GlyphCache> where PScroll: 'w, CCurs: 'w {
    //     TextBox{
    //         id: self.id,
    //         size: self.size,
    //         style: self.style,
    //         text: self.text,
    //         scroll,
    //         cursor,
    //         glyph_cache: self.glyph_cache,
    //         update: self.update,
    //         scroll_update: self.scroll_update,
    //         p: PhantomData,
    //     }
    // }

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
