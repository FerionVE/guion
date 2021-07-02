use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use super::*;

macro_rules! impl_validation_primitives {
    ($t:ty;$($tt:ty);+) => {
        impl_validation_primitives!($t);
        impl_validation_primitives!($($tt);*);
    };
    ($t:ty) => {
        
    }
}

impl_validation_primitives!(
    bool;char;
    f32;f64;
    i8;i16;i32;i64;i128;isize;
    u8;u16;u32;u64;u128;usize;
    String;PathBuf
);

/// NO validation
impl<T,E> Validator<T,E> for () {
    type Cache = ();

    fn valid(value: &T, cache: &Self::Cache) -> bool {
        false
    }

    fn validation(value: &T) -> Self::Cache {
        ()
    }
}

/// Validation by comparing to an owned clone on the type
pub struct MirrorValidated;

impl<T,E> Validator<T,E> for MirrorValidated where T: ToOwned, T::Owned: PartialEq<T> + 'static {
    type Cache = T::Owned;

    fn valid(value: &T, cache: &Self::Cache) -> bool {
        cache == value
    }

    fn validation(value: &T) -> Self::Cache {
        value.to_owned()
    }
}

/// Validation by checking raw pointer address / memory address for constants
///
/// IT SHOULD ONLY BE USED FOR 'static refs/slices and types without interior mutation  
/// IT SHOULD NOT BE USED IN CONNECTION with interior mutation types
pub struct PtrValidated;

impl<T,E> Validator<&'static T,E> for PtrValidated {
    type Cache = usize;

    fn valid(value: &&'static T, cache: &Self::Cache) -> bool {
        <Self as Validator<&'static T,E>>::validation(value) == *cache
    }

    fn validation(value: &&T) -> Self::Cache {
        *value as *const T as usize
    }
}

/// Like [`MirrorValidated`], but with preceeding Arc/Rc::ptr_eq
///
/// IT SHOULD NOT BE USED IN CONNECTION with interior mutation types
pub struct RcMirrorValidated;

impl<T,E> Validator<Rc<T>,E> for RcMirrorValidated where T: PartialEq + 'static {
    type Cache = Rc<T>;

    fn valid(value: &Rc<T>, cache: &Self::Cache) -> bool {
        Rc::ptr_eq(cache, value) || cache == value
    }

    fn validation(value: &Rc<T>) -> Self::Cache {
        Rc::clone(value)
    }
}
impl<T,E> Validator<Arc<T>,E> for RcMirrorValidated where T: PartialEq + 'static  {
    type Cache = Arc<T>;

    fn valid(value: &Arc<T>, cache: &Self::Cache) -> bool {
        Arc::ptr_eq(cache, value)|| cache == value
    }

    fn validation(value: &Arc<T>) -> Self::Cache {
        Arc::clone(value)
    }
}


/// NOTE: HashValidated does NOT handle hash collisions
pub struct HashValidated<H: Hasher + Default>(pub PhantomData<H>);

impl<T,E,H> Validator<T,E> for HashValidated<H> where T: Hash, H: Hasher + Default {
    type Cache = u64;

    fn valid(value: &T, cache: &Self::Cache) -> bool {
        <Self as Validator<T,E>>::validation(value) == *cache
    }

    fn validation(value: &T) -> Self::Cache {
        let mut hasher = H::default();
        value.hash(&mut hasher);
        hasher.finish()
    }
}
