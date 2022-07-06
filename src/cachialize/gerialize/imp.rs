use std::any::TypeId;
use std::borrow::Cow;
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::marker::{PhantomData, PhantomPinned};
use std::mem::size_of;
use std::rc::Rc;
use std::sync::Arc;

use super::{Gerialize, GerializeDyn, try_match_at, try_cut_off_left, try_match_and_cut_off_left, Stor};
use super::tih::TypeIdHasher;

macro_rules! impl_gerialize_raw {
    ($($t:ty);*) => {$(
        impl<GerializeStor,E> Gerialize<GerializeStor,E> for $t where GerializeStor: Stor {
            type TypeIdMarker = Self;

            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                dest.write(self.to_ne_bytes())
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                size_of::<Self>()
            }
            #[inline]
            fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
                ($crate::cachialize::gerialize::try_match_at(data,self.to_ne_bytes()).is_some(),size_of::<Self>())
            }
        }
    )*};
}

#[macro_export]
macro_rules! impl_gerialize_void {
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty : $markertyp:ty
        $(where $($preds:tt)+)?
    ) => {
        impl<GerializeStor,$($args)*> $crate::cachialize::gerialize::Gerialize<GerializeStor,$e> for $typ
            where GerializeStor: $crate::cachialize::gerialize::stor::Stor $(, $($preds)*)?
        {
            type TypeIdMarker = $markertyp;
        
            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                dest.write([])
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                0
            }
            #[inline]
            fn verify_gerialize(&self, _: &[u8]) -> (bool,usize) {
                (true,0)
            }
            #[inline]
            fn gerialize_and_typeid(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                dest.write([])
            }
            #[inline]
            fn len_gerialize_and_typeid(&self) -> usize {
                0
            }
            #[inline]
            fn verify_gerialize_and_typeid(&self, _: &[u8]) -> (bool,usize) {
                (true,0)
            }
        }
    }
}

#[macro_export]
macro_rules! impl_gerialize_transparent { // next-gen matching
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty => $subtyp:ty
        $(where ($($preds:tt)+))?
        |$senf:ident| $tosub:expr
    ) => {
        $crate::impl_gerialize_transparent!(
            $e;
            ($($args)*)
            $typ : Self => $subtyp
            $(where ($($preds)+))?
            |$senf| $tosub
        );
    };
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty : $markertyp:ty => $subtyp:ty
        $(where ($($preds:tt)+))?
        |$senf:ident| $tosub:expr
    ) => {
        impl<GerializeStor,$($args)*> $crate::cachialize::gerialize::Gerialize<GerializeStor,$e> for $typ
            where GerializeStor: $crate::cachialize::gerialize::stor::Stor $(, $($preds)*)?
        {
            type TypeIdMarker = <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::TypeIdMarker;
        
            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let $senf = self;
                <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::gerialize($tosub,dest)
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                let $senf = self;
                <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::len_gerialize($tosub)
            }
            #[inline]
            fn gerialize_and_typeid(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let $senf = self;
                <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::gerialize_and_typeid($tosub,dest)
            }
            #[inline]
            fn len_gerialize_and_typeid(&self) -> usize {
                let $senf = self;
                <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::len_gerialize_and_typeid($tosub)
            }
            #[inline]
            fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
                let $senf = self;
                <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::verify_gerialize($tosub,data)
            }
            #[inline]
            fn verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize) {
                let $senf = self;
                <$subtyp as $crate::cachialize::gerialize::Gerialize<GerializeStor,$e>>::verify_gerialize_and_typeid($tosub,data)
            }
        }
    }
}

