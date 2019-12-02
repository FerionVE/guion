use std::ops::Index;
use crate::core::util::border::Border;
//TODO may use priv-mod-but-pub-use trick everywhere
mod dir;
mod calc;

pub use dir::*;
pub use calc::*;

///Layouting with Lazout (qwertz intensifies)



#[derive(Clone,Copy)]
pub enum Orientation {
    Horizontal(),
    Vertical(),
}