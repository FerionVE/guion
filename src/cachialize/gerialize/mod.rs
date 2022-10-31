use std::any::TypeId;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::AddAssign;

use self::stor::Stor;
use self::tih::{ser_type_id, type_id_hashed};

pub mod tih;
pub mod imp;
pub mod stor;

const OPTBYTE_LEN: usize = size_of::<u8>();
const USIZE_LEN: usize = size_of::<usize>();
const TYPEID_LEN: usize = size_of::<u64>();

const _: () = assert!(size_of::<u8>() == 1);
const _: () = assert!(size_of::<u32>() == size_of::<char>());
const _: () = assert!(size_of::<u64>() == size_of::<TypeId>());

/// either use `sus_and_typeid` to serialize its type with typeid, or use `sus` and include typeid of `TypeIdMarker`
pub trait Gerialize<S,E> where S: Stor {
    /// returned type id must be identical for all variants to this specific type (normal TypeId behavior)  
    type TypeIdMarker: ?Sized + 'static;

    /// writes only data, type id (`generic_typeid`) must be written manually
    /// 
    /// data len store rule: If serialized data len is dynamic (not static and always the same), some bits value unique to the length (e.g. u64 len at beginning) must be at a static position
    fn gerialize(&self, dest: &mut S) -> Result<(),S::Err>;
    fn len_gerialize(&self) -> usize;

    /// writes typeid first and data, you either call `sus` OR `sus_type_id`
    /// 
    /// other than `generic_typeid`, this can write specialized type id for specific e.g. enum variants (e.g. OptionSome, OptionNone)
    fn gerialize_and_typeid(&self, dest: &mut S) -> Result<(),S::Err> {
        ser_type_id::<Self::TypeIdMarker,S>(dest)?;
        self.gerialize(dest)
    }
    fn len_gerialize_and_typeid(&self) -> usize {
        size_of::<u64>() + self.len_gerialize()
    }
    
    fn verify_gerialize(&self, data: &[u8]) -> (bool,usize);
    fn verify_gerialize_and_typeid(&self, mut data: &[u8]) -> (bool,usize) {
        if !try_match_and_cut_off_left(&mut data,type_id_hashed::<Self::TypeIdMarker>().to_ne_bytes()) {
            return (false,self.len_gerialize_and_typeid());
        }
        let result = self.verify_gerialize(data);
        (result.0,result.1 + size_of::<u64>())
    }
}

pub trait GerializeDyn<S,E> where S: Stor {
    fn dyn_gerialize_and_typeid(&self, dest: &mut S) -> Result<(),S::Err>;
    fn dyn_len_gerialize_and_typeid(&self) -> usize;
    fn dyn_verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize);
}

impl<T,S,E> GerializeDyn<S,E> for T where T: Gerialize<S,E> + ?Sized, S: Stor {
    #[inline]
    fn dyn_gerialize_and_typeid(&self, dest: &mut S) -> Result<(),S::Err> {
        Gerialize::gerialize_and_typeid(self,dest)
    }
    #[inline]
    fn dyn_len_gerialize_and_typeid(&self) -> usize {
        Gerialize::len_gerialize_and_typeid(self)
    }
    #[inline]
    fn dyn_verify_gerialize_and_typeid(&self, data: &[u8]) -> (bool,usize) {
        Gerialize::verify_gerialize_and_typeid(self,data)
    }
}

#[derive(Clone,Default,PartialEq,Eq)]
pub struct Gerialized<E> {
    pub(crate) sused: Vec<u8>,
    _e: PhantomData<E>,
}

impl<E> Gerialized<E> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            sused: Vec::with_capacity(cap),
            _e: PhantomData,
        }
    }

    pub fn verifyer(&self) -> GerializedVerifyer<E> {
        GerializedVerifyer {
            r: &self.sused,
            ok: true,
            new_len: 0,
            _e: PhantomData,
        }
    }

    pub fn _reset(&mut self) {
        self.sused.clear();
    }
}

impl<T,E> AddAssign<T> for Gerialized<E> where T: Gerialize<Vec<u8>,E> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        rhs.gerialize_and_typeid(&mut self.sused).unwrap();
    }
}

#[derive(Clone)]
pub struct GerializedFixed<E> {
    sused: (Box<[u8]>,usize),
    _e: PhantomData<E>,
}

impl<E> GerializedFixed<E> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            sused: (vec![0;cap].into_boxed_slice(),0),
            _e: PhantomData,
        }
    }

    pub fn verifyer(&self) -> GerializedVerifyer<E> {
        GerializedVerifyer {
            r: self.slice(),
            ok: true,
            new_len: 0,
            _e: PhantomData,
        }
    }

    pub fn slice(&self) -> &[u8] {
        &self.sused.0[..self.sused.1]
    }

    pub fn _reset(&mut self) {
        self.sused.1 = 0;
    }
}

impl<T,E> AddAssign<T> for GerializedFixed<E> where T: Gerialize<(Box<[u8]>,usize),E> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        rhs.gerialize_and_typeid(&mut self.sused).unwrap();
    }
}

impl<E,T> PartialEq<T> for Gerialized<E> where T: AsRef<[u8]> {
    fn eq(&self, other: &T) -> bool {
        self.sused == other.as_ref()
    }
}
impl<E,T> PartialEq<T> for GerializedFixed<E> where T: AsRef<[u8]> {
    fn eq(&self, other: &T) -> bool {
        self.slice() == other.as_ref()
    }
}

#[must_use]
pub struct GerializedVerifyer<'a,E> {
    r: &'a [u8],
    ok: bool,
    new_len: usize,
    _e: PhantomData<E>,
}

impl<'a,E> GerializedVerifyer<'a,E> {
    pub fn verify<S,T>(&mut self, data: &T) where T: Gerialize<S,E>, S: Stor {
        if self.ok {
            let (sub_ok,sub_len) = data.verify_gerialize_and_typeid(self.r);
            self.ok &= sub_ok;
            self.ok &= try_cut_off_left(&mut self.r,sub_len);
            self.new_len += sub_len;
        } else {
            self.new_len += data.len_gerialize();
        }
    }

    /// if the base serialized data starts with the data fed into [`SusserVerifyer::verify`]
    pub fn starts_with_data(&self) -> bool {
        self.ok
    }

    #[must_use]
    pub fn done(self) -> (bool,usize) {
        (self.ok && self.r.is_empty(),self.new_len)
    }
}

#[inline]
pub fn try_split_at(s: &[u8], off: usize) -> Option<(&[u8],&[u8])> {
    if s.len() >= off {
        Some(unsafe{(s.get_unchecked(..off),s.get_unchecked(off..))})
    } else {
        None
    }
}

#[inline]
pub fn try_match_at<T>(s: &[u8], v: T) -> Option<(&[u8],&[u8])> where T: AsRef<[u8]> {
    let v = v.as_ref();
    try_split_at(s, v.len()).filter(|(l,_)| **l == *v )
}
#[inline]
pub fn try_cut_off_left(v: &mut &[u8], n: usize) -> bool {
    if v.len() >= n {
        *v = unsafe{(*v).get_unchecked(n..)};
        true
    } else {
        false
    }
}
#[inline]
pub fn try_match_and_cut_off_left<T>(s: &mut &[u8], v: T) -> bool where T: AsRef<[u8]> {
    if let Some((_,r)) = try_match_at(s,v) {
        *s = r;
        true
    } else {
        false
    }
}
