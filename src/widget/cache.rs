use std::marker::PhantomData;
use std::ops::{BitOrAssign, BitAndAssign};

use crate::cachialize::cache::Cache;

use super::*;

pub trait WidgetCache<E>: Default + Sized + 'static {
    /// Reset current of current and child caches
    fn reset_current(&mut self);

    // #[inline]
    // fn full_reset(&mut self) {
    //     *self = Default::default();
    // }
    // #[inline]
    // fn with_resetted(&mut self) -> &mut Self {
    //     self.full_reset();
    //     self
    // }
    // #[inline]
    // fn with_force(&mut self, force: bool) -> &mut Self {
    //     if force {
    //         self.full_reset();
    //     }
    //     self
    // }
}

pub trait WidgetCacheDyn<E>: 'static {
    fn reset_current(&mut self);

    // fn full_reset(&mut self);

    // fn _clone_to_box(&self) -> Box<dyn WidgetCacheDyn<E>>;
}

impl<T,E> WidgetCacheDyn<E> for T where T: WidgetCache<E> {
    #[inline]
    fn reset_current(&mut self) {
        WidgetCache::reset_current(self)
    }
    // #[inline]
    // fn full_reset(&mut self) {
    //     WidgetCache::full_reset(self)
    // }
    // #[inline]
    // fn _clone_to_box(&self) -> Box<dyn WidgetCacheDyn<E>> {
    //     Box::new(self.clone())
    // }
}

pub struct DynWidgetCache<E> {
    inner_type_id: TypeId,
    inner: Box<dyn WidgetCacheDyn<E>>,
}

impl<E> DynWidgetCache<E> where E: 'static {
    #[inline]
    pub fn from_cache<T>(inner: T) -> Self where T: WidgetCache<E> {
        if TypeId::of::<Self>() == TypeId::of::<T>() {
            // avoid nesting
            let senf: Self = unsafe{ std::mem::transmute_copy(&inner) };
            std::mem::forget(inner);
            return senf;
        }
        Self {
            inner_type_id: TypeId::of::<T>(),
            inner: Box::new(inner),
        }
    }

    #[inline]
    pub fn downcast_mut_or_reset<T>(&mut self) -> &mut T where T: WidgetCache<E> {
        if TypeId::of::<Self>() == TypeId::of::<T>() {
            // return self in nested downcast
            return unsafe { &mut *(self as *mut Self as *mut T) }
        }
        if self.inner_type_id != TypeId::of::<T>() {
            self.inner = Box::new(T::default());
            self.inner_type_id = TypeId::of::<T>();
        }
        let inner = &mut *self.inner;
        unsafe { &mut *(inner as *mut dyn WidgetCacheDyn<E> as *mut T) }
    }
}

impl<E> Default for DynWidgetCache<E> where E: 'static {
    #[inline]
    fn default() -> Self {
        Self::from_cache(CacheDefault)
    }
}

// impl<E> Clone for DynWidgetCache<E> where E: 'static {
//     #[inline]
//     fn clone(&self) -> Self {
//         Self {
//             inner_type_id: self.inner_type_id,
//             inner: self.inner._clone_to_box(),
//         }
//     }
// }

impl<E> WidgetCache<E> for DynWidgetCache<E> where E: 'static {
    #[inline]
    fn reset_current(&mut self) {
        self.inner.reset_current()
    }
    // #[inline]
    // fn full_reset(&mut self) {
    //     self.inner.full_reset(); // reset the boxed type to avoid reallocation (unless the to-downcast type changes)
    // }
}

#[derive(Clone,Default)]
struct CacheDefault;

impl<E> WidgetCache<E> for CacheDefault {
    #[inline]
    fn reset_current(&mut self) {}
}

impl<E> WidgetCache<E> for () {
    #[inline]
    fn reset_current(&mut self) {}
}

#[derive(PartialEq,Clone)]
pub struct StdRenderCachors<E> where E: Env {
    pub dims: Dims,
    pub fg_color: ESColor<E>,
    pub border_color: ESColor<E>,
    pub bg_color: ESColor<E>,
    pub text_color: ESColor<E>,
    pub component_border: Border,
    pub spacing: Border,
    pub current_color: ESColor<E>,
    pub current_border: Border,
}

#[derive(PartialEq,Clone)]
pub struct StdLayoutCachors<E> where E: Env {
    pub dims: Dims,
    pub component_border: Border,
    pub spacing: Border,
    pub current_border: Border,
    pub _p: PhantomData<E>,
}

impl<E> StdRenderCachors<E> where E: Env {
    pub fn current(stack: &(impl Queron<E> + ?Sized)) -> Self {
        StdRenderProps::new(stack).current_std_render_cachors()
    }

    // pub fn validate(&self, cached: &mut Option<Self>) -> ValidationStat {
    //     if cached.as_ref() != Some(self) {
    //         *cached = Some(self.clone());
    //         ValidationStat::Invalid
    //     } else {
    //         ValidationStat::Valid
    //     }
    // }

    pub fn layout_part(&self) -> StdLayoutCachors<E> {
        StdLayoutCachors {
            dims: self.dims,
            component_border: self.component_border,
            spacing: self.spacing,
            current_border: self.current_border,
            _p: PhantomData,
        }
    }

    pub fn validate(&self, cached: &mut Option<Self>, invalidate_color: &mut bool, invalidate_layout: &mut bool) {
        if cached.as_ref() != Some(self) {
            if cached.as_ref().map(Self::layout_part) != Some(self.layout_part()) {
                *invalidate_layout = true;
            }
            *invalidate_color = true;
            *cached = Some(self.clone());
        }
    }
}

impl<E> StdLayoutCachors<E> where E: Env {
    pub fn current(stack: &(impl Queron<E> + ?Sized)) -> Self {
        let render_props = StdRenderProps::new(stack);
        let current_style = render_props.style.current();
        Self {
            dims: render_props.absolute_bounds.size,
            component_border: current_style.component_border,
            spacing: current_style.spacing,
            current_border: current_style.current_border,
            _p: PhantomData,
        }
    }

    pub fn validate(&self, cached: &mut Option<Self>) -> ValidationStat {
        if cached.as_ref() != Some(self) {
            *cached = Some(self.clone());
            ValidationStat::Invalid
        } else {
            ValidationStat::Valid
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum ValidationStat {
    Valid,
    Invalid,
}

impl ValidationStat {
    #[inline]
    pub fn from_valid(valid: bool) -> Self {
        match valid {
            true => Self::Valid,
            false => Self::Invalid,
        }
    }
    #[inline]
    pub fn from_invalid(invalid: bool) -> Self {
        Self::from_valid(!invalid)
    }

    #[inline]
    pub fn valid(&self) -> bool {
        *self == ValidationStat::Valid
    }
    #[inline]
    pub fn invalid(&self) -> bool {
        *self == ValidationStat::Invalid
    }
}

impl BitOrAssign<ValidationStat> for bool {
    fn bitor_assign(&mut self, rhs: ValidationStat) {
        *self |= rhs.invalid();
    }
}

impl BitAndAssign<ValidationStat> for bool {
    fn bitand_assign(&mut self, rhs: ValidationStat) {
        *self &= rhs.valid();
    }
}
