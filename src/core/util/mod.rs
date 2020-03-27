use super::*;

pub mod bounds;
pub mod border;
pub mod bounded_widget;
pub mod wpps;
pub mod shortlt;

pub trait AsRefMut<T> {
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
}

impl<T,I> AsRefMut<I> for T where T: AsRef<I> + AsMut<I> {
    #[inline]
    fn as_ref(&self) -> &I {
        self.as_ref()
    }
    #[inline]
    fn as_mut(&mut self) -> &mut I {
        self.as_mut()
    }
}