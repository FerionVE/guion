use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use std::ops::Range;
use std::ops::Sub;

use crate::aliases::EEKey;
use crate::env::Env;

use super::key::MatchKeyCode;
use super::key::MatchScanCode;

pub trait KeyCombo<E> where E: Env {
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>);
}

//TODO generic
pub enum MatchKey<'a,E> where E: Env {
    KeyCode(MatchKeyCode<'a>),
    ScanCode(MatchScanCode<'a>),
    EKey(EEKey<E>),
}

pub struct CNot<T>(T);
pub struct CAnd<T,U>(T,U);
pub struct COrdered<T,U>(T,U);
pub struct COr<T,U>(T,U);
pub struct CXor<T,U>(T,U);

/// Combo newtype is required to be able to implement ops trants
#[repr(transparent)]
pub struct Combo<T>(pub T);

impl<E> KeyCombo<E> for MatchKeyCode<'_> where E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        find(MatchKey::KeyCode(*self))
    }
}
impl<E> KeyCombo<E> for MatchScanCode<'_> where E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        find(MatchKey::ScanCode(*self))
    }
}

//TODO KeyCombo on EEKey<E>

impl<E,T> KeyCombo<E> for Combo<CNot<T>> where T: KeyCombo<E>, E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        let (res,range) = self.0.0.match_in(find);
        (!res,range)
    }
}

impl<E,T,U> KeyCombo<E> for Combo<CAnd<T,U>> where T: KeyCombo<E>, U: KeyCombo<E>, E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        let (res_a,range_a) = self.0.0.match_in(find);
        let (res_b,range_b) = self.0.0.match_in(find);
        (res_a & res_b, if res_a & res_b {range_range(range_a,range_b)} else {None})
    }
}

impl<E,T,U> KeyCombo<E> for Combo<COr<T,U>> where T: KeyCombo<E>, U: KeyCombo<E>, E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        let (res_a,range_a) = self.0.0.match_in(find);
        let (res_b,range_b) = self.0.0.match_in(find);
        (res_a | res_b, range_range(range_a,range_b))
    }
}

impl<E,T,U> KeyCombo<E> for Combo<CXor<T,U>> where T: KeyCombo<E>, U: KeyCombo<E>, E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        let (res_a,range_a) = self.0.0.match_in(find);
        let (res_b,range_b) = self.0.0.match_in(find);
        (res_a ^ res_b, if res_a ^ res_b {range_range(range_a,range_b)} else {None})
    }
}

impl<E,T,U> KeyCombo<E> for Combo<COrdered<T,U>> where T: KeyCombo<E>, U: KeyCombo<E>, E: Env {
    #[inline]
    fn match_in(&self, find: impl FnMut(MatchKey<'_,E>) -> (bool,Option<Range<usize>>)) -> (bool,Option<Range<usize>>) {
        let (res_a,range_a) = self.0.0.match_in(find);
        let (res_b,range_b) = self.0.0.match_in(find);
        
        match (res_a,range_a,res_b,range_b) {
            (true,None,true,None) => {
                (true,None)
            },
            (true,Some(a),true,None) => {
                (true,Some(a))
            },
            (true,None,true,Some(b)) => {
                (true,Some(b))
            },
            (true,Some(a),true,Some(b)) => {
                if a.start >= b.end {
                    (false, range_range(range_a,range_b))
                } else {
                    (true, range_range(range_a,range_b))
                }
            },
            _ => (false,None)
        }
    }
}

impl Not for MatchKeyCode<'_> {
    type Output = Combo<CNot<Self>>;
    #[inline]
    fn not(self) -> Self::Output {
        Combo(CNot(self))
    }
}
impl Not for MatchScanCode<'_> {
    type Output = Combo<CNot<Self>>;
    #[inline]
    fn not(self) -> Self::Output {
        Combo(CNot(self))
    }
}
impl<T> Not for Combo<T> {
    type Output = Combo<CNot<T>>;
    #[inline]
    fn not(self) -> Self::Output {
        Combo(CNot(self.0))
    }
}

impl<U> BitAnd<U> for MatchKeyCode<'_> {
    type Output = Combo<CAnd<Self,U>>;
    #[inline]
    fn bitand(self, rhs: U) -> Self::Output {
        Combo(CAnd(self,rhs))
    }
}
impl<U> BitAnd<U> for MatchScanCode<'_> {
    type Output = Combo<CAnd<Self,U>>;
    #[inline]
    fn bitand(self, rhs: U) -> Self::Output {
        Combo(CAnd(self,rhs))
    }
}
impl<T,U> BitAnd<U> for Combo<T> {
    type Output = Combo<CAnd<T,U>>;
    #[inline]
    fn bitand(self, rhs: U) -> Self::Output {
        Combo(CAnd(self.0,rhs))
    }
}

impl<U> Sub<U> for MatchKeyCode<'_> {
    type Output = Combo<COrdered<Self,U>>;
    #[inline]
    fn sub(self, rhs: U) -> Self::Output {
        Combo(COrdered(self,rhs))
    }
}
impl<U> Sub<U> for MatchScanCode<'_> {
    type Output = Combo<COrdered<Self,U>>;
    #[inline]
    fn sub(self, rhs: U) -> Self::Output {
        Combo(COrdered(self,rhs))
    }
}
impl<T,U> Sub<U> for Combo<T> {
    type Output = Combo<COrdered<T,U>>;
    #[inline]
    fn sub(self, rhs: U) -> Self::Output {
        Combo(COrdered(self.0,rhs))
    }
}

impl<U> BitOr<U> for MatchKeyCode<'_> {
    type Output = Combo<COr<Self,U>>;
    #[inline]
    fn bitor(self, rhs: U) -> Self::Output {
        Combo(COr(self,rhs))
    }
}
impl<U> BitOr<U> for MatchScanCode<'_> {
    type Output = Combo<COr<Self,U>>;
    #[inline]
    fn bitor(self, rhs: U) -> Self::Output {
        Combo(COr(self,rhs))
    }
}
impl<T,U> BitOr<U> for Combo<T> {
    type Output = Combo<COr<T,U>>;
    #[inline]
    fn bitor(self, rhs: U) -> Self::Output {
        Combo(COr(self.0,rhs))
    }
}

impl<U> BitXor<U> for MatchKeyCode<'_> {
    type Output = Combo<CXor<Self,U>>;
    #[inline]
    fn bitxor(self, rhs: U) -> Self::Output {
        Combo(CXor(self,rhs))
    }
}
impl<U> BitXor<U> for MatchScanCode<'_> {
    type Output = Combo<CXor<Self,U>>;
    #[inline]
    fn bitxor(self, rhs: U) -> Self::Output {
        Combo(CXor(self,rhs))
    }
}
impl<T,U> BitXor<U> for Combo<T> {
    type Output = Combo<CXor<T,U>>;
    #[inline]
    fn bitxor(self, rhs: U) -> Self::Output {
        Combo(CXor(self.0,rhs))
    }
}

fn range_range(a: Option<Range<usize>>, b: Option<Range<usize>>) -> Option<Range<usize>> {
    todo!()
}
fn range_overlap(a: Option<Range<usize>>, b: Option<Range<usize>>) -> Option<Range<usize>> {
    todo!()
}