#[macro_export]
macro_rules! gerialize_struct {
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty
        $(where ($($preds:tt)+))?
        |$senf:ident|
        $($acc:expr);*
    ) => {
        impl<GerializeStor,$($args)*> $crate::cachialize::gerialize::Gerialize<GerializeStor,$e> for $typ
            where GerializeStor: $crate::cachialize::gerialize::stor::Stor $(, $($preds)*)?
        {
            type TypeIdMarker = Self;

            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let $senf = self;

                $({
                    let v = $acc;
                    $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::gerialize(&v,dest)?;
                })*

                Ok(())
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                let $senf = self;

                let mut len = 0;

                $({
                    let v = $acc;
                    len += $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v);
                })*

                len
            }
            #[inline]
            fn verify_gerialize(&self, mut data: &[u8]) -> (bool,usize) {
                let $senf = self;

                let mut ok = true;
                let mut len = 0;

                $({
                    let v = $acc;
                    if ok {
                        let (sub_ok,sub_len) = $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::verify_gerialize(&v,&data);
                        ok &= sub_ok;
                        ok &= $crate::cachialize::gerialize::try_cut_off_left(&mut data,sub_len);
                        len += sub_len;
                    } else {
                        len += $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v);
                    }
                })*

                (ok,len)
            }
        }
    };
}

#[macro_export]
macro_rules! gerialize_enum_base {
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty
        $(where ($($preds:tt)+))?
        |$senf:ident|
        $(
            $numbah:literal : $accp:pat => $acce:expr
        );*
    ) => {
        impl<GerializeStor,$($args)*> $crate::cachialize::gerialize::Gerialize<GerializeStor,$e> for $typ
            where GerializeStor: $crate::cachialize::gerialize::stor::Stor $(, $($preds)*)?
        {
            type TypeIdMarker = Self;

            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let $senf = self;

                match $senf {
                    $(
                        $accp => {
                            let v = $acce;
                            dest.write([$numbah])?;
                            $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::gerialize(&v,dest)?;
                        }
                    ),*
                }

                Ok(())
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                let $senf = self;

                let len =
                match $senf {
                    $(
                        $accp => {
                            let v = $acce;
                            $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v)
                        }
                    ),*
                };

                len + $crate::cachialize::gerialize::OPTBYTE_LEN
            }
            #[inline]
            fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
                let $senf = self;

                match $senf {
                    $(
                        $accp => {
                            let v = $acce;

                            if let Some((_,r)) = $crate::cachialize::gerialize::try_match_at(data,[$numbah as u8]) {
                                let (ok,len) = $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::verify_gerialize(&v,r);
                                (ok,$crate::cachialize::gerialize::OPTBYTE_LEN + len)
                            } else {
                                (false,$crate::cachialize::gerialize::OPTBYTE_LEN + $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v))
                            }
                        }
                    ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gerialize_enum_adv {
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty : $markertyp:ty
        $(where ($($preds:tt)+))?
        |$senf:ident|
        $(
            $accty:ty : $numbah:literal : $accp:pat => $acce:expr
        );*
    ) => {
        impl<GerializeStor,$($args)*> $crate::cachialize::gerialize::Gerialize<GerializeStor,$e> for $typ
            where GerializeStor: $crate::cachialize::gerialize::stor::Stor $(, $($preds)*)?
        {
            type TypeIdMarker = $markertyp;

            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let $senf = self;

                match $senf {
                    $(
                        $accp => {
                            let v = $acce;
                            dest.write([$numbah])?;
                            $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::gerialize(&v,dest)?;
                        }
                    ),*
                }

                Ok(())
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                let $senf = self;

                let len =
                match $senf {
                    $(
                        $accp => {
                            let v = $acce;
                            $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v)
                        }
                    ),*
                };
                
                len + $crate::cachialize::gerialize::OPTBYTE_LEN
            }
            #[inline]
            fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
                let $senf = self;

                match $senf {
                    $(
                        $accp => {
                            let v = $acce;

                            if let Some((_,r)) = $crate::cachialize::gerialize::try_match_at(data,[$numbah as u8]) {
                                let (ok,len) = $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::verify_gerialize(&v,r);
                                (ok,$crate::cachialize::gerialize::OPTBYTE_LEN + len)
                            } else {
                                (false,$crate::cachialize::gerialize::OPTBYTE_LEN + $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v))
                            }
                        }
                    ),*
                }
            }
            #[inline]
            fn gerialize_and_typeid(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let $senf = self;

                match $senf {
                    $(
                        $accp => {
                            let v = $acce;
                            $crate::cachialize::gerialize::tih::ser_type_id::<$accty,GerializeStor>(dest)?;
                            $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::gerialize(&v,dest)?;
                        }
                    ),*
                }

                Ok(())
            }
            #[inline]
            fn len_gerialize_and_typeid(&self) -> usize {
                let $senf = self;

                let len =
                match $senf {
                    $(
                        $accp => {
                            let v = $acce;
                            $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v)
                        }
                    ),*
                };
                
                len + $crate::cachialize::gerialize::TYPEID_LEN
            }
            #[inline]
            fn verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize) {
                let $senf = self;

                match $senf {
                    $(
                        $accp => {
                            let v = $acce;

                            if let Some((_,r)) = $crate::cachialize::gerialize::try_match_at(data,$crate::cachialize::gerialize::tih::type_id_hashed::<$accty>().to_ne_bytes()) {
                                let (ok,len) = $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::verify_gerialize(&v,r);
                                (ok,$crate::cachialize::gerialize::TYPEID_LEN + len)
                            } else {
                                (false,$crate::cachialize::gerialize::TYPEID_LEN + $crate::cachialize::gerialize::Gerialize::<GerializeStor,$e>::len_gerialize(&v))
                            }
                        }
                    ),*
                }
            }
        }
    };
}


