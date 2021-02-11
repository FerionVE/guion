use crate::style::selectag::StyleSelectag;
use crate::style::selector::StyleSelectorAppend;

use super::*;
use std::ops::BitAnd;

#[derive(Clone)]
pub struct EventCompound<E> where E: Env {
    pub event: EEvent<E>,
    pub bounds: Bounds,
    pub ts: u64,
    pub filter: EEFilter<E>,
    pub style: EStyle<E>,
    pub flag: bool,
}

impl<E> EventCompound<E> where E: Env {
    /// filter event by integrated filter
    #[inline]
    pub fn filter(&self, dest: &Link<E>) -> Option<Self> {
        self.filter._filter(dest,self)
    }

    #[inline]
    pub fn slice_bounds(&self, inner_relative: &Bounds) -> Self {
        Self{
            bounds: self.bounds.slice(inner_relative),
            ..self.clone()
        }
    }
    #[inline]
    pub fn inside_border(&self, b: &Border) -> Self {
        Self{
            bounds: self.bounds.inside_border(b),
            ..self.clone()
        }
    }

    #[inline]
    pub fn with_event(&self, e: EEvent<E>) -> Self {
        Self{
            event: e,
            ..self.clone()
        }
    }
    #[inline]
    pub fn with_bounds(&self, b: Bounds) -> Self {
        Self{
            bounds: b,
            ..self.clone()
        }
    }
    #[inline]
    pub fn with_filter(&self, f: EEFilter<E>) -> Self {
        Self{
            filter: f,
            ..self.clone()
        }
    }

    #[inline]
    pub fn with_style(&self, s: &EStyle<E>) -> Self {
        Self{
            style: self.style.and(s),
            ..self.clone()
        }
    }

    #[inline]
    pub fn default_filter(&self) -> Self {
        self.with_filter(Default::default())
    }

    /// filter event just by bounds
    #[inline]
    pub fn filter_bounds(&self) -> Option<Self> {
        self.event.in_bounds(&self.bounds)
            .map(#[inline] || self.clone() )
    }

    #[inline]
    pub fn filter_inside_bounds_by_style<S>(&self, selectags: S, c: &mut E::Context) -> Option<Self> where ESSelector<E>: StyleSelectorAppend<S,E>, S: StyleSelectag<E> {
        self.inside_border(
            &self.style.border(&selectags.into_selector(),c)
        ).filter_bounds()
    }
}

//TODO opion
impl<E> BitAnd<&Bounds> for &EventCompound<E> where E: Env {
    type Output = EventCompound<E>;
    #[inline]
    fn bitand(self, rhs: &Bounds) -> EventCompound<E> {
        EventCompound{
            bounds: self.bounds & rhs,
            ..self.clone()
        }
    }
}
