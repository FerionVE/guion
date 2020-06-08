use super::*;

pub trait ICheckBox<'w,E>: 'w where E: Env {
    fn set(&mut self, v: bool, c: &mut E::Context);
    fn toggle(&mut self, c: &mut E::Context);
}

impl<'w,E,State,Text> ICheckBox<'w,E> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomStateMut<E,bool>,
    Text: 'w,
{
    fn set(&mut self, v: bool, c: &mut E::Context) {
        self.state.set(v,c);
    }
    fn toggle(&mut self, c: &mut E::Context) {
        self.state.set(!self.state.get(c),c);
    }
}

unsafe impl<'w,E> Statize<E> for dyn ICheckBox<'w,E> where E: Env {
    type Statur = dyn ICheckBox<'static,E>;
}