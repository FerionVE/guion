use super::*;
use std::ops::BitAnd;

#[derive(Clone)]
pub struct EventCompound<E>(pub EEvent<E>,pub Bounds,pub u64,pub EEFilter<E>,pub ESVariant<E>,pub bool) where E: Env;

impl<E> EventCompound<E> where E: Env {
    /// filter event by integrated filter
    #[inline]
    pub fn filter(&self, dest: &Link<'_,E>) -> Option<Self> {
        self.3._filter(dest,self)
    }

    #[inline]
    pub fn slice_bounds(&self, inner_relative: &Bounds) -> Self {
        Self(self.0.clone(),self.1.slice(inner_relative),self.2,self.3.clone(),self.4.clone(),self.5)
    }
    #[inline]
    pub fn inside_border(&self, b: &Border) -> Self {
        Self(self.0.clone(),self.1.inside_border(b),self.2,self.3.clone(),self.4.clone(),self.5)
    }

    #[inline]
    pub fn with_event(&self, e: EEvent<E>) -> Self {
        Self(e,self.1,self.2,self.3.clone(),self.4.clone(),self.5)
    }
    #[inline]
    pub fn with_bounds(&self, b: Bounds) -> Self {
        Self(self.0.clone(),b,self.2,self.3.clone(),self.4.clone(),self.5)
    }
    #[inline]
    pub fn with_filter(&self, f: EEFilter<E>) -> Self {
        Self(self.0.clone(),self.1,self.2,f,self.4.clone(),self.5)
    }

    #[inline]
    pub fn with<V>(&mut self, tags: V) -> Self where ESVariant<E>: StyleVariantSupport<V>, V: Clone {
        Self(self.0.clone(),self.1,self.2,self.3.clone(),self.4.with(tags),self.5)
    }

    #[inline]
    pub fn default_filter(&self) -> Self {
        self.with_filter(Default::default())
    }

    /// filter event just by bounds
    #[inline]
    pub fn filter_bounds(&self) -> Option<Self> {
        self.0.in_bounds(&self.1)
            .map(|| self.clone() )
    }

    #[inline]
    pub fn filter_bounds_by_border<V>(&self, s: &EStyle<E>, by: V) -> Option<Self> where ESVariant<E>: StyleVariantSupport<V>, V: Clone {
        self.inside_border(
            &s.border(&self.4.with(by))
        ).filter_bounds()
    }
}

//TODO opion
impl<E> BitAnd<&Bounds> for &EventCompound<E> where E: Env {
    type Output = EventCompound<E>;
    #[inline]
    fn bitand(self, rhs: &Bounds) -> EventCompound<E> {
        EventCompound(self.0.clone(),self.1 & rhs,self.2,self.3.clone(),self.4.clone(),self.5)
    }
}