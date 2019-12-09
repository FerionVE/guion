use crate::core::util::bounds::Dims;
use crate::core::util::bounds::Offset;
#[derive(Clone)]
pub struct Border {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Border {
    #[inline]
    pub fn new(left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self{
            left,
            right,
            top,
            bottom,
        }
    }
    #[inline]
    pub fn empty() -> Self {
        Self::new(0,0,0,0)
    }
    #[inline]
    pub fn inner(&self) -> Offset {
        Offset{
            x: self.left as i32,
            y: self.top as i32,
        }
    }
    ///the size of the effective border
    #[inline]
    pub fn border_effective(&self) -> Dims {
        Dims{
            w: self.left + self.right,
            h: self.top + self.bottom,
        }
    }
}