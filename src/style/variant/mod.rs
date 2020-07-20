use super::*;

pub mod tag;
pub use tag::*;

pub mod standard;

use standard::StdCursor;

pub trait StyleVariant: Clone + Default {
    
}

pub trait StyleVariantSupport<V>: StyleVariant where V: Copy {
    #[inline]
    fn with(&self, tags: impl IntoIterator<Item=impl Deref<Target=V>>) -> Self where Self: Sized {
        let mut s = self.clone();
        s.attach(tags);
        s
    }
    #[inline]
    fn attach(&mut self, tags: impl IntoIterator<Item=impl Deref<Target=V>>) {
        for v in tags {
            self._with(*v.deref());
        }
    }
    #[doc(hidden)]
    fn _with(&mut self, v: V);
}

pub trait StyleVariantGetStdCursor: StyleVariant {
    fn cursor(&self) -> StdCursor;
} 