use std::any::Any;
use std::sync::Arc;

use crate::aliases::ETextLayout;
use crate::env::Env;

pub mod state;
//pub mod remote_state;

// pub trait Data<T> {
//     fn with<R>(f: impl FnOnce(T)->R)->R;
// }

// pub struct SizeCache<E> where E: Env {
//     c: Option<ESize<E>>,
// }

// impl<E> SizeCache<E> where E: Env {
//     fn with(&self, mut l: Link<E>, f: fn(Link<E>) -> ESize<E>) -> ESize<E> {
//         if let Some(c) = &self.c {
//             c.clone()
//         }else{
//             let c = f(l.reference());
//             l.enqueue_validate_size(c.clone());
//             c
//         }
//     }
//     fn invalidate(&mut self) {
//         self.c = None;
//     }
// }

pub type LocalGlyphCache<E> = Option<(Arc<ETextLayout<E>>,Arc<dyn Any>)>;
