use super::*;
//TODO may use priv-mod-but-pub-use trick everywhere
pub mod size;
pub mod weight;
pub mod calc;
mod imp;

pub use size::*;
pub use weight::*;

///Layouting with Lazout (qwertz intensifies)

#[derive(Clone,Copy)]
pub enum Orientation {
    Horizontal(),
    Vertical(),
}

pub trait ISize: From<Size> {
    #[inline]
    fn add(&mut self, o: &Self, dir: Orientation) {
        match dir {
            Orientation::Horizontal() => self.add_x(o),
            Orientation::Vertical() => self.add_y(o),
        }
    }
    fn add_x(&mut self, o: &Self);
    fn add_y(&mut self, o: &Self);
}