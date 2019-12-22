//pub mod as_any;
use crate::core::*;
use util::border::Border;
use std::ops::BitAnd;

#[derive(Clone)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
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
    pub const fn from_xywh(x: i32, y: i32, w: u32, h: u32) -> Self {
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

    #[inline] pub fn x(&self) -> i32 { self.off.x }
    #[inline] pub fn y(&self) -> i32 { self.off.y }
    #[inline] pub fn w(&self) -> u32 { self.size.w }
    #[inline] pub fn h(&self) -> u32 { self.size.h }

    #[inline] pub fn x_mut(&mut self) -> &mut i32 { &mut self.off.x }
    #[inline] pub fn y_mut(&mut self) -> &mut i32 { &mut self.off.y }
    #[inline] pub fn w_mut(&mut self) -> &mut u32 { &mut self.size.w }
    #[inline] pub fn h_mut(&mut self) -> &mut u32 { &mut self.size.h }

    pub fn inside(&self, b: &Border) -> Self {
        let mut s = self.clone();
        s.off += b.inner();
        s.size -= b.border_effective();
        s
    }

    pub fn slice(&self, b: &Bounds) -> Self {
        unimplemented!()
    }
}

impl Offset {
    pub fn is_inside(&self, b: &Bounds) -> bool {
        unimplemented!()
    }
}

qwutils::opion!(add(Bounds,Offset) |s,r| {
    s.off.x += r.x;
    s.off.y += r.y;
});
qwutils::opion!(sub(Bounds,Offset) |s,r| {
    s.off.x -= r.x;
    s.off.y -= r.y;
});

qwutils::opion!(add(Offset,Offset) |s,r| {
    s.x += r.x;
    s.y += r.y;
});
qwutils::opion!(sub(Offset,Offset) |s,r| {
    s.x -= r.x;
    s.y -= r.y;
});

qwutils::opion!(add(Dims,Dims) |s,r| {
    s.w += r.w;
    s.h += r.h;
});
qwutils::opion!(sub(Dims,Dims) |s,r| {
    s.w = s.w.saturating_sub( r.w );
    s.h = s.h.saturating_sub( r.h );
});


/*qwutils::opion!(add(Bounds,Border) |s,r| {
    s.off.x += r.width();
    s.off.y += r.height();
});
qwutils::opion!(sub(Bounds,Border) |s,r| {
    s.off.x -= r.width();
    s.off.y -= r.height();
});*/

impl<'a,'b> BitAnd<&'b Bounds> for &'a Bounds {
    type Output = Bounds;

    fn bitand(self, o: &Bounds) -> Self::Output {
        unimplemented!()
    }
}
