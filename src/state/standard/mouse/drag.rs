use crate::*;
use crate::newpath::PathResolvusDyn;
use std::ffi::OsString;
use std::sync::Arc;

pub enum DragItem<E> where E: Env {
    Widget(Arc<dyn PathResolvusDyn<E>>), //TODO fix
    Text(String),
    RawText(OsString),
}
