use std::ffi::OsString;
use std::sync::Arc;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;

pub enum DragItem<E> where E: Env {
    Widget(Arc<dyn PathResolvusDyn<E>>), //TODO fix
    Text(String),
    RawText(OsString),
}
