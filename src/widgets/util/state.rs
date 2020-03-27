use super::*;

pub trait AtomState<T> {
    fn get(&self) -> T;
}
pub trait AtomStateMut<T>: AtomState<T> {
    fn set(&mut self, v: T);
}

impl<T> AtomState<T> for T where T: Copy {
    fn get(&self) -> T {
        *self
    }
}
impl<T> AtomState<T> for &T where T: Copy {
    fn get(&self) -> T {
        **self
    }
}
impl<T> AtomState<T> for &mut T where T: Copy {
    fn get(&self) -> T {
        **self
    }
}
impl<T> AtomStateMut<T> for &mut T where T: Copy {
    fn set(&mut self, v: T) {
        **self = v;
    }
}
impl<T> AtomStateMut<T> for T where T: Copy {
    fn set(&mut self, v: T) {
        *self = v;
    }
}