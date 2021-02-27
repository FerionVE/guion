use super::*;

pub trait ICheckBox<E> where E: Env {
    fn get(&self, c: &mut E::Context) -> bool;
    fn set(&mut self, v: bool, c: &mut E::Context);
}

impl<'w,E,State,Text> ICheckBox<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomState<E,bool>,
    Text: 'w,
{
    #[inline]
    fn get(&self, c: &mut E::Context) -> bool {
        self.state.get(c)
    }
    #[inline]
    fn set(&mut self, v: bool, c: &mut E::Context) {
        self.state.try_set(v,c);
    }
}
impl<'w,E,State,Text> ICheckBox<E> for &'_ CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomState<E,bool>,
    Text: 'w,
{
    #[inline]
    fn get(&self, c: &mut E::Context) -> bool {
        self.state.get(c)
    }
    #[inline]
    fn set(&mut self, v: bool, c: &mut E::Context) {
        self.state.try_set(v,c);
    }
}
impl<'w,E,State,Text> ICheckBox<E> for &'_ mut CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomState<E,bool>,
    Text: 'w,
{
    #[inline]
    fn get(&self, c: &mut E::Context) -> bool {
        self.state.get(c)
    }
    #[inline]
    fn set(&mut self, v: bool, c: &mut E::Context) {
        self.state.try_set(v,c);
    }
}


traitcast_for!(ICheckBox<E>);
