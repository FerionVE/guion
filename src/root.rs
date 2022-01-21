use crate::env::Env;

pub trait RootRef<E> where E: Env {
    fn fork<'s>(&'s self) -> E::RootRef<'s> where Self: 's;
}

pub trait RootMut<E> where E: Env {
    fn fork<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's;
}

impl<'a,T,E> RootRef<E> for &'a T where for<'z> E: Env<RootRef<'z>=&'z T> {
    fn fork<'s>(&'s self) -> E::RootRef<'s> where Self: 's {
        &**self
    }
}

impl<'a,T,E> RootMut<E> for &'a mut T where for<'z> E: Env<RootMut<'z>=&'z mut T> {
    fn fork<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's {
        &mut **self
    }
}

impl<'a,T,E> RootRef<E> for std::borrow::Cow<'a,T> where for<'z> E: Env<RootRef<'z>=std::borrow::Cow<'z,T>>, T: Clone {
    fn fork<'s>(&'s self) -> E::RootRef<'s> where Self: 's {
        std::borrow::Cow::Borrowed(self.as_ref())
    }
}
