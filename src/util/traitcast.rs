//! macros for implementing traitcast for widgets
use std::any::{TypeId, type_name};

use super::*;

/// should match the non-stabilized std::raw::TraitObject and represents an erased fat pointer
#[repr(C)]
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct TraitObject {
    data: *mut (),
    vtable: *mut (),
}

/// This macro is used inside Widget/WidgetMut impls
/// 
/// Example:
/// ```rust
/// impl_traitcast!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &s.state;
/// );
///
#[macro_export]
macro_rules! impl_traitcast {
    ($( $trait:ty => |$id:pat| $access:expr; )*) => {
        #[inline]
        unsafe fn _as_trait_ref<'impl_traitcast_lt_a>(&'impl_traitcast_lt_a self, t: std::any::TypeId) -> Option<$crate::util::traitcast::TraitObject> {
            $(
                if t == std::any::TypeId::of::<<(dyn $crate::widget::Widget<_>) as $crate::util::traitcast::Traitcast::<$trait,_>>::DestTypeID>() {
                    let $id = self;
                    let senf: &'impl_traitcast_lt_a $trait = $access;
                    let senf = std::mem::transmute::<&'impl_traitcast_lt_a $trait,$crate::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

/// This macro is used inside WidgetMut impls
/// 
/// Example:
/// ```rust
/// impl_traitcast_mut!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &mut s.state;
/// );
///
#[macro_export]
macro_rules! impl_traitcast_mut {
    ($( $trait:ty => |$id:pat| $access:expr; )*) => {
        #[inline]
        unsafe fn _as_trait_mut<'impl_traitcast_lt_a>(&'impl_traitcast_lt_a mut self, t: std::any::TypeId) -> Option<$crate::util::traitcast::TraitObject> {
            $(
                if t == std::any::TypeId::of::<<(dyn $crate::widget::WidgetMut<_>) as $crate::util::traitcast::TraitcastMut::<$trait,_>>::DestTypeID>() {
                    let $id = self;
                    let senf: &'impl_traitcast_lt_a mut $trait = $access;
                    let senf = std::mem::transmute::<&'impl_traitcast_lt_a mut $trait,$crate::util::traitcast::TraitObject>(senf);
                    return Some(senf);
                }
            );*
            None
        }
    }
}

macro_rules! impl_statize_lte {
    ($trait:ident) => {
        unsafe impl<'w,E> Statize<E> for dyn $trait<'w,E> where E: Env {
            type Statur = dyn $trait<'static,E>;
        }
        unsafe impl<'l,'s,E> Statize<E> for &'s dyn $trait<'l,E> where E: Env, 'l: 's {
            type Statur = &'static dyn $trait<'static,E>;
        }
        unsafe impl<'l,'s,E> Statize<E> for &'s mut dyn $trait<'l,E> where E: Env, 'l: 's {
            type Statur = &'static mut dyn $trait<'static,E>;
        }
    };
}

impl<E> dyn Widget<E>+'_ where E: Env {
    #[inline]
    pub fn traitcast_ref<'s,T>(&'s self) -> Result<&'s T,GuionError<E>> where Self: Traitcast<T,E>, T: ?Sized {
        unsafe{Self::_traitcast_ref(self.erase())}
    }
}
impl<E> dyn WidgetMut<E>+'_ where E: Env {
    #[inline]
    pub fn traitcast_mut<'s,T>(&'s mut self) -> Result<&'s mut T,GuionError<E>> where Self: TraitcastMut<T,E>, T: ?Sized {
        unsafe{Self::_traitcast_mut(self.erase_mut())}
    }
}

/// trait to secure Traitcasting, generally implemented by macro  
/// - always implemented on `dyn Widget<E>`
/// - `T` is the destination `dyn Trait` to which should be traitcasted
/// - `DestTypeID` must be the same type as `T`, but with 'static lifetimes. Used to retrieve TypeID
pub unsafe trait Traitcast<T,E>: Widget<E> where T: ?Sized, E: Env {
    type DestTypeID: ?Sized + 'static;

