use super::*;

pub trait IButton<E> where E: Env {
    fn trigger_auto(&self, l: &mut Link<E>);
    fn trigger(&self, l: Link<E>);
    fn trigger_mut(&mut self, c: &mut E::Context<'_>);
}

impl<'w,E,Text,Tr,TrMut> IButton<E> for Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Text: 'w,
    Tr: Trigger<E>,
    TrMut: TriggerMut<E>,
{
    #[inline]
    fn trigger_auto(&self, l: &mut Link<E>) {
        self.trigger(l.reference());
        if self.trigger.run_mut_trigger(l.reference()) {
            l.mutate_closure(Box::new(move |mut w,c,_|
                w.traitcast_mut::<dyn TriggerMut<E>>().unwrap().trigger_mut(c)
            ));
        }
    }
    #[inline]
    fn trigger(&self, l: Link<E>) {
        self.trigger.trigger(l)
    }
    #[inline]
    fn trigger_mut(&mut self, c: &mut E::Context<'_>) {
        self.trigger_mut.trigger_mut(c)
    }
}

traitcast_for!(IButton<E>);
