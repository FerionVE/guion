use super::*;
use std::borrow::Cow;
use std::{ffi::{OsStr,OsString}, path::*};
use crate::validation::validated::Validated;

pub trait Caption<'w,E> {
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's;
    #[inline]
    fn len<'s>(&'s self) -> usize where 'w: 's {
        self.caption().len()
    }
}

impl<'w,E> Caption<'w,E> for str {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        Cow::Borrowed(self)
    }
}
impl<'w,E> Caption<'w,E> for String {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        Cow::Borrowed(self)
    }
}

impl<'w,E> Caption<'w,E> for Path {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}
impl<'w,E> Caption<'w,E> for PathBuf {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}

impl<'w,E> Caption<'w,E> for OsStr {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}
impl<'w,E> Caption<'w,E> for OsString {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        self.to_string_lossy()
    }
}

impl<'w,'l,E,T> Caption<'w,E> for &'w T where T: Caption<'l,E>+?Sized, 'l: 'w {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        (**self).caption()
    }
}
impl<'w,'l,E,T> Caption<'w,E> for &'w mut T where T: Caption<'l,E>+?Sized, 'l: 'w {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        (**self).caption()
    }
}

impl<'w,E,T> Caption<'w,E> for Validated<E,T> where T: Caption<'w,E> {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> where 'w: 's {
        (**self).caption()
    }
}
impl<'w,E,T> CaptionMut<'w,E> for Validated<E,T> where T: CaptionMut<'w,E> {
    fn push<'s>(&'s mut self, off: usize, s: &str) where 'w: 's {
        (**self).push(off,s)
    }
    fn pop_left<'s>(&'s mut self, off: usize, n: usize) where 'w: 's {
        (**self).pop_left(off,n)
    }
    fn replace<'s>(&'s mut self, s: &str) where 'w: 's {
        (**self).replace(s)
    }
}

macro_rules! impl_caption_gen {
    ($t:ty;$($tt:ty);+) => {
        impl_caption_gen!($t);
        impl_caption_gen!($($tt);*);
    };
    ($t:ty) => {
        impl<'w,E> Caption<'w,E> for $t {
            #[inline]
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

pub trait CaptionMut<'w,E>: Caption<'w,E> {
    fn push<'s>(&'s mut self, off: usize, s: &str) where 'w: 's;
    fn pop_left<'s>(&'s mut self, off: usize, n: usize) where 'w: 's;
    fn replace<'s>(&'s mut self, s: &str) where 'w: 's;
}

impl<'w,E> CaptionMut<'w,E> for String {
    fn push<'s>(&'s mut self, off: usize, s: &str) where 'w: 's {
        self.insert_str(off,s);
    }
    fn pop_left<'s>(&'s mut self, off: usize, n: usize) where 'w: 's {
        let popable = n.min(off).min(Caption::<E>::len(self));
        let pop_start = off - popable;
        for _ in 0..popable { //TODO VERY INEFFICIENT optimize
            self.remove(pop_start);
        }
    }
    fn replace<'s>(&'s mut self, s: &str) where 'w: 's {
        *self = s.to_owned(); //TODO more efficient alloc-keeping replace
    }
}

impl<'w,'l,E,T> CaptionMut<'w,E> for &'w mut T where T: CaptionMut<'l,E>+?Sized, 'l: 'w {
    fn push<'s>(&'s mut self, off: usize, s: &str) where 'w: 's {
        (**self).push(off,s)
    }
    fn pop_left<'s>(&'s mut self, off: usize, n: usize) where 'w: 's {
        (**self).pop_left(off,n)
    }
    fn replace<'s>(&'s mut self, s: &str) where 'w: 's {
        (**self).replace(s)
    }
}

unsafe impl<'w,E> Statize<E> for dyn Caption<'w,E> where E: 'static {
    type Statur = dyn Caption<'static,E>;
}
unsafe impl<'w,E> Statize<E> for dyn CaptionMut<'w,E> where E: 'static {
    type Statur = dyn CaptionMut<'static,E>;
}