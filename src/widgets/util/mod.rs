use super::*;

pub mod caption;
pub mod state;

pub trait Data<T> {
    fn with<R>(f: impl FnOnce(T)->R)->R;
}

pub struct SizeCache<E> where E: Env {
    c: Option<ESize<E>>,
}

impl<E> SizeCache<E> where E: Env {
    fn with(&self, mut l: Link<E>, f: fn(Link<E>) -> ESize<E>) -> ESize<E> {
        if let Some(c) = &self.c {
            c.clone()
        }else{
            let c = f(l.reference());
            l.enqueue_validate_size(c.clone());
            c
        }
    }
    fn invalidate(&mut self) {
        self.c = None;
    }
}