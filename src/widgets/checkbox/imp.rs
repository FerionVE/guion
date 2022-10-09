use super::*;

pub trait ICheckBox<E> where E: Env {
    fn state(&self) -> &dyn AtomState<E,bool>;
    fn set(&self, v: bool, ctx: &mut E::Context<'_>);
}

impl<'w,E,State,Text> ICheckBox<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    State: AtomState<E,bool>,
{
    #[inline]
    fn state(&self) -> &dyn AtomState<E,bool> {
        &self.state
    }
    fn set(&self, v: bool, ctx: &mut E::Context<'_>) {
        self.updater.submit_update(v, ctx);
    }
}

traitcast_for_from_widget!(ICheckBox<E>);
