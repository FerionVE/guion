use super::*;

/// Size +/+= Border
#[derive(Clone,Copy)]
pub struct StdGonstraints {
    pub x: StdGonstraintAxis,
    pub y: StdGonstraintAxis,
}

/// The SizeAxis holds layouting constraints for one axis
/// Supported Operators: Add, BitAnd
#[derive(Clone,Copy)]
pub struct StdGonstraintAxis {
    pub min: u32,
    pub preferred: u32,
    pub max: Option<u32>,
    pub pressure: f32,
}

impl StdGonstraints {
    #[inline]
    pub const fn fixed_const(w: u32, h: u32) -> Self {
        Self{
            x: StdGonstraintAxis{
                min: w,
                preferred: w,
                max: Some(w),
                pressure: 1.0,
            },
            y: StdGonstraintAxis{
                min: h,
                preferred: h,
                max: Some(h),
                pressure: 1.0,
            }
        }
    }
}

impl StdGonstraintAxis {
    #[inline]
    pub const fn empty_const() -> Self {
        StdGonstraintAxis {
            min: 0,
            preferred: 0,
            max: None,
            pressure: 1.0,
        }
    }

    #[inline]
    pub const fn fixed_const(s: u32) -> Self {
        StdGonstraintAxis {
            min: s,
            preferred: s,
            max: Some(s),
            pressure: 0.0,
        }
    }
}

#[macro_export]
macro_rules! constraint {
    (# $min:literal ~ $pref:literal - $max:tt @ $p:literal | $($m:tt)*) => {
        $crate::layout::size::StdGonstraints{
            x: $crate::constraint!(#$min ~ $pref - $max @ $p),
            y: $crate::constraint!($($m)*),
        }
    };
    (# $min:literal ~ $pref:literal - None @ $p:literal) => {
        $crate::layout::size::StdGonstraintAxis{min:$min,preferred:$pref,max:None,pressure:$p}
    };
    (# $min:literal ~ $pref:literal - $max:literal @ $p:literal) => {
        $crate::layout::size::StdGonstraintAxis{min:$min,preferred:$pref,max:Some($max),pressure:$p}
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
