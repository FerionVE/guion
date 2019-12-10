use std::ffi::OsString;
use crate::core::ctx::Context;

pub enum DragItem<E> where E: Context {
    Widget(E::WidgetID),
    Text(String),
    RawText(OsString),
}