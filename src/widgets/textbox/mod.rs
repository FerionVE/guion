use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Range;

use crate::aliases::{ESize, EStyle, ETCurSel};
use crate::env::Env;
use crate::layout::Gonstraints;
use crate::text::stor::TextStorMut;
use crate::view::mut_target::MuTarget;
use crate::view::mutor_trait::{MutorEndBuilder, MutorToBuilder, MutorToBuilderExt};

use self::state::TextBoxMeta;

use super::util::state::AtomStateMut;

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<'w,E,Text,Scroll,Curs,TBUpd> where
    E: Env,
{
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub text: Text,
    pub scroll: Option<Scroll>,
    pub cursor: Option<Curs>,
    pub update: TBUpd,
    pub tbmeta: Option<&'w TextBoxMeta<E>>,
    p: PhantomData<()>,
}

impl<E> TextBox<'_,E,(),(u32,u32),ETCurSel<E>,()> where
    E: Env,
{
    #[inline]
    pub fn new() -> Self {
        Self{
            size: Gonstraints::empty_fill(),
            style: Default::default(),
            text: (),
            scroll: None,
            cursor: None, //TODO default trait
            update: (),
            p: PhantomData,
            tbmeta: None,
        }
    }
}

impl<E,Text> TextBox<'_,E,Text,(u32,u32),ETCurSel<E>,()> where
    E: Env,
{
    #[inline]
    pub fn of_text(text: Text) -> Self {
        Self{
            size: Gonstraints::empty_fill(),
            style: Default::default(),
            text,
            scroll: None,
            cursor: None, //TODO default trait
            update: (),
            p: PhantomData,
            tbmeta: None,
        }
    }
}

impl<'w,E,Text,Scroll,Curs,TBUpd> TextBox<'w,E,Text,Scroll,Curs,TBUpd> where
    E: Env,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,Scroll,Curs,TBUpd> {
        TextBox{
            size: self.size,
            style: self.style,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            update: self.update,
            p: PhantomData,
            tbmeta: self.tbmeta,
        }
    }

    #[inline]
    pub fn with_cursor<CCurs>(self, cursor: CCurs) -> TextBox<'w,E,Text,Scroll,CCurs,TBUpd> {
        TextBox{
            size: self.size,
            style: self.style,
            text: self.text,
            scroll: self.scroll,
            cursor: Some(cursor),
            update: self.update,
            tbmeta: self.tbmeta,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_scroll<PScroll>(self, scroll: PScroll) -> TextBox<'w,E,Text,PScroll,Curs,TBUpd> {
        TextBox{
            size: self.size,
            style: self.style,
            text: self.text,
            scroll: Some(scroll),
            cursor: self.cursor,
            update: self.update,
            tbmeta: self.tbmeta,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_meta(self, meta: &'w TextBoxMeta<E>) -> TextBox<'w,E,Text,Scroll,Curs,TBUpd> {
        TextBox{
            size: self.size,
            style: self.style,
            text: self.text,
            scroll: self.scroll,
            cursor: self.cursor,
            update: self.update,
            tbmeta: Some(meta),
            p: PhantomData,
        }
    }

    pub fn with_update<NewTBUpd>(self, mutor: NewTBUpd) -> TextBox<'w,E,Text,Scroll,Curs,NewTBUpd> where NewTBUpd: MutorEndBuilder<TextBoxUpdate<E>,E> {
        TextBox{
            size: self.size,
            style: self.style,
            text: self.text,
            scroll: self.scroll,
            cursor: self.cursor,
            update: mutor,
            tbmeta: self.tbmeta,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_update_if<LeftMutor,LeftArgs,LeftTarget,RightFn>(self, left_mutor: LeftMutor, left_arg: LeftArgs, right_fn: RightFn) -> TextBox<'w,E,Text,Scroll,Curs,impl MutorEndBuilder<TextBoxUpdate<E>,E>>
    where 
        LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
        LeftTarget: MuTarget<E> + ?Sized,
        LeftArgs: Clone + Sized + Send + Sync + 'static,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut LeftTarget::Mutable<'ss>,&'ss (),
            TextBoxUpdate<E>,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        self.with_update(
            left_mutor.mutor_end_if(left_arg, right_fn)
        )
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

#[non_exhaustive]
#[derive(Default,Clone)]
pub struct TextBoxUpdate<E> where E: Env {
    pub update_text: Option<(Range<usize>,Cow<'static,str>)>,
    pub update_cursor: Option<ETCurSel<E>>,
    pub update_scroll_pos: Option<(u32,u32)>,
}

impl<E> TextBoxUpdate<E> where E: Env {
    #[inline]
    pub fn apply_to_text<Text>(&self, mut text: Text) -> bool where E: Env, Text: TextStorMut<E> {
        let mut mutated = false;
        if let Some(tbupd) = self.update_text.as_ref() {
            text.replace(tbupd.0.clone(),tbupd.1.as_ref());
            mutated = true;
        }
        mutated
    }

    #[inline]
    pub fn apply_to_meta(&self, meta: &mut TextBoxMeta<E>) -> bool {
        let mut mutated = false;
        if let Some(curs) = self.update_cursor.as_ref() {
            meta.selection = curs.clone();
            mutated = true;
        }
        if let Some(scroll) = self.update_scroll_pos.as_ref() {
            meta.scroll = *scroll;
            mutated = true;
        }
        mutated 
    }
    
    #[inline]
    pub fn apply_to_cursor(&self, selection: &mut ETCurSel<E>) -> bool where E: Env {
        let mut mutated = false;
        if let Some(curs) = self.update_cursor.as_ref() {
            *selection = curs.clone();
            mutated = true;
        }
        mutated 
    }

    #[inline]
    pub fn apply_to_scroll(&self, scroll: &mut (u32,u32)) -> bool where E: Env {
        let mut mutated = false;
        if let Some(s) = self.update_scroll_pos.as_ref() {
            *scroll = *s;
            mutated = true;
        }
        mutated 
    }

    #[inline]
    pub fn apply_to_selection_state<S>(&self, mut selection: S, ctx: &mut E::Context<'_>) -> bool where E: Env, S: AtomStateMut<E,ETCurSel<E>> {
        let mut mutated = false;
        if let Some(curs) = self.update_cursor.as_ref() {
            selection.set(curs.clone(),ctx);
            mutated = true;
        }
        mutated 
    }

    #[inline]
    pub fn apply_to_scroll_state<S>(&self, mut scroll: S, ctx: &mut E::Context<'_>) -> bool where E: Env, S: AtomStateMut<E,(u32,u32)> {
        let mut mutated = false;
        if let Some(s) = self.update_scroll_pos.as_ref() {
            scroll.set(*s,ctx);
            mutated = true;
        }
        mutated 
    }

    #[inline]
    pub fn apply_to_state<S>(&self, mut meta: S, ctx: &mut E::Context<'_>) -> bool where E: Env, S: AtomStateMut<E,ETCurSel<E>> + AtomStateMut<E,(u32,u32)> {
        let mut mutated = false;
        if let Some(curs) = self.update_cursor.as_ref() {
            meta.set(curs.clone(),ctx);
            mutated = true;
        }
        if let Some(s) = self.update_scroll_pos.as_ref() {
            meta.set(*s,ctx);
            mutated = true;
        }
        mutated 
    }
}
