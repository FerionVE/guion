use super::*;

pub trait WPProviders<E> where E: Env {
    fn len(&self) -> usize;
    fn idx(&self, i: usize) -> &dyn WPProvider<E>;
}