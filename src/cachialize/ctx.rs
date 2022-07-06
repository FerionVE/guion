use crate::ctx::Context;
use crate::env::Env;

use super::cache::Cache;

pub trait CtxCachialize<'cc,E>: Context<'cc,E> where E: Env {
    type GlobalCounter: Copy + Clone + PartialEq + Eq + Send + Sync + 'static;
    type Cache: Cache<E> + 'cc;

    /// ```ignore
    /// let (global_counter,overflowed) = self.global_counter.overflowing_add(1);
    /// self.global_counter = global_counter;
    /// if overflowed {
    ///     // clear all caches on counter overflow
    ///     self.cache.clear();
    /// }
    /// global_counter
    /// ```
    fn global_counter_incremented(&mut self) -> Self::GlobalCounter;

    fn cache(&self) -> &Self::Cache;
    fn cache_mut(&mut self) -> &mut Self::Cache;


}
