use super::*;

pub trait ICheckBox<'w>: 'w {
    fn set(&mut self, v: bool);
    fn toggle(&mut self);
}

impl<'w,E,State,Text> ICheckBox<'w> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomStateMut<bool>,
    Text: 'w,
{
    fn set(&mut self, v: bool) {
        self.state.set(v);
    }
    fn toggle(&mut self) {
        self.state.set(!self.state.get());
    }
}

unsafe impl<'w> Statize for dyn ICheckBox<'w> {
    type Statur = dyn ICheckBox<'static>;
}