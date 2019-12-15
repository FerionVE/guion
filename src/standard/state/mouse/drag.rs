use std::ffi::OsString;
use crate::core::ctx::*;

pub enum DragItem<E> where E: Env {
    Widget(E::WidgetID),
    Text(String),
    RawText(OsString),
}