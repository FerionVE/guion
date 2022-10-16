use std::any::{TypeId, Any};
use std::collections::HashMap;

use crate::env::Env;

pub trait Cache<E> where E: Env {
    fn get<T>(&self, path: &E::WidgetPath, coexistant: &[u8], replacant: &[u8]) -> Option<&T> where T: Send + Sync + 'static;
    fn set<T>(&mut self, path: E::WidgetPath, coexistant: Vec<u8>, replacant: Vec<u8>, v: T) where T: Send + Sync + 'static;
    // do if any counter overflows
    fn clear(&mut self);
}

// pub struct StdCache<E> where E: Env {
//     inner: HashMap<(E::WidgetPath,TypeId,Vec<u8>),(Vec<u8>,Box<dyn Any+Send+Sync>)>,
// }