#[macro_export]
macro_rules! gerialize_enum_adv2 {
    (
        $e:ty;
        ($($args:tt)*)
        $typ:ty : $markertyp:ty
        $(where ($($preds:tt)+))?
        |$senf:ident|
        $(
            $accty:ident $(<$($acctyargs:ident),*>)? $(where $($acctypreds:tt)+)?
            : $numbah:literal : $accp:pat => $acce:expr
        );*
    ) => {
        $(
            #[doc(hidden)]
            pub struct $accty $(<$($acctyargs),*>)? $( $($acctyargs),* )? $(where $($acctypreds)*)?;
        )*

        $crate::gerialize_enum_adv!(
            $e;
            ($($args)*)
            $typ : $markertyp
            $(where ($($preds)+))?
            |$senf|
            $(
                $accty : $numbah : $accp => $acce
            );*
        );
    };
}

impl_gerialize_raw!(
    u8;u16;u32;u64;u128;usize;
    i8;i16;i32;i64;i128;isize
);

impl<GerializeStor,E> Gerialize<GerializeStor,E> for () where GerializeStor: Stor {
    type TypeIdMarker = Self;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        dest.write([])
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        0
    }
    #[inline]
    fn verify_gerialize(&self, _: &[u8]) -> (bool,usize) {
        (true,0)
    }
}

impl_gerialize_void!(E;(T,E) PhantomData<T> : PhantomData<()> where T: ?Sized);
impl_gerialize_void!(E;(E) PhantomPinned : PhantomPinned);

impl<GerializeStor,E> Gerialize<GerializeStor,E> for bool where GerializeStor: Stor {
    type TypeIdMarker = Self;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        dest.write([*self as u8])
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        size_of::<u8>()
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        (try_match_at(data,[*self as u8]).is_some(),size_of::<u8>())
    }
}

impl<GerializeStor,E> Gerialize<GerializeStor,E> for char where GerializeStor: Stor {
    type TypeIdMarker = Self;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        Gerialize::<GerializeStor,E>::gerialize(&unsafe{std::mem::transmute::<char,u32>(*self)},dest)
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        size_of::<char>()
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        Gerialize::<GerializeStor,E>::verify_gerialize(&unsafe{std::mem::transmute::<char,u32>(*self)},data)
    }
}

impl<GerializeStor,E> Gerialize<GerializeStor,E> for str where GerializeStor: Stor {
    type TypeIdMarker = Self;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        Gerialize::<GerializeStor,E>::gerialize(&self.len(),dest)?;
        dest.write(self)
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        size_of::<usize>() + self.len()
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        if let Some((_,r)) = try_match_at(data,self.len().to_ne_bytes()) {
            (try_match_at(r,self).is_some(),Gerialize::<GerializeStor,E>::len_gerialize(self))
        } else {
            (false,Gerialize::<GerializeStor,E>::len_gerialize(self))
        }
    }
}

