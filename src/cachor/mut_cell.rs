use std::borrow::{Borrow, BorrowMut};
use std::hash::Hash;
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};

use super::AsCachor;

//TODO prevent use with interior mutable types
#[derive(Clone,Copy,Debug)]
pub struct MutCell<T> where T: ?Sized {
    mut_counter: NonZeroU64,
    inner: T,
}

impl<T> MutCell<T> where T: ?Sized {
    #[inline]
    pub fn new(inner: T) -> Self where T: Sized {
        Self {
            mut_counter: unsafe { NonZeroU64::new_unchecked(1) },
            inner,
        }
    }

    #[inline]
    pub fn into_inner(self) -> T where T: Sized {
        self.inner
    }

    #[inline]
    pub fn invalidate(&mut self) {
        self.mut_counter = self.mut_counter.checked_add(1).unwrap();
    }
}

impl<T> Deref for MutCell<T> where T: ?Sized {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for MutCell<T> where T: ?Sized {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.invalidate();
        &mut self.inner
    }
}

impl<T,U> AsRef<U> for MutCell<T> where T: AsRef<U> + ?Sized, U: ?Sized {
    #[inline]
    fn as_ref(&self) -> &U {
        self.inner.as_ref()
    }
}

impl<T,U> AsMut<U> for MutCell<T> where T: AsMut<U> + ?Sized, U: ?Sized {
    #[inline]
    fn as_mut(&mut self) -> &mut U {
        self.invalidate();
        self.inner.as_mut()
    }
}

impl<T> Borrow<T> for MutCell<T> where T: ?Sized {
    #[inline]
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T> BorrowMut<T> for MutCell<T> where T: ?Sized {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.invalidate();
        &mut self.inner
    }
}

// impl<T,U> PartialEq<U> for MutCell<T> where T: PartialEq<U> + ?Sized {
//     #[inline]
//     fn eq(&self, other: &U) -> bool {
//         self.inner.eq(other)
//     }
// }
impl<T> PartialEq for MutCell<T> where T: PartialEq + ?Sized {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> Eq for MutCell<T> where T: Eq + ?Sized {}

impl<T> PartialOrd for MutCell<T> where T: PartialOrd + ?Sized {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Ord for MutCell<T> where T: Ord + ?Sized {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Hash for MutCell<T> where T: Hash + ?Sized {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> From<T> for MutCell<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Default for MutCell<T> where T: Default {
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T,E> AsCachor<E> for MutCell<T> where T: ?Sized {
    type Cachor = NonZeroU64;

    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.mut_counter
    }
}
