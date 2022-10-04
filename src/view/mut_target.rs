use std::marker::PhantomData;

use crate::env::Env;

pub trait MuTarget<E>: Send + Sync + 'static {
    type Mutable<'k>: ?Sized + 'k;
}

pub struct MStatic<T>(PhantomData<&'static T>) where T: ?Sized + 'static;

impl<T,E> MuTarget<E> for MStatic<T> where T: ?Sized + Send + Sync + 'static {
    type Mutable<'k> = T;
}

pub struct RootMutTarget<E>(PhantomData<E>) where E: Env;

impl<E> MuTarget<E> for RootMutTarget<E> where E: Env {
    type Mutable<'k> = E::RootMut<'k>;
}

macro_rules! impl_mutarget_static {
    ($($t:ty)*) => {
        $(
            impl<E> MuTarget<E> for $t {
                type Mutable<'k> = Self;
            }
        )*
    };
}

macro_rules! impl_mutarget_transparent1_sized {
    ($(($($t:tt)*))*) => {
        $(
            impl<T,E> MuTarget<E> for $($t)* <T> where T: MuTarget<E>, for<'a> T::Mutable<'a>: Sized {
                type Mutable<'k> = $($t)* <T::Mutable<'k>>;
            }
        )*
    };
}
macro_rules! impl_mutarget_transparent1_unsized {
    ($(($($t:tt)*))*) => {
        $(
            impl<T,E> MuTarget<E> for $($t)* <T> where T: MuTarget<E> + Send + Sync + ?Sized {
                type Mutable<'k> = $($t)* <T::Mutable<'k>>;
            }
        )*
    };
}
macro_rules! impl_mutarget_transparent2_sized {
    ($(($($t:tt)*))*) => {
        $(
            impl<T,U,E> MuTarget<E> for $($t)* <T,U> where T: MuTarget<E>, for<'a> T::Mutable<'a>: Sized, U: MuTarget<E>, for<'a> U::Mutable<'a>: Sized {
                type Mutable<'k> = $($t)* <T::Mutable<'k>,U::Mutable<'k>>;
            }
        )*
    };
}
macro_rules! impl_mutarget_transparent3_sized {
    ($(($($t:tt)*))*) => {
        $(
            impl<T,U,V,E> MuTarget<E> for $($t)* <T,U,V> where T: MuTarget<E>, for<'a> T::Mutable<'a>: Sized, U: MuTarget<E>, for<'a> U::Mutable<'a>: Sized, V: MuTarget<E>, for<'a> V::Mutable<'a>: Sized {
                type Mutable<'k> = $($t)* <T::Mutable<'k>,U::Mutable<'k>>;
            }
        )*
    };
}

impl_mutarget_static!(
    u8 u16 u32 u64 u128 usize 
    i8 i16 i32 i64 i128 isize
    bool char
    String std::path::PathBuf std::path::Path str
    std::ffi::OsStr std::ffi::OsString
    std::collections::hash_map::RandomState std::collections::hash_map::DefaultHasher
    std::net::IpAddr std::net::Ipv4Addr std::net::Ipv6Addr
);

impl<T,E> MuTarget<E> for [T] where T: MuTarget<E>, for<'a> T::Mutable<'a>: Sized {
    type Mutable<'k> = [T::Mutable<'k>];
}
impl<T,E,const N: usize> MuTarget<E> for [T;N] where T: MuTarget<E>, for<'a> T::Mutable<'a>: Sized {
    type Mutable<'k> = [T::Mutable<'k>;N];
}

impl_mutarget_transparent1_sized!(
    (Option)
    (Vec) (std::collections::VecDeque)
    (std::sync::Mutex) (std::sync::RwLock)
);
impl_mutarget_transparent1_unsized!(
    (Box) (std::sync::Arc)
);
impl_mutarget_transparent2_sized!(
    (Result)
    (std::collections::HashSet)
);
impl_mutarget_transparent3_sized!(
    (std::collections::HashMap)
);

macro_rules! impl_tuple {
    {
        $t:ident $($tt:ident)+
    } => {
        impl_tuple!($($tt)+);

        impl<E,$t,$($tt),+> MuTarget<E> for ($t,$($tt),+) where
            $t: MuTarget<E>, for<'a> $t::Mutable<'a>: Sized,
            $($tt: MuTarget<E>, for<'a> $tt::Mutable<'a>: Sized),+
        {
            type Mutable<'k> = (
                $t::Mutable<'k>,
                $($tt::Mutable<'k>),+
            );
        }
    };
    {
        $t:ident
    } => {}
}

impl_tuple!(
    A B C D F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF AG
);
