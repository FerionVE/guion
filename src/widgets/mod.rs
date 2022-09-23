//! some standard widgets. WIP
use super::*;

#[macro_use]
macro_rules! try_or_false {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return false,
        }
    };
}

pub mod util;

pub mod pane;
pub mod button;
// //#[allow(unused)]
// //pub mod null;
pub mod label;
pub mod pbar;
pub mod checkbox;
pub mod splitpane;
pub mod textbox;
pub mod area;
