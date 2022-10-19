use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Range;

use crate::aliases::{ESize, EStyle, ETCurSel};
use crate::env::Env;
use crate::layout::Gonstraints;
use crate::view::mutor_trait::MutorEndBuilder;

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<E,Text,Scroll,Curs,TBUpd,TBScr> where
    E: Env,
{
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub scroll: Scroll,
    pub cursor: Curs,
    pub update: TBUpd,
    pub scroll_update: TBScr,
    p: PhantomData<()>,
}

impl<E> TextBox<E,String,(u32,u32),ETCurSel<E>,(),()> where
    E: Env,
{
    #[inline]
    pub fn new() -> Self {
        Self{
            size: Gonstraints::empty(),
            style: Default::default(),
            text: "".to_owned(),
            scroll: (0,0),
            cursor: Default::default(), //TODO default trait
            update: (),
            scroll_update: (),
            p: PhantomData,
        }
    }
}
// impl<'w,E,Text> TextBox<'w,E,Text,RemoteState<E,(u32,u32)>,RemoteState<E,ETCurSel<E>>,(),()> where
//     E: Env,
// {
//     #[inline]
//     pub fn immediate(text: Text) -> Self {
//         Self{
//             size: Gonstraints::empty(),
//             style: Default::default(),
//             text,
//             scroll: RemoteState::for_widget(id.clone()),
//             cursor: RemoteState::for_widget(id.clone()), //TODO default trait
//             update: (),
//             scroll_update: (),
//             id,
//             p: PhantomData,
//         }
//     }
// }

impl<E,Text,Scroll,Curs,TBUpd,TBScr> TextBox<E,Text,Scroll,Curs,TBUpd,TBScr> where
    E: Env,
    TBUpd: MutorEndBuilder<(Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<E>>),E>,
    TBScr: MutorEndBuilder<(u32,u32),E>,
{
    #[inline]
    pub fn immediate_test(text: Text, scroll: Scroll, cursor: Curs, tbupd: TBUpd, tbscr: TBScr) -> Self {
        Self{
            size: Gonstraints::empty(),
            style: Default::default(),
            text,
            scroll: scroll,
            cursor: cursor, //TODO default trait
            update: tbupd,
            scroll_update: tbscr,
            p: PhantomData,
        }
    }
}

impl<E,Text,Scroll,Curs,TBUpd,TBScr> TextBox<E,Text,Scroll,Curs,TBUpd,TBScr> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<E,T,Scroll,Curs,TBUpd,TBScr> {
        TextBox{
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
    #[inline]
    pub fn with_states<PScroll,CCurs>(self, scroll: PScroll, cursor: CCurs) -> TextBox<E,Text,PScroll,CCurs,TBUpd,TBScr> {
        TextBox{
            size: self.size,
            style: self.style,
            text: self.text,
            scroll,
            cursor,
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
