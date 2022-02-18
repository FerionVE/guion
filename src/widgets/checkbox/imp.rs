use super::*;

pub trait ICheckBox<E> where E: Env {
    fn state(&self) -> &dyn AtomState<E,bool>;
    fn set(&self, l: Link<E>, v: bool);
}

impl<'w,E,State,Text,TrMut> ICheckBox<E> for CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
    State: AtomState<E,bool>,
    TrMut: TriggerMut<E>,
{
    #[inline]
    fn state(&self) -> &dyn AtomState<E,bool> {
        &self.state
    }
    fn set(&self, mut l: Link<E>, v: bool) {
        if let Some(t) = self.updater.boxed(v) {
            l.mutate_closure(t)
        }
    }
}

traitcast_for_from_widget!(ICheckBox<E>);
