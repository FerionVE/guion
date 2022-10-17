/// Size +/+= Border
#[derive(Clone,Copy,PartialEq)]
pub struct StdGonstraints {
    pub x: StdGonstraintAxis,
    pub y: StdGonstraintAxis,
}

/// The SizeAxis holds layouting constraints for one axis
/// Supported Operators: Add, BitAnd
#[derive(Clone,Copy,PartialEq)]
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

/// Defines constraints for one or both axis
///
/// `preferred` is the preferred/optimal size  
/// `min` is the minimum size if e.g. bounds have to be shrinked below preferred  
/// `max` is the maximum size if e.g. bounds are extended  
///
/// Syntax:
///
/// `constraint!( AXIS w [| AXIS y] )`  
/// `AXIS [[min]~]preferred[-[max]]`  
/// 
/// `[]`: Optional argument  
/// `preferred`: The preferred size  
/// `~`: If given, The minimum size can be smaller than preferred (default: 0)  
/// `min`: Additional to `~`, limits the minimum size to the specific min  
/// `-`: If given, the maximum size can be bigger than preferred (default: infinity)  
/// `max`: Additional to `-`, limits the maximum size to the specific max  
/// 
/// Example:
/// 
/// `5`: Fixed size 5  
/// `~5`: Preferred 5, can be smaller (shrinked)  
/// `2~5`: Preferred 5, can bes smaller, but not smaller than 2  
/// `5-`: Preferred 5, can be bigger (extended)  
/// `5-8`: Preferred 5, can be bigger, but not bigger than 8  
#[macro_export]
macro_rules! constraint {
    ($min:tt ~ $pref:tt - $max:tt @ $p:tt | $($m:tt)*) => {
        $crate::layout::size::StdGonstraints{
            x: $crate::constraint!($min ~ $pref - $max @ $p),
            y: $crate::constraint!($($m)*),
        }
    };

    ($min:tt ~ $pref:tt - None @ $p:tt) => {
        $crate::layout::size::StdGonstraintAxis{min:$min,preferred:$pref,max:None,pressure:$p}
    };
    ($min:tt ~ $pref:tt - $max:tt @ $p:tt) => {
        $crate::layout::size::StdGonstraintAxis{min:$min,preferred:$pref,max:Some($max),pressure:$p}
    };

    ($min:tt ~ $pref:tt - $max:tt $(| $($m:tt)*)?) => {
        $crate::constraint!($min ~ $pref - $max @ 1.0 $(| $($m)*)?)
    };
    ($min:tt ~ $pref:tt - $(@ $p:tt)? $(| $($m:tt)*)?) => {
        $crate::constraint!($min ~ $pref - None $(@ $p)? $(| $($m)*)?)
    };
    ($min:tt ~ $pref:tt $(@ $p:tt)? $(| $($m:tt)*)?) => {
        $crate::constraint!($min ~ $pref - $pref $(@ $p)? $(| $($m)*)?)
    };

    (~ $pref:tt $($m:tt)*) => {
        $crate::constraint!(0 ~ $pref $($m)*)
    };
    ($pref:tt $($m:tt)*) => {
        $crate::constraint!($pref ~ $pref $($m)*)
    };
}
