use super::*;
use std::{any::TypeId, marker::PhantomData};
use state::{AtomStateXMut, AtomStateX};

pub struct RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    id: E::WidgetID,
    _p: PhantomData<T>,
}

impl<E,T> AtomStateX<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    fn get(&self, c: &mut E::Context) -> T {
        c.state().remote_states()
            .entry((self.id.clone(),TypeId::of::<T>()))
            .or_default()
            .clone()
    }
}

impl<E,T> AtomStateXMut<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    fn set(&mut self, v: T, c: &mut E::Context) {
        *c.state().remote_states()
        .entry((self.id.clone(),TypeId::of::<T>()))
        .or_default() = v
    }
}