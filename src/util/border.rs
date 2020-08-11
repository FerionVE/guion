use super::*;

#[derive(Clone,Copy)]
pub struct Border {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Border {
    #[inline]
    pub const fn new(left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self{
            left,
            right,
            top,
            bottom,
        }
    }
    #[inline]
    pub const fn uniform(side: u32) -> Self {
        Self::new(side,side,side,side)
    }
    #[inline]
    pub const fn empty() -> Self {
        Self::new(0,0,0,0)
    }
    /// get the offset of the inner relative to the outer
    #[inline]
    pub const fn inner(&self) -> Offset {
        Offset{
            x: self.left as i32,
            y: self.top as i32,
        }
    }
    ///the size of the effective border
    #[inline]
    pub const fn border_effective(&self) -> Dims {
        Dims{
            w: self.left + self.right,
            h: self.top + self.bottom,
        }
    }
}

qwutils::opion!(mul(Border,u32) |s,r| {
    let r = r.clone();
    s.left *= r;
    s.right *= r;
    s.top *= r;
    s.bottom *= r;
});

qwutils::opion!(div(Border,u32) |s,r| {
    let r = r.clone();
    s.left /= r;
    s.right /= r;
    s.top /= r;
    s.bottom /= r;
});