use super::*;

pub trait IButton<E> where E: Env {
    fn trigger(&self, l: &mut Link<E>);
}

impl<'w,E,Text,Tr,TrMut> IButton<E> for Button<'w,E,Text,Tr,TrMut> where
    E: Env,
    Text: 'w,
    Tr: Trigger<E>,
    TrMut: for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>) + Clone + 'static,
{
    #[inline]
    fn trigger(&self, l: &mut Link<E>) {
        self.trigger.trigger(l.reference());
        if let Some(t) = &self.trigger_mut {
            l.mutate_closure(Box::new(t.clone()));
        }
    }
}

traitcast_for_from_widget!(IButton<E>);
