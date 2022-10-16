use std::marker::PhantomData;

use super::*;

//TODO rework
pub trait IBoundedWidget<E>: Clone where E: Env {
    fn bounds(&self) -> &Bounds;
    #[inline]
    fn into_a(&self) -> BoundedWidget<E> {
        BoundedWidget{
            bounds: self.bounds().clone(),
            _p: PhantomData,
        }
    }
}

//#[derive(Clone)]
pub struct BoundedWidget<E> where E: Env {
    pub bounds: Bounds,
    pub _p: PhantomData<E>,
}

impl<E> IBoundedWidget<E> for BoundedWidget<E> where E: Env {
    #[inline]
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

impl<E> Clone for BoundedWidget<E> where E: Env {
    #[inline]
    fn clone(&self) -> Self {
        Self{
            bounds: self.bounds,
            _p: PhantomData,
        }
    }
}
