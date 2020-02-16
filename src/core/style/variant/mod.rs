use super::*;

pub mod verb;
pub use verb::*;

pub trait StyleVariant: Clone + Default {
    
}

pub trait StyleVariantSupport<V>: StyleVariant where V: Copy {
    #[inline]
    fn with(&self, verbs: impl IntoIterator<Item=impl Deref<Target=V>>) -> Self where Self: Sized {
        let mut s = self.clone();
        s.attach(verbs);
        s
    }
    #[inline]
    fn attach(&mut self, verbs: impl IntoIterator<Item=impl Deref<Target=V>>) {
        for v in verbs {
            self._with(*v.deref());
        }
    }
    #[doc(hidden)]
    fn _with(&mut self, v: V);
}