    #[inline]
    unsafe fn _traitcast_ref<'s>(senf: &'s dyn Widget<E>) -> Result<&'s T,GuionError<E>> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_ref(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,&'s T>(&v))
        } else if let Some(senf) = senf.inner() {
            Self::_traitcast_ref(senf)
        } else {
            Err(traitcast_error_info::<E,Self::DestTypeID>(senf,"traitcast"))
        }
    }
}

/// trait to secure Traitcasting, generally implemented by macro  
/// - always implemented on `dyn WidgetMut<E>`
/// - `T` is the destination `dyn Trait` to which should be traitcasted
/// - `DestTypeID` must be the same type as `T`, but with 'static lifetimes. Used to retrieve TypeID
pub unsafe trait TraitcastMut<T,E>: WidgetMut<E> where T: ?Sized, E: Env {
    type DestTypeID: ?Sized + 'static;

    #[inline]
    unsafe fn _traitcast_ref<'s>(senf: &'s dyn Widget<E>) -> Result<&'s T,GuionError<E>> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_ref(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,&'s T>(&v))
        } else if let Some(senf) = senf.inner() {
            Self::_traitcast_ref(senf)
        } else {
            Err(traitcast_error_info::<E,Self::DestTypeID>(senf,"traitcast"))
        }
    }
    #[inline]
    fn traitcast_ref<'s>(&'s self) -> Result<&'s T,GuionError<E>> {
        unsafe{Self::_traitcast_ref(self.erase())}
    }
    #[inline]
    unsafe fn _traitcast_mut<'s>(senf: &'s mut dyn WidgetMut<E>) -> Result<&'s mut T,GuionError<E>> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let error_info = traitcast_error_info::<E,Self::DestTypeID>(senf.erase(),"traitcast_mut"); //TODO optimize
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_mut(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,&'s mut T>(&v))
        } else if let Some(senf) = senf.inner_mut() {
            Self::_traitcast_mut(senf)
        } else {
            Err(error_info)
        }
    }
}

#[macro_export]
macro_rules! traitcast_for_immu {
    ($trait:path) => {
        unsafe impl<'w,E> $crate::util::traitcast::Traitcast<dyn $trait+'w,E> for dyn $crate::widget::Widget<E>+'w where E: $crate::env::Env {
            type DestTypeID = dyn $trait+'static;
        }
    }
}
#[macro_export]
macro_rules! traitcast_for_mut {
    ($trait:path) => {
        unsafe impl<'w,E> $crate::util::traitcast::TraitcastMut<dyn $trait+'w,E> for dyn $crate::widget::WidgetMut<E>+'w where E: $crate::env::Env {
            type DestTypeID = dyn $trait+'static;
        }
    }
}

/// Implement Traitcast for traits to be traitcasted from Widget
/// 
/// Syntax: traitcast_for!(trait_path;mut_trait_path);
/// 
/// Implements for: Widget -> Trait, WidgetMut -> Trait, WidgetMut -> TraitMut
/// 
/// Example:
/// traitcast_for!(ICheckBox<E>;ICheckBoxMut<E>);
#[macro_export]
macro_rules! traitcast_for {
    ($trait_immu:path;$trait_mut:path) => {
        $crate::traitcast_for_immu!($trait_immu);
        $crate::traitcast_for_mut!($trait_immu);
        $crate::traitcast_for_mut!($trait_mut);
    };
}

fn traitcast_error_info<E,DestTypeID>(senf: &(dyn Widget<E>+'_), op: &'static str) -> GuionError<E> where E: Env, DestTypeID: ?Sized + 'static {
    GuionError::TraitcastError(Box::new(TraitcastError{
        op,
        src_type: senf.debugged_type_name(),
        dest_trait_type: type_name::<DestTypeID>(),
    }))
}
fn traitcast_error_info_mut<E,DestTypeID>(senf: &mut (dyn WidgetMut<E>+'_), op: &'static str) -> GuionError<E> where E: Env, DestTypeID: ?Sized + 'static {
    GuionError::TraitcastError(Box::new(TraitcastError{
        op,
        src_type: senf.debugged_type_name_mut(),
        dest_trait_type: type_name::<DestTypeID>(),
    }))
}
