use std::borrow::Cow;
use std::cell::{RefMut, Ref};
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Range, Deref, DerefMut};
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};

use crate::cachor::{MutCell, AsCachor};
use crate::env::Env;
use crate::traitcast_for_from_widget;
use crate::util::immu::Immutable;

use super::layout::TxtLayout;

pub trait TextStor<E> {
    fn caption<'s>(&'s self) -> Cow<'s,str>;
    #[inline]
    fn chars(&self) -> usize {
        self.caption().chars().count()
    }

    #[inline]
    fn len(&self) -> usize {
        self.caption().len()
    }

    #[inline]
    fn immutable(self) -> Immutable<E,Self,()> where Self: Sized {
        Immutable(PhantomData,self)
    }
}

traitcast_for_from_widget!(TextStor<E>); //TODO mutable Traitcast

pub trait TextStorMut<E>: TextStor<E> {
    fn replace(&mut self, replace_range: Range<usize>, insert: &str);

    #[inline]
    fn on_modification<F>(self, f: F) -> OnModification<E,Self,F> where Self: Sized, F: FnMut(&mut Self) {
        OnModification(f,PhantomData,self)
    }
}

impl<E> TextStor<E> for () {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed("")
    }
}
impl<E> TextStor<E> for str {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(self)
    }
}
impl<E> TextStor<E> for String {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Borrowed(self)
    }
}

impl<E> TextStorMut<E> for String {
    fn replace(&mut self, range: Range<usize>, insert: &str) {
        let range = fix_boundary(self, range.start) .. fix_boundary(self, range.end);
        self.drain(range.clone());
        self.insert_str(range.start, insert);
    }
}

impl<E,A> TextStor<E> for &A where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<E,A> TextStor<E> for &mut A where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A> TextStorMut<E> for &mut A where A: TextStorMut<E> + ?Sized {
    #[inline]
    fn replace(&mut self, replace_range: Range<usize>, insert: &str){
        (**self).replace(replace_range,insert)
    }
}

impl<E,A> TextStor<E> for MutexGuard<'_,A> where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A> TextStorMut<E> for MutexGuard<'_,A> where A: TextStorMut<E> + ?Sized {
    #[inline]
    fn replace(&mut self, replace_range: Range<usize>, insert: &str){
        (**self).replace(replace_range,insert)
    }
}

impl<E,A> TextStor<E> for RwLockReadGuard<'_,A> where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<E,A> TextStor<E> for RwLockWriteGuard<'_,A> where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A> TextStorMut<E> for RwLockWriteGuard<'_,A> where A: TextStorMut<E> + ?Sized {
    #[inline]
    fn replace(&mut self, replace_range: Range<usize>, insert: &str){
        (**self).replace(replace_range,insert)
    }
}

impl<E,A> TextStor<E> for Ref<'_,A> where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<E,A> TextStor<E> for RefMut<'_,A> where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A> TextStorMut<E> for RefMut<'_,A> where A: TextStorMut<E> + ?Sized {
    #[inline]
    fn replace(&mut self, replace_range: Range<usize>, insert: &str){
        (**self).replace(replace_range,insert)
    }
}

impl<E,A> TextStor<E> for ManuallyDrop<A> where A: TextStor<E> + ?Sized {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A> TextStorMut<E> for ManuallyDrop<A> where A: TextStorMut<E> + ?Sized {
    #[inline]
    fn replace(&mut self, replace_range: Range<usize>, insert: &str){
        (**self).replace(replace_range,insert)
    }
}

impl<E,T> TextStor<E> for MutCell<T> where T: TextStor<E> {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,T> TextStorMut<E> for MutCell<T> where T: TextStorMut<E> {
    fn replace(&mut self, replace_range: Range<usize>, insert: &str) {
        (**self).replace(replace_range,insert)
    }
}
pub struct OnModification<E,S: ?Sized,F>(F,PhantomData<E>,S) where F: FnMut(&mut S);

impl<E,S,F> Deref for OnModification<E,S,F> where F: FnMut(&mut S) {
    type Target = S;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.2
    }
}
impl<E,S,F> DerefMut for OnModification<E,S,F> where F: FnMut(&mut S) {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.2
    }
}

