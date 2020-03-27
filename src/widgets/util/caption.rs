use super::*;
use std::borrow::Cow;
use std::{ffi::{OsStr,OsString}, path::*};

pub trait Caption<'w> {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's;
}

impl<'w> Caption<'w> for str {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        Cow::Borrowed(self)
    }
}
impl<'w> Caption<'w> for String {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        Cow::Borrowed(self)
    }
}

impl<'w> Caption<'w> for Path {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}
impl<'w> Caption<'w> for PathBuf {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}

impl<'w> Caption<'w> for OsStr {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}
impl<'w> Caption<'w> for OsString {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}

impl<'w,'l,T> Caption<'w> for &'w T where T: Caption<'l>, 'l: 'w {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        (**self).caption()
    }
}
impl<'w,'l,T> Caption<'w> for &'w mut T where T: Caption<'l>, 'l: 'w {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        (**self).caption()
    }
}

macro_rules! impl_caption_gen {
    ($t:ty;$($tt:ty);+) => {
        impl_caption_gen!($t);
        impl_caption_gen!($($tt);*);
    };
    ($t:ty) => {
        impl<'w> Caption<'w> for $t {
            fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
                Cow::Owned(self.to_string())
            }
        }
    }
}

impl_caption_gen!(
    bool;char;
    f32;f64;
    i8;i16;i32;i64;i128;isize;
    u8;u16;u32;u64;u128;usize
);