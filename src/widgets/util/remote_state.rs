use super::*;
use std::{any::TypeId, marker::PhantomData};
use state::{AtomStateXMut, AtomStateX};

pub struct RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
    id: E::WidgetID,
    _p: PhantomData<T>,
}

/*impl<E,T> AtomStateX<E,T> for RemoteState<E,T> where E: Env, T: Clone + Default + 'static, E::Context: AsHandlerStateful<E> {
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
}*/

#[macro_export]
macro_rules! impl_remote_state {
    ($t:ty,$e:ty) => {
        impl $crate::widgets::util::state::AtomStateX<$e,$t> for $crate::widgets::util::remote_state::RemoteState<$e,$t> {
            fn get(&self, c: &mut <$e as $crate::env::Env>::Context) -> $t {
                $crate::state::handler::HandlerStateful::remote_state_or_default(
                    c.state(), self.id.clone()
                )
            }
        }
        
        impl $crate::widgets::util::state::AtomStateXMut<$e,$t> for $crate::widgets::util::remote_state::RemoteState<$e,$t> {
            fn set(&mut self, v: $t, c: &mut <$e as $crate::env::Env>::Context) {
                $crate::state::handler::HandlerStateful::remote_state_or_default(
                    c.state_mut(), v, self.id.clone()
                )
            }
        }
    };
}