impl_gerialize_transparent!(E;(E) String => str |s| &**s);

impl_gerialize_transparent!(E;(T,E) &T => T where (T: Gerialize<GerializeStor,E> + ?Sized) |s| &**s);
impl_gerialize_transparent!(E;(T,E) &mut T => T where (T: Gerialize<GerializeStor,E> + ?Sized) |s| &**s);
impl_gerialize_transparent!(E;(T,E) Box<T> => T where (T: Gerialize<GerializeStor,E> + ?Sized) |s| &**s);
impl_gerialize_transparent!(E;(T,E) Rc<T> => T where (T: Gerialize<GerializeStor,E> + ?Sized) |s| &**s);
impl_gerialize_transparent!(E;(T,E) Arc<T> => T where (T: Gerialize<GerializeStor,E> + ?Sized) |s| &**s);
impl_gerialize_transparent!(E;(T,E) Cow<'_,T> => T where (T: Gerialize<GerializeStor,E> + Clone + ?Sized) |s| &**s);

#[doc(hidden)]
pub struct GerializedSlice<T>(T) where T: ?Sized + 'static;

impl<T,GerializeStor,E> Gerialize<GerializeStor,E> for [T] where T: Gerialize<GerializeStor,E> + Sized, GerializeStor: Stor {
    type TypeIdMarker = GerializedSlice<T::TypeIdMarker>;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        Gerialize::<GerializeStor,E>::gerialize(&self.len(),dest)?;
        for v in self {
            v.gerialize(dest)?;
        }
        Ok(())
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        let mut len = size_of::<usize>();
        for v in self {
            len += v.len_gerialize();
        }
        len
    }
    #[inline]
    fn verify_gerialize(&self, mut data: &[u8]) -> (bool,usize) {
        if !try_match_and_cut_off_left(&mut data,self.len().to_ne_bytes()) {
            return (false,Gerialize::<GerializeStor,E>::len_gerialize(self));
        }

        let mut ok = true;
        let mut len = size_of::<usize>();

        for v in self {
            if ok {
                let (sub_ok,sub_len) = v.verify_gerialize(data);
                ok &= sub_ok;
                ok &= try_cut_off_left(&mut data,sub_len);
                len += sub_len;
            } else {
                len += v.len_gerialize();
            }
        }

        (ok,len)
    }
}

impl_gerialize_transparent!(E;(T,E) Vec<T> => [T] where (T: Gerialize<GerializeStor,E> + Sized) |s| &**s);
impl_gerialize_transparent!(E;(T,E,const N: usize) [T;N] => [T] where (T: Gerialize<GerializeStor,E> + Sized) |s| s as &[T]);

#[doc(hidden)]
pub struct GerializedOption<T>(T) where T: ?Sized + 'static;
#[doc(hidden)]
pub struct GerializedOptionSome<T>(T) where T: ?Sized + 'static;
#[doc(hidden)]
pub struct GerializedOptionNone<T>(T) where T: ?Sized + 'static;

gerialize_enum_adv!(
    E; (T,E) Option<T> : GerializedOption<T::TypeIdMarker> where (T: Gerialize<GerializeStor,E>) |s|
    GerializedOptionNone<T::TypeIdMarker>:0: None => ();
    GerializedOptionSome<T::TypeIdMarker>:1: Some(v) => v
);

#[doc(hidden)]
pub struct GerializedResult<T,U>(&'static T,&'static U) where T: ?Sized + 'static, U: ?Sized + 'static;
#[doc(hidden)]
pub struct GerializedResultOk<T,U>(&'static T,&'static U) where T: ?Sized + 'static, U: ?Sized + 'static;
#[doc(hidden)]
pub struct GerializedResultErr<T,U>(&'static T,&'static U) where T: ?Sized + 'static, U: ?Sized + 'static;

