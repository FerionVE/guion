use std::any::Any;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Range;
use std::sync::Arc;

use crate::env::Env;
use crate::traitcast_for;
use crate::util::translate::immu::Immutable;

pub trait TextStor<E> {
    fn caption<'s>(&'s self) -> Cow<'s,str>;
    #[inline]
    fn chars(&self) -> usize {
        self.caption().chars().count()
    }

    #[inline]
    fn len(&self) -> usize {
        self.caption().len()
    }

    #[inline]
    fn immutable(self) -> Immutable<E,Self,()> where Self: Sized {
        Immutable(PhantomData,self)
    }
}

pub trait TextStorMut<E>: TextStor<E> {
    fn remove_chars(&mut self, range: Range<usize>);

    fn remove_chars_old(&mut self, off: usize, n: usize) {
        let len = TextStor::<E>::chars(self);
        let popable = n.min(off).min(len);
        let pop_start = off - popable;
        let pop_end = off;

        assert!(pop_end >= pop_start);



        self.remove_chars(pop_start..pop_end)
    }
    /// off in char units
    fn push_chars(&mut self, off: usize, chars: &str);

    fn replace(&mut self, s: &str) {
        self.remove_chars(0..self.len());
        self.push_chars(0,s);
    }

    #[inline]
    fn on_modification<F>(self, f: F) -> OnModification<E,Self,F> where Self: Sized, F: FnMut(&mut Self) {
        OnModification(f,PhantomData,self)
    }
}

impl<E> TextStor<E> for str  {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(&self[..])
    }
}
impl<E> TextStor<E> for &str {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(&self[..])
    }
}
impl<E> TextStor<E> for String {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(&self[..])
    }
}

impl<E> TextStorMut<E> for String {
    fn remove_chars(&mut self, range: Range<usize>) {
        let range = fix_boundary(self, range.start) .. fix_boundary(self, range.end);
        self.drain(range);
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
        let off = fix_boundary(self, off);
        self.insert_str(off, chars);
    }
}

impl<E,A> TextStor<E> for &A where A: TextStor<E>, E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }

    fn chars(&self) -> usize {
        (**self).chars()
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<E,A> TextStor<E> for &mut A where A: TextStor<E>, E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }

    fn chars(&self) -> usize {
        (**self).chars()
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A> TextStorMut<E> for &mut A where A: TextStorMut<E>, E: Env {
    fn remove_chars(&mut self, range: Range<usize>) {
        (**self).remove_chars(range)
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
        (**self).push_chars(off,chars)
    }

    fn remove_chars_old(&mut self, off: usize, n: usize) {
        (**self).remove_chars_old(off,n)
    }

    fn replace(&mut self, s: &str) {
        (**self).replace(s)
    }
}

traitcast_for!(TextStor<E>;TextStorMut<E>);

fn char_off(s: impl AsRef<str>, o: usize) -> usize {
    let s = s.as_ref();
    match s.char_indices().skip(o).next() {
        Some((i,_)) => i,
        None => s.len(),
    }
}

pub struct OnModification<E,S: ?Sized,F>(F,PhantomData<E>,S) where F: FnMut(&mut S);

impl<E,S,F> Deref for OnModification<E,S,F> where F: FnMut(&mut S) {
    type Target = S;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.2
    }
}
impl<E,S,F> DerefMut for OnModification<E,S,F> where F: FnMut(&mut S) {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.2
    }
}

impl<E,S,F> TextStor<E> for OnModification<E,S,F> where S: TextStor<E>, F: FnMut(&mut S) {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }

    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,S,F> TextStorMut<E> for OnModification<E,S,F> where S: TextStorMut<E>, F: FnMut(&mut S) {
    #[inline]
    fn remove_chars(&mut self, range: Range<usize>) {
        (**self).remove_chars(range);
        (self.0)(&mut self.2);
    }

    #[inline]
    fn push_chars(&mut self, off: usize, chars: &str) {
        (**self).push_chars(off,chars);
        (self.0)(&mut self.2);
    }

    #[inline]
    fn remove_chars_old(&mut self, off: usize, n: usize) {
        (**self).remove_chars_old(off,n);
        (self.0)(&mut self.2);
    }

    #[inline]
    fn replace(&mut self, s: &str) {
        (**self).replace(s);
        (self.0)(&mut self.2);
    }
}

macro_rules! impl_caption_gen {
    ($t:ty;$($tt:ty);+) => {
        impl_caption_gen!($t);
        impl_caption_gen!($($tt);*);
    };
    ($t:ty) => {
        impl<E> TextStor<E> for $t {
            #[inline]
            fn caption(&self) -> Cow<str> {
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

pub fn fix_boundary(s: &str, mut off: usize) -> usize {
    while !s.is_char_boundary(off) && off!=0 {
        off = off.saturating_sub(1); //TODO efficient algorithm
    }
    off
}
