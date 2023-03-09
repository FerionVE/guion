//! some standard widgets. WIP

use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn, PathResolvus};

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

// pub mod pane;
// pub mod button;
// // //#[allow(unused)]
// // //pub mod null;
pub mod label;
// pub mod pbar;
// pub mod checkbox;
// pub mod splitpane;
// pub mod textbox;
// pub mod area;

pub fn soft_single_child_resolve_check<V,E>(a: Option<&(dyn PathResolvusDyn<E>+'_)>, b: V) -> bool where E: Env, V: PathFragment<E> + PartialEq {
    a.map_or(true, |a| a.inner().is_none() || a.try_fragment::<V>() == Some(&b) )
}