gerialize_enum_adv!(
    E; (T,U,E) Result<T,U> : GerializedResult<T::TypeIdMarker,U::TypeIdMarker> where (T: Gerialize<GerializeStor,E>, U: Gerialize<GerializeStor,E>) |s|
    GerializedResultErr<T::TypeIdMarker,U::TypeIdMarker>:0: Err(v) => v;
    GerializedResultOk<T::TypeIdMarker,U::TypeIdMarker>:1: Ok(v) => v
);

impl<GerializeStor,E> Gerialize<GerializeStor,E> for dyn GerializeDyn<GerializeStor,E> + '_ where E: 'static, GerializeStor: Stor {
    type TypeIdMarker = dyn GerializeDyn<Vec<u8>,E>; //TODO not VecU8

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        GerializeDyn::dyn_gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        GerializeDyn::dyn_len_gerialize_and_typeid(self)
    }
    #[inline]
    fn gerialize_and_typeid(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        GerializeDyn::dyn_gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn len_gerialize_and_typeid(&self) -> usize {
        GerializeDyn::dyn_len_gerialize_and_typeid(self)
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        GerializeDyn::dyn_verify_gerialize_and_typeid(self,data)
    }
    #[inline]
    fn verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize) {
        GerializeDyn::dyn_verify_gerialize_and_typeid(self,data)
    }
}

impl<GerializeStor,E> Gerialize<GerializeStor,E> for dyn GerializeDyn<GerializeStor,E> + Send + '_ where E: 'static, GerializeStor: Stor {
    type TypeIdMarker = dyn GerializeDyn<Vec<u8>,E>;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        GerializeDyn::dyn_gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        GerializeDyn::dyn_len_gerialize_and_typeid(self)
    }
    #[inline]
    fn gerialize_and_typeid(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        GerializeDyn::dyn_gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn len_gerialize_and_typeid(&self) -> usize {
        GerializeDyn::dyn_len_gerialize_and_typeid(self)
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        GerializeDyn::dyn_verify_gerialize_and_typeid(self,data)
    }
    #[inline]
    fn verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize) {
        GerializeDyn::dyn_verify_gerialize_and_typeid(self,data)
    }
}

impl<GerializeStor,E> Gerialize<GerializeStor,E> for dyn GerializeDyn<GerializeStor,E> + Send + Sync + '_ where E: 'static, GerializeStor: Stor {
    type TypeIdMarker = dyn GerializeDyn<Vec<u8>,E>;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        GerializeDyn::dyn_gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        GerializeDyn::dyn_len_gerialize_and_typeid(self)
    }
    #[inline]
    fn gerialize_and_typeid(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        GerializeDyn::dyn_gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn len_gerialize_and_typeid(&self) -> usize {
        GerializeDyn::dyn_len_gerialize_and_typeid(self)
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        GerializeDyn::dyn_verify_gerialize_and_typeid(self,data)
    }
    #[inline]
    fn verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize) {
        GerializeDyn::dyn_verify_gerialize_and_typeid(self,data)
    }
}

