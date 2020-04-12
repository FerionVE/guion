use super::*;

pub mod size;
pub mod calc;
mod imp;

pub use size::*;

///Layouting with Lazout (qwertz intensifies)

#[derive(Clone,Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

//TODO move to trait submodule
pub trait ISize: From<Size> + Into<Size> + Clone {
    #[inline]
    fn empty() -> Self where Self: Sized {
        Size::empty().into()
    }
    #[inline]
    fn add(&mut self, o: &Self, dir: Orientation) {
        match dir {
            Orientation::Horizontal => self.add_x(o),
            Orientation::Vertical => self.add_y(o),
        }
    }
    fn add_x(&mut self, o: &Self);
    fn add_y(&mut self, o: &Self);
    fn flip(&mut self);
    fn as_std(&self) -> Size;
}

impl ISize for Size {
    fn add_x(&mut self, o: &Self) {
        Size::add_x(self,o)
    }
    fn add_y(&mut self, o: &Self) {
        Size::add_y(self,o)
    }
    fn flip(&mut self) {
        Size::flip(self)
    }
    fn as_std(&self) -> Size {
        self.clone()
    }
}