use crate::core::util::border::Border;
//TODO may use priv-mod-but-pub-use trick everywhere
pub mod size;
pub mod weight;
mod imp;

pub use size::*;
pub use weight::*;

///Layouting with Lazout (qwertz intensifies)

#[derive(Clone,Copy)]
pub enum Orientation {
    Horizontal(),
    Vertical(),
}