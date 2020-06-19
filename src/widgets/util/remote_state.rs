use super::*;
use std::marker::PhantomData;
use state::*;

pub struct RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    id: E::WidgetID,
    _p: PhantomData<T>,
}

impl<E,T> RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            _p: PhantomData,
        }
    }
}

impl<E,T> AtomState<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    fn get(&self, c: &mut E::Context) -> T {
        c.state().remote_state_or_default(
            self.id.clone()
        )
    }
    fn get_direct(&self) -> Result<T,()> {
        Err(())
    }
}

impl<E,T> AtomStateMut<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    fn set(&mut self, v: T, c: &mut E::Context) {
        c.state_mut().push_remote_state(
            self.id.clone(), v
        )
    }
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        Err(())
    }
}
