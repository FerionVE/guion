//! standard variants
use super::*;

#[derive(Clone,Debug)]
pub struct KbdDown<E> where E: Env {
    pub key: EEKey<E>,
}
#[derive(Clone,Debug)]
pub struct KbdUp<E> where E: Env {
    pub key: EEKey<E>,
    pub down_widget: WidgetIdent<E>,
    pub down_ts: u64,
}
#[derive(Clone,Debug)]
pub struct KbdPress<E> where E: Env {
    pub key: EEKey<E>,
    pub down_widget: WidgetIdent<E>,
    pub down_ts: u64,
}
#[derive(Clone,Debug)]
pub struct TextInput {
    pub text: String, //TODO Arc<str> for less clonery
}

#[derive(Clone,Debug)]
pub struct MouseDown<E> where E: Env {
    pub key: EEKey<E>,
    pub pos: Offset,
}
#[derive(Clone,Debug)]
pub struct MouseUp<E> where E: Env {
    pub key: EEKey<E>,
    pub pos: Offset,
    pub down_pos: Offset,
    pub down_widget: WidgetIdent<E>,
    pub down_ts: u64,
}

#[derive(Clone,Debug)]
pub struct MouseScroll {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone,Debug)]
pub struct MouseMove {
    pub pos: Offset,
}

#[derive(Clone,Debug)]
pub struct MouseEnter;
#[derive(Clone,Debug)]
pub struct MouseLeave;

#[derive(Clone,Debug)]
pub struct WindowMove {
    pub pos: Offset,
    pub size: Dims,
}

#[derive(Clone,Debug)]
pub struct WindowResize {
    pub size: Dims,
}

#[derive(Clone,Debug)]
pub struct Focus;

#[derive(Clone,Debug)]
pub struct Unfocus;

macro_rules! pos {
    ($field:ident) => {
        #[inline]
        fn in_bounds(&self, b: &Bounds) -> bool {
            self.$field.is_inside(b)
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
impl<E> Variant<E> for TextInput where E: Env {focused!();}

impl<E> Variant<E> for MouseDown<E> where E: Env {consuming!();hovered!();pos!(pos);}
impl<E> Variant<E> for MouseUp<E> where E: Env {consuming!();hovered!();pos!(pos);}
impl<E> Variant<E> for MouseScroll where E: Env {consuming!();hovered!();}
impl<E> Variant<E> for MouseMove where E: Env {consuming!();root!();pos!(pos);}
impl<E> Variant<E> for MouseEnter where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for MouseLeave where E: Env {consuming!();invalid!();}

impl<E> Variant<E> for WindowMove where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for WindowResize where E: Env {consuming!();invalid!();}

impl<E> Variant<E> for Focus where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for Unfocus where E: Env {consuming!();invalid!();}
#[non_exhaustive]
#[derive(Clone,Debug)]
pub enum RootEvent<E> where E: Env {
    KbdDown{key: EEKey<E>},
    KbdPress{key: EEKey<E>},
    KbdUp{key: EEKey<E>},
    TextInput{text: String},
    MouseDown{key: EEKey<E>},
    MouseUp{key: EEKey<E>},
    MouseScroll{x: i32, y: i32}, //TODO replace with Offset
    MouseMove{pos: Offset}, //TODO which mouse moves??
    WindowMove{pos: Offset,size: Dims},
    WindowResize{size: Dims},
    MouseLeaveWindow{},
}

impl<E> Variant<E> for RootEvent<E> where E: Env {
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
