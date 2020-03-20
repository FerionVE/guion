use super::*;

pub trait Data<T> {
    fn with<R>(f: impl FnOnce(T)->R)->R;
}

pub struct SizeCache<E> where E: Env {
    c: Option<ESize<E>>,
}

impl<E> SizeCache<E> where E: Env {
    fn with(&self, l: Link<E>, f: fn(Link<E>) -> ESize<E>) -> ESize<E> {
        if let Some(c) = &self.c {
            c.clone()
        }else{
            c = f(l);
            l.enqueue_validate_size(c.clone());
            c
        }
    }
    fn invalidate(&mut self) {
        self.c = None;
    }
}