macro_rules! impl_tuple {
    {
        $t:ident $($tt:ident)+;
        $l:ident $($ll:ident)+;
    } => {
        impl_tuple!($($tt)+;$($ll)+;);

        impl<GerializeStor,E,$t,$($tt),+> $crate::cachialize::gerialize::Gerialize<GerializeStor,E> for ($t,$($tt),+) where
            GerializeStor: $crate::cachialize::gerialize::stor::Stor,
            $t: $crate::cachialize::gerialize::Gerialize<GerializeStor,E>,
            $($tt: $crate::cachialize::gerialize::Gerialize<GerializeStor,E>),+ 
        {
            type TypeIdMarker = (&'static <$t as $crate::cachialize::gerialize::Gerialize<GerializeStor,E>>::TypeIdMarker,$(&'static <$tt as $crate::cachialize::gerialize::Gerialize<GerializeStor,E>>::TypeIdMarker),+) ;
            
            #[inline]
            fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
                let ($l,$($ll),*) = self;

                $crate::cachialize::gerialize::Gerialize::gerialize($l,dest)?;
                $(
                    $crate::cachialize::gerialize::Gerialize::gerialize($ll,dest)?;
                )*
                Ok(())
            }
            #[inline]
            fn len_gerialize(&self) -> usize {
                let ($l,$($ll),*) = self;

                let mut len = 0;

                len += $crate::cachialize::gerialize::Gerialize::len_gerialize($l);
                $(
                    len += $crate::cachialize::gerialize::Gerialize::len_gerialize($ll);
                )*

                len
            }
            #[inline]
            fn verify_gerialize(&self, mut data: &[u8]) -> (bool,usize) {
                let ($l,$($ll),*) = self;

                let mut ok = true;
                let mut len = 0;

                {
                    let (sub_ok,sub_len) = $crate::cachialize::gerialize::Gerialize::verify_gerialize($l,data);
                    ok &= sub_ok;
                    ok &= $crate::cachialize::gerialize::try_cut_off_left(&mut data,sub_len);
                    len += sub_len;
                }

                $(
                    if ok {
                        let (sub_ok,sub_len) = $crate::cachialize::gerialize::Gerialize::verify_gerialize($ll,data);
                        ok &= sub_ok;
                        ok &= $crate::cachialize::gerialize::try_cut_off_left(&mut data,sub_len);
                        len += sub_len;
                    } else {
                        len += $crate::cachialize::gerialize::Gerialize::len_gerialize($ll);
                    }
                )*

                (ok,len)
            }
        }
    };
    {
        $t:ident;$l:ident;
    } => {}
}

impl_tuple!(
    A B C D F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF AG;
    a b c d f g h i j k l m n o p q r s t u v w x y z aa ab ac ad ae af ag;
);

impl<GerializeStor,E> Gerialize<GerializeStor,E> for TypeId where GerializeStor: Stor {
    type TypeIdMarker = Self;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        let mut th = TypeIdHasher::default();
        self.hash(&mut th);
        dest.write(th.finish().to_ne_bytes())
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        size_of::<u64>()
    }
    #[inline]
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize) {
        let mut th = TypeIdHasher::default();
        self.hash(&mut th);
        (try_match_at(data,th.finish().to_ne_bytes()).is_some(),size_of::<u64>())
    }
}

#[doc(hidden)]
pub struct GerializedVecDeque<T>(T) where T: ?Sized + 'static;

impl<T,GerializeStor,E> Gerialize<GerializeStor,E> for VecDeque<T> where T: Gerialize<GerializeStor,E> + Sized, GerializeStor: Stor {
    type TypeIdMarker = GerializedVecDeque<T::TypeIdMarker>;

    #[inline]
    fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
        Gerialize::<GerializeStor,E>::gerialize(&self.len(),dest)?;
        for v in self {
            v.gerialize(dest)?;
        }
        Ok(())
    }
    #[inline]
    fn len_gerialize(&self) -> usize {
        let mut len = size_of::<usize>();
        for v in self {
            len += v.len_gerialize();
        }
        len
    }
    #[inline]
    fn verify_gerialize(&self, mut data: &[u8]) -> (bool,usize) {
        if !try_match_and_cut_off_left(&mut data,self.len().to_ne_bytes()) {
            return (false,Gerialize::<GerializeStor,E>::len_gerialize(self));
        }

        let mut ok = true;
        let mut len = size_of::<usize>();

        for v in self {
            if ok {
                let (sub_ok,sub_len) = v.verify_gerialize(data);
                ok &= sub_ok;
                ok &= try_cut_off_left(&mut data,sub_len);
                len += sub_len;
            } else {
                len += v.len_gerialize();
            }
        }

        (ok,len)
    }
}

// #[doc(hidden)]
// pub struct GerializedMap<K,V>(&'static K,&'static V) where K: ?Sized + 'static, V: ?Sized + 'static;

