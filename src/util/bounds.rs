//! Bounds (x,y,w,h) and functionality

use std::fmt::Debug;

use crate::layout::Orientation;

use super::border::Border;

#[derive(Clone,Copy,Default,PartialEq,Eq)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone,Copy,Default,PartialEq,Eq)]
pub struct Dims {
    pub w: u32,
    pub h: u32,
}
#[derive(Clone,Copy,Default,PartialEq,Eq)]
pub struct Bounds {
    pub off: Offset,
    pub size: Dims,
}

impl Bounds {
    #[inline]
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
    #[inline]
    pub fn from_xyxy(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self{
            off: Offset{
                x: x0,
                y: y0,
            },
            size: Dims{
                w: x1.saturating_sub(x0) as u32,
                h: y1.saturating_sub(y0) as u32,
            }
        }
    }
    #[inline]
    pub const fn from_xy(x: i32, y: i32) -> Self {
        Self::from_xywh(x,y,0,0)
    }
    #[inline]
    pub const fn from_wh(w: u32, h: u32) -> Self {
        Self::from_xywh(0,0,w,h)
    }
    #[inline]
    pub const fn from_off(off: Offset) -> Self {
        Self::from_xywh(off.x,off.y,0,0)
    }
    #[inline]
    pub const fn from_size(size: Dims) -> Self {
        Self::from_xywh(0,0,size.w,size.h)
    }

    #[inline] pub const fn x(&self) -> i32 { self.off.x }
    #[inline] pub const fn y(&self) -> i32 { self.off.y }
    #[inline] pub const fn w(&self) -> u32 { self.size.w }
    #[inline] pub const fn h(&self) -> u32 { self.size.h }

    #[inline] pub fn x_mut(&mut self) -> &mut i32 { &mut self.off.x }
    #[inline] pub fn y_mut(&mut self) -> &mut i32 { &mut self.off.y }
    #[inline] pub fn w_mut(&mut self) -> &mut u32 { &mut self.size.w }
    #[inline] pub fn h_mut(&mut self) -> &mut u32 { &mut self.size.h }

    #[inline] pub const fn x1(&self) -> i32 { self.off.x + (self.size.w as i32) }
    #[inline] pub const fn y1(&self) -> i32 { self.off.y + (self.size.h as i32) }

    #[inline]
    pub const fn center(&self) -> Offset {
        Offset{
            x: self.off.x + (self.size.w/2) as i32,
            y: self.off.y + (self.size.h/2) as i32,
        }
    }

    /// Get the bounds inside this bound (subtract border)
    #[inline]
    pub fn inside_border(&self, b: &Border) -> Self {
        let mut s = *self;
        s.off += b.inner();
        s.size -= b.border_effective();
        s
    }

    /// Get the part of the inner which also is inside this bound
    /// 
    /// Use the BitAnd operator if the inner bound is absolute
    #[inline]
    pub fn slice(&self, inner_relative: &Bounds) -> Self {
        let inner_abs = inner_relative + self.off;
        self & inner_abs
    }
    //TODO doc
    #[inline]
    pub fn step(&self, step: i32) -> Self {
        let mut senf = *self;
        senf.off.x += step;
        senf.off.y += step;
        senf.size.w = (senf.size.w as i32 - step*2).max(0) as u32;
        senf.size.h = (senf.size.h as i32 - step*2).max(0) as u32;
        senf
    }
    /// Get bound with size s and centered relative to self
    #[inline]
    pub const fn inner_centered(&self, size: Dims) -> Self {
        let nx = (self.size.w as i32 - size.w as i32)/2;
        let ny = (self.size.h as i32 - size.h as i32)/2;
        Self{
            off: Offset{
                x: self.off.x + nx,
                y: self.off.y + ny,
            },
            size,
        }
    }
    /// [`inner_centered`](Self::inner_centered) but advanced
    /// 
    /// `align`: is the start-to-end relative position (0.0 - 1.0)
    #[inline]
    pub fn inner_aligned(&self, size: Dims, align: (f32,f32)) -> Self {
        //let align = (align.0.min(1.0).max(0.0), align.1.min(1.0).max(0.0));
        let nx = (self.size.w as f32 - size.w as f32)*align.0;
        let ny = (self.size.h as f32 - size.h as f32)*align.1;
        Self{
            off: Offset{
                x: self.off.x + nx as i32,
                y: self.off.y + ny as i32,
            },
            size,
        }
    }
    /// [`inner_centered`](Self::inner_centered) but advanced
    /// 
    /// `align`: the start-to-end relative position (0.0 - 1.0)
    #[inline]
    pub fn inner_aligned_f(&self, size: (f32,f32), align: (f32,f32)) -> Self {
        let align = (align.0.clamp(0.0,1.0), align.1.clamp(0.0,1.0));
        let nx = (self.size.w as f32 - size.0)*align.0;
        let ny = (self.size.h as f32 - size.1)*align.1;
        let nw = ((nx+size.0) as i32 - nx as i32) as u32;
        let nh = ((ny+size.1) as i32 - ny as i32) as u32;
        Self{
            off: Offset{
                x: self.off.x + nx as i32,
                y: self.off.y + ny as i32,
            },
            size: Dims{w: nw, h: nh},
        }
    }

    #[inline]
    pub fn from_ori(par_off: i32, unpar_off: i32, par_size: u32, unpar_size: u32, o: Orientation) -> Self {
        match o {
            Orientation::Horizontal => Self::from_xywh(par_off,unpar_off,par_size,unpar_size),
            Orientation::Vertical => Self::from_xywh(unpar_off,par_off,unpar_size,par_size),
        }
    }

