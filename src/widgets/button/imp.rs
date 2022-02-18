use super::*;

pub trait IButton<E> where E: Env {
    fn trigger(&self, l: &mut Link<E>);
}

impl<'w,E,Text,Tr,TrMut> IButton<E> for Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Tr: Trigger<E>,
    TrMut: TriggerMut<E>,
{
    #[inline]
    fn trigger(&self, l: &mut Link<E>) {
        self.trigger.trigger(l.reference());
        if let Some(t) = self.trigger_mut.boxed() {
            l.mutate_closure(t);
        }
    }
}

traitcast_for_from_widget!(IButton<E>);
