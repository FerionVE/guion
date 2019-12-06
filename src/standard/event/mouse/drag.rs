use std::ffi::OsString;
use crate::core::env::Env;

pub enum DragItem<E> where E: Env {
    Widget(E::WidgetID),
    Text(String),
    RawText(OsString),
}