// impl<E,S,F> Validation<E> for OnModification<E,S,F> where S: Validation<E>, F: FnMut(&mut S) {
//     #[inline]
//     fn valid(&self, v: &dyn Any) -> bool {
//         (**self).valid(v)
//     }
//     #[inline]
//     fn validation(&self) -> Arc<dyn Any> {
//         (**self).validation()
//     }
// }
// impl<E,S,F> ValidationMut<E> for OnModification<E,S,F> where S: ValidationMut<E>, F: FnMut(&mut S) {
//     #[inline]
//     fn validate(&mut self) -> Arc<dyn Any> {
//         (**self).validate()
//         //TODO trigger OnModification?
//     }
// }

impl<E,S,F> TextStor<E> for OnModification<E,S,F> where S: TextStor<E>, F: FnMut(&mut S) {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }

    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,S,F> TextStorMut<E> for OnModification<E,S,F> where S: TextStorMut<E>, F: FnMut(&mut S) {
    #[inline]
    fn replace(&mut self, replace_range: Range<usize>, insert: &str) {
        (**self).replace(replace_range,insert);
        (self.0)(&mut self.2);
    }
}

macro_rules! impl_caption_gen {
    ($t:ty;$($tt:ty);+) => {
        impl_caption_gen!($t);
        impl_caption_gen!($($tt);*);
    };
    ($t:ty) => {
        impl<E> TextStor<E> for $t {
            #[inline]
            fn caption(&self) -> Cow<str> {
                Cow::Owned(self.to_string())
            }
        }
    }
}

impl_caption_gen!(
    bool;char;
    f32;f64;
    i8;i16;i32;i64;i128;isize;
    u8;u16;u32;u64;u128;usize
);

#[inline]
pub fn fix_boundary(s: &str, mut off: usize) -> usize {
    while !s.is_char_boundary(off) && off!=0 {
        off = off.saturating_sub(1); //TODO efficient algorithm
    }
    off
}

pub trait ToTextLayout<S,E>: TextStor<E> where E: Env, S: TxtLayout<E> {
    fn to_text_layout(&self, c: &mut E::Context<'_>) -> S;
    fn update_text_layout(&self, s: &mut S, c: &mut E::Context<'_>);
}

impl<T,S,E> ToTextLayout<S,E> for &T where E: Env, T: ToTextLayout<S,E> + ?Sized, S: TxtLayout<E> {
    fn to_text_layout(&self, c: &mut E::Context<'_>) -> S {
        (**self).to_text_layout(c)
    }

    fn update_text_layout(&self, s: &mut S, c: &mut E::Context<'_>) {
        (**self).update_text_layout(s,c)
    }
}

impl<T,S,E,F> ToTextLayout<S,E> for OnModification<E,T,F> where E: Env, T: ToTextLayout<S,E>, S: TxtLayout<E>, F: FnMut(&mut T) {
    fn to_text_layout(&self, c: &mut E::Context<'_>) -> S {
        (**self).to_text_layout(c)
    }

    fn update_text_layout(&self, s: &mut S, c: &mut E::Context<'_>) {
        (**self).update_text_layout(s,c)
    }
}

impl<T,S,E> ToTextLayout<S,E> for MutCell<T> where E: Env, T: ToTextLayout<S,E>, S: TxtLayout<E> {
    fn to_text_layout(&self, c: &mut E::Context<'_>) -> S {
        (**self).to_text_layout(c)
    }

    fn update_text_layout(&self, s: &mut S, c: &mut E::Context<'_>) {
        (**self).update_text_layout(s,c)
    }
}

impl<T,S,E,Z> ToTextLayout<S,E> for Immutable<E,T,Z> where E: Env, T: ToTextLayout<S,E>, S: TxtLayout<E> {
    fn to_text_layout(&self, c: &mut E::Context<'_>) -> S {
        (**self).to_text_layout(c)
    }

    fn update_text_layout(&self, s: &mut S, c: &mut E::Context<'_>) {
        (**self).update_text_layout(s,c)
    }
}

pub struct TextDisplay<T>(pub T) where T: Display;

impl<T,E> TextStor<E> for TextDisplay<T> where T: Display, E: Env {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        Cow::Owned(format!("{}",self.0))
    }
}

impl<T,E> AsCachor<E> for TextDisplay<T> where T: AsCachor<E> + Display, E: Env {
    type Cachor = T::Cachor;

    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.0.cachor()
    }

    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        self.0.valid(cachored)
    }
}

// macro_rules! macor2 {
//     ( $fmt:expr, $( $($iden:ident =)? $vala:expr ),* ) => {
        
//     };
// }

// fn akw() -> usize {
//     let mut sk = 1;
//     sk = 2
// }

// macor2!("ad",a,4,2);