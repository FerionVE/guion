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

use super::*;

impl<T,E> Validator<T,E> for () {
    type Cache = ();

    fn valid(value: &T, cache: &Self::Cache) -> bool {
        false
    }

    fn validation(value: &T) -> Self::Cache {
        ()
    }
}

pub struct MirrorValidated;

impl<T,E> Validator<T,E> for MirrorValidated where T: ToOwned, T::Owned: PartialEq<T> {
    type Cache = T::Owned;

    fn valid(value: &T, cache: &Self::Cache) -> bool {
        cache == value
    }

    fn validation(value: &T) -> Self::Cache {
        value.to_owned()
    }
}

/// IT SHOULD ONLY USED FOR 'static refs/slices and types without interior mutation
pub struct PtrValidated;

impl<T,E> Validator<&T,E> for PtrValidated {
    type Cache = usize;

    fn valid(value: &&T, cache: &Self::Cache) -> bool {
        Self::validation(value) == cache
    }

    fn validation(value: &&T) -> Self::Cache {
        value as *const T as usize
    }
}