    #[inline]
    pub fn par(&self, o: Orientation) -> (i32,u32) { //TODO improve with negate orientation
        match o {
            Orientation::Horizontal => (self.off.x,self.size.w),
            Orientation::Vertical => (self.off.y,self.size.h),
        }
    }
    #[inline]
    pub fn unpar(&self, o: Orientation) -> (i32,u32) {
        match o {
            Orientation::Horizontal => (self.off.y,self.size.h),
            Orientation::Vertical => (self.off.x,self.size.w),
        }
    }

    #[inline]
    pub fn not_empty(&self) -> bool {
        self.size.not_empty()
    }

    #[inline]
    pub fn shift_to_fit(&mut self, inner_abs: &Bounds) {
        #[inline]
        fn shift_to_fit_axis(axis: &mut (i32,i32), inner_abs: (i32,i32)) {
            if axis.1 < inner_abs.1 {
                axis.0 += inner_abs.1 - axis.1;
                axis.1 += inner_abs.1 - axis.1;
            }
            if axis.0 > inner_abs.0 {
                axis.0 -= axis.0 - inner_abs.0;
                axis.1 -= axis.0 - inner_abs.0;
            }
        }
        let mut xx = (self.off.x,self.x1());
        let mut yy = (self.off.y,self.y1());
        shift_to_fit_axis(&mut xx, (inner_abs.off.x,inner_abs.x1()) );
        shift_to_fit_axis(&mut yy, (inner_abs.off.y,inner_abs.y1()) );
        self.off.x = xx.0; self.size.w = (xx.1-xx.0) as u32;
        self.off.y = yy.0; self.size.h = (yy.1-yy.0) as u32;
    }

    pub fn overlap(&self, o: &Bounds) -> bool {
        (o.x() < self.x1() || o.x() == self.x()) &&
        (o.x1() > self.x() || o.x1() == self.x1()) &&
        (o.y() < self.y1() || o.y() == self.y()) &&
        (o.y1() > self.y() || o.y1() == self.y1())
    }
}

impl Offset {
    /// If the offset is inside the bound b
    #[inline]
    pub fn is_inside(&self, b: &Bounds) -> bool {
        self.x >= b.x() && self.x < b.x1() &&
        self.y >= b.y() && self.y < b.y1()
    }

    #[inline]
    pub fn par(&self, o: Orientation) -> i32 {
        match o {
            Orientation::Horizontal => self.x,
            Orientation::Vertical => self.y,
        }
    }
    #[inline]
    pub fn unpar(&self, o: Orientation) -> i32 {
        match o {
            Orientation::Horizontal => self.y,
            Orientation::Vertical => self.x,
        }
    }

    #[inline]
    pub fn from_ori(par: i32, unpar: i32, o: Orientation) -> Self {
        match o {
            Orientation::Horizontal => Self{x: par, y: unpar},
            Orientation::Vertical => Self{x: unpar, y: par},
        }
    }
}

impl Dims {
    /// Parallel axis of orientation e.g. Horizontal=>x Vertical=>y
    #[inline]
    pub fn par(&self, o: Orientation) -> u32 {
        match o {
            Orientation::Horizontal => self.w,
            Orientation::Vertical => self.h,
        }
    }
    /// Non-parallel axis of orientation e.g. Horizontal=>y Vertical=>x
    #[inline]
    pub fn unpar(&self, o: Orientation) -> u32 {
        match o {
            Orientation::Horizontal => self.h,
            Orientation::Vertical => self.w,
        }
    }

    /// Dims from orientation and parallel and non-parallel axis
    #[inline]
    pub fn from_ori(par: u32, unpar: u32, o: Orientation) -> Self {
        match o {
            Orientation::Horizontal => Self{w: par, h: unpar},
            Orientation::Vertical => Self{w: unpar, h: par},
        }
    }

    #[inline]
    pub fn not_empty(&self) -> bool {
        self.w > 0 && self.h > 0
    }
}

impl<T,U> From<(T,U)> for Dims where T: Into<u32>, U: Into<u32> {
    #[inline]
    fn from(s: (T,U)) -> Self {
        Self{w: s.0.into(), h: s.1.into()}
    }
}

/*impl<T,U> From<(T,U)> for Offset where T: Into<i32>, U: Into<i32> {
    fn from(s: (T,U)) -> Self {
        Self{x: s.0.into(), y: s.1.into()}
    }
}*/

// TODO richer casting over num trait, also reverse impl

impl From<(i32,i32)> for Offset {
    #[inline]
    fn from(s: (i32,i32)) -> Self {
        Self{x: s.0, y: s.1}
    }
}
impl From<(u32,u32)> for Offset {
    #[inline]
    fn from(s: (u32,u32)) -> Self {
        Self{x: s.0 as i32, y: s.1 as i32}
    }
}
impl From<Offset> for (i32,i32) {
    fn from(value: Offset) -> Self {
        (value.x,value.y)
    }
}
impl AsRef<Offset> for Offset {
    #[inline]
    fn as_ref(&self) -> &Offset {
        self
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

qwutils::opion!(bitand(Bounds,Bounds) |s,r| {
    let nx = s.off.x.max(r.off.x);
    let ny = s.off.y.max(r.off.y);
    let nx1 = s.x1().min(r.x1());
    let ny1 = s.y1().min(r.y1());
    *s = Bounds{
        off: Offset{
            x: nx,
            y: ny,
        },
        size: Dims{
            w: (nx1 - nx).max(0) as u32,
            h: (ny1 - ny).max(0) as u32,
        }
    }
});

impl Debug for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"X{}Y{}W{}H{}",self.off.x,self.off.y,self.size.w,self.size.h)
    }
}
impl Debug for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"X{}Y{}",self.x,self.y)
    }
}
impl Debug for Dims {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"W{}H{}",self.w,self.h)
    }
}
