//! standard variants
use super::*;

#[derive(Clone)]
pub struct KbdDown<E> where E: Env {
    pub key: EEKey<E>,
}
#[derive(Clone)]
pub struct KbdUp<E> where E: Env {
    pub key: EEKey<E>,
    pub down_widget: E::WidgetPath,
    pub down_ts: u64,
}
#[derive(Clone)]
pub struct KbdPress<E> where E: Env {
    pub key: EEKey<E>,
    pub down_widget: E::WidgetPath,
    pub down_ts: u64,
}

#[derive(Clone)]
pub struct MouseDown<E> where E: Env {
    pub key: EEKey<E>,
    pub pos: Offset,
}
#[derive(Clone)]
pub struct MouseUp<E> where E: Env {
    pub key: EEKey<E>,
    pub pos: Offset,
    pub down_pos: Offset,
    pub down_widget: E::WidgetPath,
    pub down_ts: u64,
}

#[derive(Clone)]
pub struct MouseMove {
    pub pos: Offset,
}

#[derive(Clone)]
pub struct MouseEnter;
#[derive(Clone)]
pub struct MouseLeave;

#[derive(Clone)]
pub struct WindowMove {
    pub pos: Offset,
    pub size: Dims,
}

#[derive(Clone)]
pub struct WindowResize {
    pub size: Dims,
}

#[derive(Clone)]
pub struct Focus;

#[derive(Clone)]
pub struct Unfocus;

macro_rules! pos {
    ($field:ident) => {
        #[inline]
        fn position(&self) -> Option<Offset> {
            Some(self.$field.clone())
        }
    };
}

macro_rules! consuming {
    () => {
        #[inline]
        fn consuming(&self) -> bool {
            true
        }
    };
}

macro_rules! focused {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::FOCUSED
        }
    };
}

macro_rules! hovered {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::HOVERED
        }
    };
}

macro_rules! root {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::ROOT
        }
    };
}

macro_rules! invalid {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::INVALID
        }
    };
}

impl<E> Variant<E> for KbdDown<E> where E: Env {focused!();}
impl<E> Variant<E> for KbdPress<E> where E: Env {focused!();}
impl<E> Variant<E> for KbdUp<E> where E: Env {focused!();}

impl<E> Variant<E> for MouseDown<E> where E: Env {consuming!();hovered!();pos!(pos);}
impl<E> Variant<E> for MouseUp<E> where E: Env {consuming!();hovered!();pos!(pos);}
impl<E> Variant<E> for MouseMove where E: Env {consuming!();root!();pos!(pos);}
impl<E> Variant<E> for MouseEnter where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for MouseLeave where E: Env {consuming!();invalid!();}

impl<E> Variant<E> for WindowMove where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for WindowResize where E: Env {consuming!();invalid!();}

impl<E> Variant<E> for Focus where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for Unfocus where E: Env {consuming!();invalid!();}
#[non_exhaustive]
#[derive(Clone)]
pub enum RootEvent<E> where E: Env {
    KbdDown{key: EEKey<E>},
    KbdPress{key: EEKey<E>},
    KbdUp{key: EEKey<E>},
    MouseDown{key: EEKey<E>},
    MouseUp{key: EEKey<E>},
    MouseMove{pos: Offset}, //TODO which mouse moves??
    WindowMove{pos: Offset,size: Dims},
    WindowResize{size: Dims},
    MouseLeaveWindow{},
}

impl<E> Variant<E> for RootEvent<E> where E: Env {
    #[inline]
    fn position(&self) -> Option<Offset> {
        None
    }
    #[inline]
    fn filter(&self, _: &Bounds) -> bool {
        false
    }
    #[inline]
    fn consuming(&self) -> bool {
        true
    }
    #[inline]
    fn destination(&self) -> EEDest<E> {
        Destination::ROOT
    }
    #[inline]
    fn _root_only(&self) -> bool {
        true
    }
}