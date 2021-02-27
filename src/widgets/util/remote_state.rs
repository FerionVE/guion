use super::*;
use std::marker::PhantomData;
use state::*;

#[derive(Clone)]
pub struct RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: DynState<E> {
    id: E::WidgetID,
    _p: PhantomData<T>,
}

impl<E,T> RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: DynState<E> {
    #[inline]
    pub fn for_widget(id: E::WidgetID) -> Self {
        Self{
            id,
            _p: PhantomData,
        }
    }
}

impl<E,T> AtomState<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: DynState<E> {
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        c.remote_state_or_default(
            self.id.clone()
        )
    }
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Err(())
    }
    #[inline]
    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,T>,GuionError<E>> {
        Ok(self)
    }
    #[inline]
    fn try_set(&mut self, v: T, c: &mut E::Context) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set(self,v,c);
        Ok(())
    }
    #[inline]
    fn try_set_direct(&mut self, v: T) -> Result<(),GuionError<E>> {
        AtomStateMut::<E,T>::set_direct(self,v)?;
        Ok(())
    }
}

impl<E,T> AtomStateMut<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: DynState<E> {
    #[inline]
    fn set(&mut self, v: T, c: &mut E::Context) {
        c.push_remote_state(
            self.id.clone(), v
        )
    }
    #[inline]
    fn set_direct(&mut self, _: T) -> Result<(),()> {
        Err(())
    }
}
