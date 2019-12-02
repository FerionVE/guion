use std::ops::Index;
use crate::core::util::border::Border;
//TODO may use priv-mod-but-pub-use trick everywhere
mod dir;
mod calc;

pub use dir::*;
pub use calc::*;

///Layouting with Lazout (qwertz intensifies)
#[derive(Clone)]
pub struct Lazout {
    pub x: LazoutDir,
    pub y: LazoutDir,
}

impl Lazout {
    pub fn fixed(w: u32, h: u32) -> Self {
        Self{
            x: LazoutDir{
                min: w,
                max: w,
                optimal: w,
                pressure: 0.0,
            },
            y: LazoutDir{
                min: h,
                max: h,
                optimal: h,
                pressure: 0.0,
            }
        }
    }

    pub fn with_border(&self, b: &Border) -> Self {
        let mut c = self.clone();
        c.add_border(b);
        c
    }

    pub fn add_border(&mut self, b: &Border) {
        self.x.add(b.left+b.right);
        self.y.add(b.top+b.bottom);
    }
}

impl Index<Orientation> for Lazout {
    type Output = LazoutDir;

    fn index(&self, i: Orientation) -> &Self::Output {
        match i {
            Orientation::Horizontal() => &self.x,
            Orientation::Vertical() => &self.y,
        }
    }
}

pub enum Orientation {
    Horizontal(),
    Vertical(),
}