use super::*;

/// Size +/+= Border
#[derive(Clone)]
pub struct Size {
    pub x: SizeAxis,
    pub y: SizeAxis,
}

/// The SizeAxis holds layouting constraints for one axis
/// Supported Operators: Add, BitAnd
#[derive(Clone)]
pub struct SizeAxis {
    pub min: u32,
    pub preferred: u32,
    pub max: Option<u32>,
    pub pressure: f32,
}

impl Size {
    #[inline]
    pub const fn fixed(w: u32, h: u32) -> Self {
        Self{
            x: SizeAxis{
                min: w,
                preferred: w,
                max: Some(w),
                pressure: 1.0,
            },
            y: SizeAxis{
                min: h,
                preferred: h,
                max: Some(h),
                pressure: 1.0,
            }
        }
    }

    #[inline]
    pub fn add(&mut self, o: &Self, dir: Orientation) {
        match dir {
            Orientation::Horizontal => self.add_x(o),
            Orientation::Vertical => self.add_y(o),
        }
    }

    #[inline]
    pub fn add_x(&mut self, o: &Self) {
        self.x += &o.x;
        self.y &= &o.y;
    }

    #[inline]
    pub fn add_y(&mut self, o: &Self) {
        self.x &= &o.x;
        self.y += &o.y;
    }

    #[inline]
    pub fn max(&self, o: &Self) -> Self {
        Self{
            x: self.x.max(&o.x),
            y: self.y.max(&o.y),
        }
    }

    #[inline]
    pub const fn empty() -> Self {
        Size {
            x: SizeAxis::empty(),
            y: SizeAxis::empty(),
        }
    }

    #[inline]
    pub fn flip(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y)
    }

    #[inline]
    pub fn par(&self, dir: Orientation) -> &SizeAxis {
        match dir {
            Orientation::Horizontal => &self.x,
            Orientation::Vertical => &self.y,
        }
    }
    #[inline]
    pub fn unpar(&self, dir: Orientation) -> &SizeAxis {
        match dir {
            Orientation::Horizontal => &self.y,
            Orientation::Vertical => &self.x,
        }
    }

    #[inline]
    pub fn from_parallel(par: SizeAxis, unpar: SizeAxis, dir: Orientation) -> Self {
        match dir {
            Orientation::Horizontal => Self{x: par, y: unpar},
            Orientation::Vertical => Self{x: unpar, y: par},
        }
    }
}

impl SizeAxis {
    #[inline]
    pub const fn empty() -> Self {
        SizeAxis {
            min: 0,
            preferred: 0,
            max: None,
            pressure: 1.0,
        }
    }

    #[inline]
    pub const fn fixed(s: u32) -> Self {
        SizeAxis {
            min: s,
            preferred: s,
            max: Some(s),
            pressure: 0.0,
        }
    }

    #[inline]
    pub fn max(&self, r: &SizeAxis) -> Self {
        let mut s = self.clone();

        s.min = s.min.max(r.min);
        s.preferred = s.preferred.max(r.preferred);
        s.max = s.max.with_if( &r.max, #[inline] |s,r| (*s).max(*r) );

        s.max.map(#[inline] |m| s.preferred = s.preferred.min(m) );
        s.preferred = s.preferred.max(s.min);
        
        s.pressure = s.pressure.max(r.pressure);

        s
    }
}

#[macro_export]
macro_rules! constraint {
    (# $min:literal ~ $pref:literal - $max:tt @ $p:literal | $($m:tt)*) => {
        $crate::layout::size::Size{
            x: $crate::constraint!(#$min ~ $pref - $max @ $p),
            y: $crate::constraint!($($m)*),
        }
    };
    (# $min:literal ~ $pref:literal - None @ $p:literal) => {
        SizeAxis{min:$min,preferred:$pref,max:None,pressure:$p}
    };
    (# $min:literal ~ $pref:literal - $max:literal @ $p:literal) => {
        $crate::layout::size::SizeAxis{min:$min,preferred:$pref,max:Some($max),pressure:$p}
    };
    (# $min:literal ~ $pref:literal - $max:tt $($m:tt)*) => {
        $crate::constraint!(#$min ~ $pref - $max @ 1.0 $($m)*)
    };
    ($min:literal ~ $pref:literal - $max:literal $($m:tt)*) => {
        $crate::constraint!(#$min ~ $pref - $max $($m)*)
    };
    ($min:literal ~ $pref:literal - $($m:tt)*) => {
        $crate::constraint!(#$min ~ $pref - None $($m)*)
    };
    ($min:literal ~ $pref:literal $($m:tt)*) => {
        $crate::constraint!(#$min ~ $pref - $pref $($m)*)
    };
    (~ $pref:literal $($m:tt)*) => {
        $crate::constraint!(0 ~ $pref $($m)*)
    };
    ($pref:literal $($m:tt)*) => {
        $crate::constraint!($pref ~ $pref $($m)*)
    };
}
