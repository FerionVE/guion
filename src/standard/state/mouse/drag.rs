use crate::core::*;
use std::ffi::OsString;

pub enum DragItem<E> where E: Env {
    Widget(E::WidgetID),
    Text(String),
    RawText(OsString),
}