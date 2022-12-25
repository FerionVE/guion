//! Incrementation-based validation and memoization
//!
//! Based on simple incrementing integers which are cheap to compare, can be used as a proxy to track invalidaton
//! 
//! All incremetations that are use interchangebly should be incremented from the same source

use std::num::{NonZeroI64, NonZeroU64};
use std::ops::{BitOrAssign, BitOr};
use std::sync::atomic::{AtomicI64, Ordering};

use super::AsCachor;


#[repr(transparent)]
pub struct IncrementationSource {
    inner: NonZeroU64,
}

impl IncrementationSource {
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: unsafe { NonZeroU64::new_unchecked(256) },
        }
    }

    #[inline]
    pub fn next_increment(&mut self) -> Incrementation {
        let v = self.inner;
        if let Some(new) = NonZeroU64::new(v.get().wrapping_add(1)) {
            self.inner = new;
            Incrementation { inner: v }
        } else {
            panic!("Overflow of incrementation counter")
        }
    }

    #[inline]
    pub fn clone_current_incrementation(&self) -> Self {
        Self {
            inner: self.inner,
        }
    }
}

#[repr(transparent)]
pub struct AtomicIncrementationSource {
    inner: AtomicI64,
}

impl AtomicIncrementationSource {
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: AtomicI64::new(256)
        }
    }

    #[inline]
    pub fn next_increment(&self) -> Incrementation {
        let v = self.inner.fetch_add(1, Ordering::Relaxed);
        if v > 0 {
            Incrementation { inner: unsafe { NonZeroU64::new_unchecked( v as u64 ) } }
        } else {
            panic!("Overflow of incrementation counter")
        }
    }

    #[inline]
    pub fn clone_current_incrementation(&self) -> Self {
        Self {
            inner: AtomicI64::new(self.inner.load(Ordering::Relaxed))
        }
    }
}

/// Incremental (in)validation
#[repr(transparent)]
#[derive(Clone,PartialEq/*,PartialOrd*/)]
pub struct Incrementation {
    inner: NonZeroU64,
}

impl Incrementation {
    #[inline]
    pub const fn smallest() -> Self {
        Self { inner: unsafe { NonZeroU64::new_unchecked(1) } }
    }
}

impl BitOrAssign<&Incrementation> for Incrementation {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Incrementation) {
        self.inner = self.inner.max(rhs.inner);
    }
}

impl BitOr<&Incrementation> for &Incrementation {
    type Output = Incrementation;

    #[inline]
    fn bitor(self, rhs: &Incrementation) -> Self::Output {
        Incrementation { inner: self.inner.max(rhs.inner) }
    }
}

impl PartialEq<&Self> for Incrementation {
    #[inline]
    fn eq(&self, other: &&Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<Incrementation> for &Incrementation {
    #[inline]
    fn eq(&self, other: &Incrementation) -> bool {
        self.inner == other.inner
    }
}

// impl PartialOrd<&Self> for Incrementation {
//     #[inline]
//     fn partial_cmp(&self, other: &&Self) -> Option<std::cmp::Ordering> {
//         self.inner.partial_cmp(&other.inner)
//     }
// }

// impl PartialOrd<Incrementation> for &Incrementation {
//     #[inline]
//     fn partial_cmp(&self, other: &Incrementation) -> Option<std::cmp::Ordering> {
//         self.inner.partial_cmp(&other.inner)
//     }
// }

impl<E> AsCachor<E> for Incrementation {
    type Cachor = Self;

    #[inline]
    fn cachor(&self) -> Self::Cachor {
        Self { inner: self.inner }
    }
}
