use super::*;
use std::marker::PhantomData;

pub mod widget;

pub struct ProgressBar<'w,E> where 
    E: Env,
{
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub value: f32,
    pub orientation: Orientation,
    p: PhantomData<&'w ()>,
}

impl<'w,E> ProgressBar<'w,E> where 
    E: Env,
{
    #[inline]
    pub fn new(o: Orientation) -> Self {
        Self {
            size: Gonstraints::empty(),
            style: Default::default(),
            value: 0.0,
            orientation: o,
            p: PhantomData,
        }
    }
}

impl<'w,E> ProgressBar<'w,E> where 
    E: Env,
{
    #[inline]
    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    #[inline]
    pub fn with_size(mut self, size: ESize<E>) -> Self {
        self.size = size;
        self
    }
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }
}
