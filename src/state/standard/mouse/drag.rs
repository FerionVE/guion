use crate::*;
use std::ffi::OsString;

pub enum DragItem<E> where E: Env {
    //Widget(E::WidgetID), //TODO fix
    Text(String),
    RawText(OsString),
}
