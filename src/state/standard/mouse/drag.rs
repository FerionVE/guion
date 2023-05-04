use std::ffi::OsString;
use std::sync::Arc;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;
use crate::pathslice::PathSliceOwned;

pub enum DragItem {
    Widget(PathSliceOwned), //TODO fix
    Text(String),
    RawText(OsString),
}
