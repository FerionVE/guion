use super::*;
use std::borrow::Cow;
use std::{ffi::{OsStr,OsString}, path::*};
use crate::validation::validated::Validated;

pub trait Caption<E> {
    fn caption<'s>(&'s self) -> Cow<'s,str>;
    #[inline]
    fn len(&self) -> usize {
        self.caption().len()
    }

    fn try_push(&mut self, off: usize, s: &str);
    fn try_pop_left(&mut self, off: usize, n: usize);
    fn try_replace(&mut self, s: &str);
}

impl<E> Caption<E> for str {
    #[inline]
    fn caption(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }

    #[inline] fn try_push(&mut self, off: usize, s: &str) {}
    #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
    #[inline] fn try_replace(&mut self, s: &str) {}
}
impl<E> Caption<E> for String {
    #[inline]
    fn caption(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }

    #[inline]
    fn try_push(&mut self, off: usize, s: &str) {
        CaptionMut::<E>::push(self,off,s)
    }
    #[inline]
    fn try_pop_left(&mut self, off: usize, n: usize) {
        CaptionMut::<E>::pop_left(self,off,n)
    }
    #[inline]
    fn try_replace(&mut self, s: &str) {
        CaptionMut::<E>::replace(self,s)
    }
}

impl<E> Caption<E> for Path {
    #[inline]
    fn caption(&self) -> Cow<str> {
        self.to_string_lossy()
    }

    #[inline] fn try_push(&mut self, off: usize, s: &str) {}
    #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
    #[inline] fn try_replace(&mut self, s: &str) {}
}
impl<E> Caption<E> for PathBuf {
    #[inline]
    fn caption(&self) -> Cow<str> {
        self.to_string_lossy()
    }

    #[inline] fn try_push(&mut self, off: usize, s: &str) {}
    #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
    #[inline] fn try_replace(&mut self, s: &str) {}
}

impl<E> Caption<E> for OsStr {
    #[inline]
    fn caption(&self) -> Cow<str> {
        self.to_string_lossy()
    }

    #[inline] fn try_push(&mut self, off: usize, s: &str) {}
    #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
    #[inline] fn try_replace(&mut self, s: &str) {}
}
impl<E> Caption<E> for OsString {
    #[inline]
    fn caption(&self) -> Cow<str> {
        self.to_string_lossy()
    }

    #[inline] fn try_push(&mut self, off: usize, s: &str) {}
    #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
    #[inline] fn try_replace(&mut self, s: &str) {}
}

impl<E,T> Caption<E> for &'_ T where T: Caption<E>+?Sized {
    #[inline]
    fn caption(&self) -> Cow<str> {
        (**self).caption()
    }

    #[inline] fn try_push(&mut self, off: usize, s: &str) {}
    #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
    #[inline] fn try_replace(&mut self, s: &str) {}
}
impl<'l,E,T> Caption<E> for &'_ mut T where T: Caption<E>+?Sized {
    #[inline]
    fn caption(&self) -> Cow<str> {
        (**self).caption()
    }

    #[inline]
    fn try_push(&mut self, off: usize, s: &str) {
        (**self).try_push(off,s)
    }
    #[inline]
    fn try_pop_left(&mut self, off: usize, n: usize) {
        (**self).try_pop_left(off,n)
    }
    #[inline]
    fn try_replace(&mut self, s: &str) {
        (**self).try_replace(s)
    }
}

impl<E,T> Caption<E> for Validated<E,T> where T: Caption<E> {
    #[inline]
    fn caption(&self) -> Cow<str> {
        (**self).caption()
    }

    #[inline]
    fn try_push(&mut self, off: usize, s: &str) {
        (**self).try_push(off,s)
    }
    #[inline]
    fn try_pop_left(&mut self, off: usize, n: usize) {
        (**self).try_pop_left(off,n)
    }
    #[inline]
    fn try_replace(&mut self, s: &str) {
        (**self).try_replace(s)
    }
}
impl<E,T> CaptionMut<E> for Validated<E,T> where T: CaptionMut<E> {
    fn push(&mut self, off: usize, s: &str) {
        (**self).push(off,s)
    }
    fn pop_left(&mut self, off: usize, n: usize) {
        (**self).pop_left(off,n)
    }
    fn replace(&mut self, s: &str) {
        (**self).replace(s)
    }
}

macro_rules! impl_caption_gen {
    ($t:ty;$($tt:ty);+) => {
        impl_caption_gen!($t);
        impl_caption_gen!($($tt);*);
    };
    ($t:ty) => {
        impl<E> Caption<E> for $t {
            #[inline]
            fn caption(&self) -> Cow<str> {
                Cow::Owned(self.to_string())
            }

            #[inline] fn try_push(&mut self, off: usize, s: &str) {}
            #[inline] fn try_pop_left(&mut self, off: usize, n: usize) {}
            #[inline] fn try_replace(&mut self, s: &str) {}
        }
    }
}

impl_caption_gen!(
    bool;char;
    f32;f64;
    i8;i16;i32;i64;i128;isize;
    u8;u16;u32;u64;u128;usize
);

pub trait CaptionMut<E>: Caption<E> {
    fn push(&mut self, off: usize, s: &str);
    fn pop_left(&mut self, off: usize, n: usize);
    fn replace(&mut self, s: &str);
}

impl<E> CaptionMut<E> for String {
    fn push(&mut self, off: usize, s: &str) {
        self.insert_str(off,s);
    }
    fn pop_left(&mut self, off: usize, n: usize) {
        let popable = n.min(off).min(Caption::<E>::len(self));
        let pop_start = off - popable;
        for _ in 0..popable { //TODO VERY INEFFICIENT optimize
            self.remove(pop_start);
        }
    }
    fn replace(&mut self, s: &str) {
        *self = s.to_owned(); //TODO more efficient alloc-keeping replace
    }
}

impl<E,T> CaptionMut<E> for &'_ mut T where T: CaptionMut<E>+?Sized {
    fn push(&mut self, off: usize, s: &str) {
        (**self).push(off,s)
    }
    fn pop_left(&mut self, off: usize, n: usize) {
        (**self).pop_left(off,n)
    }
    fn replace(&mut self, s: &str) {
        (**self).replace(s)
    }
}

unsafe impl<E> Statize<E> for dyn Caption<E> where E: 'static {
    type Statur = dyn Caption<E>+'static;
}
unsafe impl<E> Statize<E> for dyn CaptionMut<E> where E: 'static {
    type Statur = dyn CaptionMut<E>+'static;
}

traitcast_for!(Caption<E>);
