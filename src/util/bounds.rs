//! Bounds (x,y,w,h) and functionality
use super::*;

#[derive(Clone,Copy,Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone,Copy,Default)]
pub struct Dims {
    pub w: u32,
    pub h: u32,
}
#[derive(Clone,Copy,Default)]
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
    pub const fn from_xy(x: i32, y: i32) -> Self {
        Self::from_xywh(x,y,0,0)
    }
    pub const fn from_wh(w: u32, h: u32) -> Self {
        Self::from_xywh(0,0,w,h)
    }
    pub const fn from_off(off: Offset) -> Self {
        Self::from_xywh(off.x,off.y,0,0)
    }
    pub const fn from_size(size: Dims) -> Self {
        Self::from_xywh(0,0,size.w,size.h)
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

    /// get the bounds inside this bound (subtract border)
    pub fn inside_border(&self, b: &Border) -> Self {
        let mut s = *self;
        s.off += b.inner();
        s.size -= b.border_effective();
        s
    }

    /// get the part of the inner which also is inside this bound
    /// 
    /// Use the BitAnd operator if the inner bound is absolute
    pub fn slice(&self, inner_relative: &Bounds) -> Self {
        let b = inner_relative;
        Self{
            off: &self.off + &b.off,
            size: Dims{
                w: b.size.w .min( (self.size.w as i32).saturating_sub(b.off.x) as u32 ),
                h: b.size.h .min( (self.size.h as i32).saturating_sub(b.off.y) as u32 ),
            }
        }
    }
    //TODO doc
    pub fn step(&self, step: i32) -> Self {
        let mut senf = *self;
        senf.off.x += step;
        senf.off.y += step;
        senf.size.w = (senf.size.w as i32 - step*2).max(0) as u32;
        senf.size.h = (senf.size.h as i32 - step*2).max(0) as u32;
        senf
    }
    /// get bound with size s and centered relative to self
    pub fn inner_centered(&self, size: Dims) -> Self {
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
    /// inner_centered but advanced
    /// align is the start-to-end relative position (0.0 - 1.0)
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

    pub fn from_ori(par_off: i32, unpar_off: i32, par_size: u32, unpar_size: u32, o: Orientation) -> Self {
        match o {
            Orientation::Horizontal => Self::from_xywh(par_off,unpar_off,par_size,unpar_size),
            Orientation::Vertical => Self::from_xywh(unpar_off,par_off,unpar_size,par_size),
        }
    }

    pub fn par(&self, o: Orientation) -> (i32,u32) { //TODO improve with negate orientation
        match o {
            Orientation::Horizontal => (self.off.x,self.size.w),
            Orientation::Vertical => (self.off.y,self.size.h),
        }
    }
    pub fn unpar(&self, o: Orientation) -> (i32,u32) {
        match o {
            Orientation::Horizontal => (self.off.y,self.size.h),
            Orientation::Vertical => (self.off.x,self.size.w),
        }
    }
}

impl Offset {
    /// if the offset is inside the bound b
    pub fn is_inside(&self, b: &Bounds) -> bool {
        self.x >= b.x() && self.x < b.x1() &&
        self.y >= b.y() && self.y < b.y1()
    }

    pub fn par(&self, o: Orientation) -> i32 {
        match o {
            Orientation::Horizontal => self.x,
            Orientation::Vertical => self.y,
        }
    }
    pub fn unpar(&self, o: Orientation) -> i32 {
        match o {
            Orientation::Horizontal => self.y,
            Orientation::Vertical => self.x,
        }
    }

    pub fn from_ori(par: i32, unpar: i32, o: Orientation) -> Self {
        match o {
            Orientation::Horizontal => Self{x: par, y: unpar},
            Orientation::Vertical => Self{x: unpar, y: par},
        }
    }
}

impl Dims {
    pub fn par(&self, o: Orientation) -> u32 {
        match o {
            Orientation::Horizontal => self.w,
            Orientation::Vertical => self.h,
        }
    }
    pub fn unpar(&self, o: Orientation) -> u32 {
        match o {
            Orientation::Horizontal => self.h,
            Orientation::Vertical => self.w,
        }
    }

    pub fn from_ori(par: u32, unpar: u32, o: Orientation) -> Self {
        match o {
            Orientation::Horizontal => Self{w: par, h: unpar},
            Orientation::Vertical => Self{w: unpar, h: par},
        }
    }
}

impl<T,U> From<(T,U)> for Dims where T: Into<u32>, U: Into<u32> {
    fn from(s: (T,U)) -> Self {
        Self{w: s.0.into(), h: s.1.into()}
    }
}

impl<T,U> From<(T,U)> for Offset where T: Into<i32>, U: Into<i32> {
    fn from(s: (T,U)) -> Self {
        Self{x: s.0.into(), y: s.1.into()}
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
    *s = Bounds{
        off: Offset{
            x: nx,
            y: ny,
        },
        size: Dims{
            w: (s.x1().min(r.x1()) - nx).max(0) as u32,
            h: (s.y1().min(r.y1()) - ny).max(0) as u32,
        }
    }
});
