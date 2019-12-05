//pub mod as_any;
use std::ops::BitAnd;
use std::ops::AddAssign;
use std::ops::SubAssign;

#[derive(Clone)]
pub struct Offset {
    pub x: u32,
    pub y: u32,
}
#[derive(Clone)]
pub struct Dims {
    pub w: u32,
    pub h: u32,
}
#[derive(Clone)]
pub struct Bounds {
    pub off: Offset,
    pub size: Dims,
}

impl Bounds {
    pub fn from_xywh(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self{
            off: Offset{
                x,
                y,
            },
            size: Dims{
                w,
                h,
            }
        }
    }

    #[inline] pub fn x(&self) -> u32 { self.off.x }
    #[inline] pub fn y(&self) -> u32 { self.off.y }
    #[inline] pub fn w(&self) -> u32 { self.size.w }
    #[inline] pub fn h(&self) -> u32 { self.size.h }

    #[inline] pub fn x_mut(&mut self) -> &mut u32 { &mut self.off.x }
    #[inline] pub fn y_mut(&mut self) -> &mut u32 { &mut self.off.y }
    #[inline] pub fn w_mut(&mut self) -> &mut u32 { &mut self.size.w }
    #[inline] pub fn h_mut(&mut self) -> &mut u32 { &mut self.size.h }
}

impl<'a> AddAssign<&'a Offset> for Bounds {
    fn add_assign(&mut self, o: &Offset) {
        self.off.x += o.x;
        self.off.y += o.y;
    }
}

impl<'a> SubAssign<&'a Offset> for Bounds {
    fn sub_assign(&mut self, o: &Offset) {
        self.off.x -= o.x;
        self.off.y -= o.y;
    }
}

impl<'a,'b> BitAnd<&'b Bounds> for &'a Bounds {
    type Output = Bounds;

    fn bitand(self, o: &Bounds) -> Self::Output {
        unimplemented!()
    }
}
