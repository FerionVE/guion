use super::*;
use std::marker::PhantomData;
use state::*;

#[derive(Clone)]
pub struct RemoteState<E,T> where E: Env, T: Clone + Default + 'static {
    id: E::WidgetID,
    _p: PhantomData<T>,
}

impl<E,T> RemoteState<E,T> where E: Env, T: Clone + Default + 'static {
    #[inline]
    pub fn for_widget(id: E::WidgetID) -> Self {
        Self{
            id,
            _p: PhantomData,
        }
    }
}

impl<E,T> AtomState<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, for<'a> E::Context<'a>: DynState<E> {
    #[inline]
    fn get(&self, c: &mut E::Context<'_>) -> T {
        c.remote_state_or_default(
            self.id.clone()
        )
    }
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Err(())
    }
}

impl<E,T> AtomStateMut<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, for<'a> E::Context<'a>: DynState<E> {
    #[inline]
    fn set(&mut self, v: T, c: &mut E::Context<'_>) {
        c.push_remote_state(
            self.id.clone(), v
        )
    }
    #[inline]
    fn set_direct(&mut self, _: T) -> Result<(),()> {
        Err(())
    }
}
