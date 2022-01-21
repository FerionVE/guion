//! Macros for implementing traitcast for widgets
use std::any::TypeId;

pub mod widget;
pub mod handler;

// TODO use ptr_metadata

/// U N S O U N D
#[repr(C)]
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct TraitObject {
    data: *mut (),
    vtable: *mut (),
}

const _: () = assert!(std::mem::size_of::<TraitObject>() == std::mem::size_of::<&dyn std::any::Any>());
const _: () = assert!(std::mem::align_of::<TraitObject>() == std::mem::align_of::<&dyn std::any::Any>());

/// This macro is used inside [`Widget/WidgetMut`](Widget) impls
/// 
/// Example:
/// ```ignore
/// impl_traitcast!(
///     dyn IButton => |s| s;
///     dyn IButtonState => |s| &s.state;
/// );
///
#[macro_export]
macro_rules! impl_traitcast {
    ($srcor:ty: $($trait:ty => |$id:ident| $access:expr; )*) => {
        #[inline]
        unsafe fn _as_trait_ref<'impl_traitcast_lt_a>(&'impl_traitcast_lt_a self, t: std::any::TypeId) -> Option<$crate::traitcast::TraitObject> {
            $(
                if t == std::any::TypeId::of::<<($srcor) as $crate::traitcast::TraitcastImpl::<'_,$trait>>::DestTypeID>() {
                    let $id = self;
                    let s: &'impl_traitcast_lt_a $trait = $access;
                    let s = std::mem::transmute::<&'impl_traitcast_lt_a $trait,$crate::traitcast::TraitObject>(s);
                    return Some(s);
                }
            );*
            None
        }
    }
}

#[doc(hidden)]
pub trait TraitcastImplBase<'a> {
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject>;
}

/// Trait to secure Traitcasting, generally implemented by [macro](traitcast_for)  
/// - Always implemented on `dyn Widget<E>` or `dyn Handler<E>`
/// - `T` is the destination `dyn Trait` to which should be traitcasted
/// - `DestTypeID` must be the same type as `T`, but with 'static lifetimes. Used to retrieve TypeID
#[doc(hidden)]
pub unsafe trait TraitcastImpl<'a,T>: TraitcastImplBase<'a> where T: ?Sized {
    type DestTypeID: ?Sized + 'static;

    #[inline]
    unsafe fn _try_traitcast_ref<'s>(senf: &'s Self) -> Result<&'s T,()> {
        // god plz fix https://github.com/rust-lang/rust/issues/51826
        let t = TypeId::of::<Self::DestTypeID>();
        let t = senf._as_trait_ref(t);
        if let Some(v) = t {
            Ok(std::mem::transmute_copy::<TraitObject,&'s T>(&v))
        } else {
            Err(())
        }
    }
}

/// Syntax:  
/// `traitcast_for!([<...>] Trait[<...>] [where ...]);`  
#[macro_export]
macro_rules! traitcast_for {
    (
        $( < $($args:ident),* $(,)* > )?
        $trait:path
        $(where $($preds:tt)+)?
    ) => {
        unsafe impl<'w,TCO,$( $($args),* )?> $crate::traitcast::TraitcastImpl<'w,dyn $trait+'w> for TCO where TCO: $crate::traitcast::TraitcastImplBase<'w> + ?Sized, $($($preds)+ )? {
            type DestTypeID = dyn $trait+'static;
        }
    }
}
