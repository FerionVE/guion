use std::borrow::Borrow;
use std::cell::{Ref, RefMut};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::{Arc, MutexGuard, RwLockReadGuard, RwLockWriteGuard};

use super::{AsCachorOf, AsCachor};

impl<T,E> AsCachorOf<T,E> for () where T: AsCachor<E> + ?Sized {
    type Cachor = T::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &T) -> Self::Cachor {
        v.cachor()
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &T, cachored: &Self::Cachor) -> bool {
        v.valid(cachored)
    }
}

impl<T,E> AsCachor<E> for Option<T> where T: AsCachor<E> {
    type Cachor = Option<T::Cachor>;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.as_ref().map(T::cachor)
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        match (self,cachored) {
            (Some(s),Some(cachored)) => s.valid(cachored),
            (None,None) => true,
            _ => false,
        }
    }
}

impl<T,E> AsCachor<E> for &T where T: AsCachor<E> + ?Sized {
    type Cachor = T::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        (**self).cachor()
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        (**self).valid(cachored)
    }
}

macro_rules! impl_cachor_via_toowned {
    ($($t:ty)*) => {
        $(
            impl<E> AsCachor<E> for $t {
                type Cachor = <$t as ToOwned>::Owned;
            
                #[must_use]
                #[inline]
                fn cachor(&self) -> Self::Cachor {
                    self.to_owned()
                }
            
                #[must_use]
                #[inline]
                fn valid(&self, cachored: &Self::Cachor) -> bool {
                    self == <<$t as ToOwned>::Owned as Borrow<$t>>::borrow(cachored)
                }
            }
        )*
    };
}

macro_rules! impl_cachor_transparent1 {
    ($(($($t:tt)*))*) => {
        $(
            impl<T,E> AsCachor<E> for $($t)* <T> where T: AsCachor<E> + ?Sized {
                type Cachor = <T as AsCachor<E>>::Cachor;
            
                #[must_use]
                #[inline]
                fn cachor(&self) -> Self::Cachor {
                    <T as AsCachor<E>>::cachor(&**self)
                }
            
                #[must_use]
                #[inline]
                fn valid(&self, cachored: &Self::Cachor) -> bool {
                    <T as AsCachor<E>>::valid(&**self,cachored)
                }
            }
        )*
    };
}

macro_rules! impl_cachor_transparent2 {
    ($(($($t:tt)*))*) => {
        $(
            impl<'a,T,E> AsCachor<E> for $($t)* <'a,T> where T: AsCachor<E> + ?Sized + 'a {
                type Cachor = <T as AsCachor<E>>::Cachor;
            
                #[must_use]
                #[inline]
                fn cachor(&self) -> Self::Cachor {
                    <T as AsCachor<E>>::cachor(&**self)
                }
            
                #[must_use]
                #[inline]
                fn valid(&self, cachored: &Self::Cachor) -> bool {
                    <T as AsCachor<E>>::valid(&**self,cachored)
                }
            }
        )*
    };
}

impl_cachor_via_toowned!(
    u8 u16 u32 u64 u128 usize 
    i8 i16 i32 i64 i128 isize
    bool char
    String std::path::PathBuf std::path::Path str
    std::ffi::OsStr std::ffi::OsString
    std::net::IpAddr std::net::Ipv4Addr std::net::Ipv6Addr
);

impl_cachor_transparent1!(
    (Box) (Rc) (Arc)
    (ManuallyDrop)
);

impl_cachor_transparent2!(
    (MutexGuard) (RwLockReadGuard) (RwLockWriteGuard)
    (Ref) (RefMut)
);
