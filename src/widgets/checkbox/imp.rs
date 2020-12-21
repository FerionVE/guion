use crate::util::traitcast::{Traitcast, TraitcastMut};

use super::*;

pub trait ICheckBox<E> where E: Env {
    fn state(&self) -> &dyn AtomState<E,bool>;
}
pub trait ICheckBoxMut<E>: ICheckBox<E> where E: Env {
    fn state_mut(&mut self) -> &mut dyn AtomStateMut<E,bool>;
}

impl<'w,E,State,Text,Stil> ICheckBox<E> for CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    State: AtomState<E,bool>,
    Text: 'w,
    Stil: 'w,
{
    #[inline]
    fn state(&self) -> &dyn AtomState<E,bool> {
        &self.state
    }
}
impl<'w,E,State,Text,Stil> ICheckBoxMut<E> for CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    State: AtomStateMut<E,bool>,
    Text: 'w,
    Stil: 'w,
{
    #[inline]
    fn state_mut(&mut self) -> &mut dyn AtomStateMut<E,bool> {
        &mut self.state
    }
}

unsafe impl<'w,E> Statize<E> for dyn ICheckBox<E>+'_ where E: Env {
    type Statur = dyn ICheckBox<E>;
}
unsafe impl<'w,E> Statize<E> for dyn ICheckBoxMut<E>+'_ where E: Env {
    type Statur = dyn ICheckBoxMut<E>;
}

traitcast_for!(ICheckBox<E>;ICheckBoxMut<E>);
