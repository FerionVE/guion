//pub mod as_any;
use crate::core::lazout::size::Size;
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

    #[inline] pub fn x1(&self) -> i32 { self.off.x + (self.size.w as i32) }
    #[inline] pub fn y1(&self) -> i32 { self.off.y + (self.size.h as i32) }

    pub fn inside(&self, b: &Border) -> Self {
        let mut s = self.clone();
        s.off += b.inner();
        s.size -= b.border_effective();
        s
    }

    pub fn slice(&self, b: &Bounds) -> Self {
        Self{
            off: &self.off + &b.off,
            size: Dims{
                w: b.size.w .min( (self.size.w as i32).saturating_sub(b.off.x) as u32 ),
                h: b.size.h .min( (self.size.h as i32).saturating_sub(b.off.y) as u32 ),
            }
        }
    }
    ///TODO doc
    pub fn step(&self, step: i32) -> Self {
        let mut senf = self.clone();
        senf.off.x += step;
        senf.off.y += step;
        senf.size.w = (senf.size.w as i32 - step*2).max(0) as u32;
        senf.size.h = (senf.size.h as i32 - step*2).max(0) as u32;
        senf
    }

    pub fn inner_centered(&self, s: Dims) -> Self {
        let nx = (self.size.w as i32 - s.w as i32)/2;
        let ny = (self.size.h as i32 - s.h as i32)/2;
        Self{
            off: Offset{
                x: self.off.x + nx,
                y: self.off.y + ny,
            },
            size: s,
        }
    }
}

impl Offset {
    pub fn is_inside(&self, b: &Bounds) -> bool {
        self.x >= b.x() && self.x < b.x1() &&
        self.y >= b.y() && self.y < b.y1()
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
        let nx = self.off.x.max(o.off.x);
        let ny = self.off.y.max(o.off.y);
        Bounds{
            off: Offset{
                x: nx,
                y: ny,
            },
            size: Dims{
                w: (self.x1().min(o.x1()) - nx).max(0) as u32,
                h: (self.y1().min(o.y1()) - ny).max(0) as u32,
            }
        }
    }
}
