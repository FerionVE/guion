use super::*;

pub trait ICheckBox<E> where E: Env {
    fn state(&self) -> &dyn AtomState<E,bool>;
}
pub trait ICheckBoxMut<E>: ICheckBox<E> where E: Env {
    fn state_mut(&mut self) -> &mut dyn AtomState<E,bool>;
}

impl<'w,E,State,Text> ICheckBox<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomState<E,bool>,
    Text: 'w,
{
    #[inline]
    fn state(&self) -> &dyn AtomState<E,bool> {
        &self.state
    }
}
impl<'w,E,State,Text> ICheckBoxMut<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomState<E,bool>,
    Text: 'w,
{
    #[inline]
    fn state_mut(&mut self) -> &mut dyn AtomState<E,bool> {
        &mut self.state
    }
}

traitcast_for!(ICheckBox<E>;ICheckBoxMut<E>);
