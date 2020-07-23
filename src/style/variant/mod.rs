use super::*;

pub mod tag;
pub use tag::*;

pub mod standard;

use standard::StdCursor;

pub trait StyleVariant: Clone + Default {
    
}

pub trait StyleVariantSupport<V>: StyleVariant where V: Clone {
    #[inline]
    fn with(&self, tags: V) -> Self where Self: Sized {
        let mut s = self.clone();
        s.attach(tags);
        s
    }
    fn attach(&mut self, tags: V);
}

pub trait StyleVariantGetStdCursor: StyleVariant {
    fn cursor(&self) -> StdCursor;
} 