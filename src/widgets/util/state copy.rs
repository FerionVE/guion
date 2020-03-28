use super::*;

pub trait AtomState<'w,T>: 'w {
    type Mutur: AtomStateMut<'w,T>;

    fn get(&self) -> T;
}
pub trait AtomStateMut<'w,T>: AtomState<'w,T> {
    fn set(&mut self, v: T);
}

impl<'w,T> AtomState<'w,T> for T where T: Copy + 'w {
    type Mutur = T;

    fn get(&self) -> T {
        *self
    }
}
impl<'l,'w,T> AtomState<'w,T> for &'l T where T: Copy, 'l: 'w {
    type Mutur = &'w mut T;

    fn get(&self) -> T {
        **self
    }
}
impl<'l,'w,T> AtomState<'w,T> for &'l mut T where T: Copy, 'l: 'w {
    type Mutur = &'w mut T;

    fn get(&self) -> T {
        **self
    }
}
impl<'l,'w,T> AtomStateMut<'w,T> for &'l mut T where T: Copy, 'l: 'w {
    fn set(&mut self, v: T) {
        **self = v;
    }
}
impl<'w,T> AtomStateMut<'w,T> for T where T: Copy + 'w {
    fn set(&mut self, v: T) {
        *self = v;
    }
}