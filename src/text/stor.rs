use std::borrow::Cow;
use std::ops::Range;

use crate::env::Env;
use crate::traitcast_for;

use super::layout::TxtLayout;

pub trait TextStor<E> where E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str>;
    #[inline]
    fn chars(&self) -> usize {
        self.caption().chars().count()
    }
}

pub trait TextStorMut<E>: TextStor<E> where E: Env {
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
        self.remove_chars(0..self.chars());
        self.push_chars(0,s);
    }
}

impl<E> TextStor<E> for str where E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(&self[..])
    }
}
impl<E> TextStor<E> for &str where E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(&self[..])
    }
}
impl<E> TextStor<E> for String where E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(&self[..])
    }
}

impl<E> TextStorMut<E> for String where E: Env {
    fn remove_chars(&mut self, range: Range<usize>) {
        let len = TextStor::<E>::chars(self);

        let pop_start_bin = char_off(&self, range.start);
        let pop_end_bin = char_off(&self, range.end);

        assert!(pop_end_bin >= pop_start_bin);

        self.drain(pop_start_bin..pop_end_bin);
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
        let off = char_off(&self, off);

        self.insert_str(off, chars);
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

impl<E,T> TextStor<E> for crate::validation::validated::Validated<E,T> where T: TextStor<E>, E: Env {
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }

    fn chars(&self) -> usize {
        (**self).chars()
    }
}
impl<E,T> TextStorMut<E> for crate::validation::validated::Validated<E,T> where T: TextStorMut<E>, E: Env {
    fn remove_chars(&mut self, range: Range<usize>) {
        (**self).remove_chars(range)
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
        (**self).push_chars(off,chars)
    }
}
