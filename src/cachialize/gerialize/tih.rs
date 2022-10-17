use std::any::TypeId;
use std::hash::{Hasher, BuildHasher, Hash};

use super::stor::Stor;

#[cfg(debug_assertions)]
type TypeIdHash = Option<u64>;
#[cfg(not(debug_assertions))]
type TypeIdHash = u64;

#[derive(Default)]
#[repr(transparent)]
pub(crate) struct TypeIdHasher(TypeIdHash);

impl Hasher for TypeIdHasher {
    #[inline]
    fn finish(&self) -> u64 {
        #[cfg(debug_assertions)] {
            self.0.unwrap()
        }
        #[cfg(not(debug_assertions))] {
            self.0
        }
    }

    fn write(&mut self, _: &[u8]) {
        panic!()
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        #[cfg(debug_assertions)] {
            assert!( self.0.is_none() );
            self.0 = Some(i);
        }
        #[cfg(not(debug_assertions))] {
            self.0 = i;
        }
    }
}

impl BuildHasher for TypeIdHasher {
    type Hasher = Self;
    
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

#[inline]
pub fn type_id_hashed<T: ?Sized + 'static>() -> u64 {
    let mut th = TypeIdHasher::default();
    TypeId::of::<T>().hash(&mut th);
    th.finish()
}

#[inline]
pub fn ser_type_id<T: ?Sized + 'static,S: Stor>(d: &mut S) -> Result<(),S::Err> {
    d.write(type_id_hashed::<T>().to_ne_bytes())
}