// impl<K,V,S,GerializeStor,E> Gerialize<GerializeStor,E> for HashMap<K,V,S> where K: Gerialize<GerializeStor,E> + Sized + Ord, V: Gerialize<GerializeStor,E> + Sized, GerializeStor: Stor {
//     type TypeIdMarker = GerializedMap<K::TypeIdMarker,V::TypeIdMarker>;

//     fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
//         Gerialize::<GerializeStor,E>::gerialize(&self.len(),dest)?;

//         let mut ultra = self.iter()
//             .collect::<Vec<_>>();

//         ultra.sort_by_key(#[inline] |(k,_)| *k ); //TODO perf dipshit

//         for (k,v) in ultra {
//             k.gerialize(dest)?;
//             v.gerialize(dest)?;
//         }

//         Ok(())
//     }

//     fn len_gerialize(&self) -> usize {
//         let mut len = size_of::<usize>();
//         for (k,v) in self {
//             len += k.len_gerialize() + v.len_gerialize();
//         }
//         len
//     }

//     fn verify_gerialize(&self, mut data: &[u8]) -> (bool,usize) {
//         if !try_match_and_cut_off_left(&mut data,self.len().to_ne_bytes()) {
//             return (false,Gerialize::<GerializeStor,E>::len_gerialize(self));
//         }

//         let mut ultra = self.iter()
//             .collect::<Vec<_>>();

//         ultra.sort_by_key(#[inline] |(k,_)| *k ); //TODO perf dipshit

//         let mut ok = true;
//         let mut len = size_of::<usize>();

//         for (k,v) in self {
//             if ok {
//                 let (sub_ok,sub_len) = k.verify_gerialize(data);
//                 ok &= sub_ok;
//                 ok &= try_cut_off_left(&mut data,sub_len);
//                 len += sub_len;
//             } else {
//                 len += k.len_gerialize();
//             }
//             if ok {
//                 let (sub_ok,sub_len) = v.verify_gerialize(data);
//                 ok &= sub_ok;
//                 ok &= try_cut_off_left(&mut data,sub_len);
//                 len += sub_len;
//             } else {
//                 len += v.len_gerialize();
//             }
//         }

//         (ok,len)
//     }
// }

// #[doc(hidden)]
// pub struct GerializedSet<K>(K) where K: ?Sized + 'static;

// impl<K,S,GerializeStor,E> Gerialize<GerializeStor,E> for HashSet<K,S> where K: Gerialize<GerializeStor,E> + Sized + Ord, GerializeStor: Stor {
//     type TypeIdMarker = GerializedSet<K::TypeIdMarker>;

//     fn gerialize(&self, dest: &mut GerializeStor) -> Result<(),GerializeStor::Err> {
//         Gerialize::<GerializeStor,E>::gerialize(&self.len(),dest)?;

//         let mut ultra = self.iter()
//             .collect::<Vec<_>>();

//         ultra.sort(); //TODO perf dipshit

//         for k in ultra {
//             k.gerialize(dest)?;
//         }

//         Ok(())
//     }

//     fn len_gerialize(&self) -> usize {
//         let mut len = size_of::<usize>();
//         for k in self {
//             len += k.len_gerialize();
//         }
//         len
//     }

//     fn verify_gerialize(&self, mut data: &[u8]) -> (bool,usize) {
//         if !try_match_and_cut_off_left(&mut data,self.len().to_ne_bytes()) {
//             return (false,Gerialize::<GerializeStor,E>::len_gerialize(self));
//         }

//         let mut ultra = self.iter()
//             .collect::<Vec<_>>();

//         ultra.sort(); //TODO perf dipshit

//         let mut ok = true;
//         let mut len = size_of::<usize>();

//         for k in self {
//             if ok {
//                 let (sub_ok,sub_len) = k.verify_gerialize(data);
//                 ok &= sub_ok;
//                 ok &= try_cut_off_left(&mut data,sub_len);
//                 len += sub_len;
//             } else {
//                 len += k.len_gerialize();
//             }
//         }

//         (ok,len)
//     }
// }

//TODO Ordering
