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

pub trait TOrientation {
    #[inline] fn val() -> Orientation;
}

pub struct Horizontal;

impl TOrientation for Horizontal {
    #[inline] fn val() -> Orientation {
        Orientation::Horizontal()
    }
}

pub struct Vertical;

impl TOrientation for Vertical {
    #[inline] fn val() -> Orientation {
        Orientation::Vertical()
    }
}