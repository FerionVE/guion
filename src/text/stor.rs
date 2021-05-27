use std::borrow::Cow;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Range;

use crate::env::Env;
use crate::traitcast_for;
use crate::validation::Validation;
use crate::validation::ValidationMut;
use crate::validation::validated::Validated;
use crate::widgets::util::caption::Caption;
use crate::widgets::util::caption::CaptionMut;
use crate::widgets::util::state::AtomState;

use super::layout::TxtLayout;

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

    fn on_modification<F>(self, f: F) -> OnModification<Self,F> where Self: Sized, F: FnMut(&mut Self) {
        OnModification(self,f)
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
        self.drain(range);
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
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

impl<E,T> TextStor<E> for Validated<E,T> where T: TextStor<E>, E: Env {
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
impl<E,T> TextStorMut<E> for Validated<E,T> where T: TextStorMut<E>, E: Env {
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

pub struct OnModification<S,F>(S,F) where F: FnMut(&mut S);

impl<S,F> Deref for OnModification<S,F> where F: FnMut(&mut S) {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<S,F> DerefMut for OnModification<S,F> where F: FnMut(&mut S) {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E,S,F> Validation<E> for OnModification<S,F> where S: Validation<E>, F: FnMut(&mut S) {
    fn valid(&self, v: &dyn std::any::Any) -> bool {
        (**self).valid(v)
    }
}
impl<E,S,F> ValidationMut<E> for OnModification<S,F> where S: ValidationMut<E>, F: FnMut(&mut S) {
    fn validate(&mut self) -> std::sync::Arc<dyn std::any::Any> {
        (**self).validate()
        //TODO trigger OnModification?
    }
}

impl<E,S,F> TextStor<E> for OnModification<S,F> where S: TextStor<E>, F: FnMut(&mut S) {
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
impl<E,S,F> TextStorMut<E> for OnModification<S,F> where S: TextStorMut<E>, F: FnMut(&mut S) {
    fn remove_chars(&mut self, range: Range<usize>) {
        (**self).remove_chars(range);
        (self.1)(&mut self.0);
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
        (**self).push_chars(off,chars);
        (self.1)(&mut self.0);
    }

    fn remove_chars_old(&mut self, off: usize, n: usize) {
        (**self).remove_chars_old(off,n);
        (self.1)(&mut self.0);
    }

    fn replace(&mut self, s: &str) {
        (**self).replace(s);
        (self.1)(&mut self.0);
    }
}

impl<E,S,F> Caption<E> for OnModification<S,F> where S: Caption<E>, F: FnMut(&mut S) {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,S,F> CaptionMut<E> for OnModification<S,F> where S: CaptionMut<E>, F: FnMut(&mut S) {
    fn push(&mut self, off: usize, s: &str) {
        (**self).push(off,s);
        (self.1)(&mut self.0);
    }

    fn pop_left(&mut self, off: usize, n: usize) {
        (**self).pop_left(off,n);
        (self.1)(&mut self.0);
    }

    fn replace(&mut self, s: &str) {
        (**self).replace(s);
        (self.1)(&mut self.0);
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
