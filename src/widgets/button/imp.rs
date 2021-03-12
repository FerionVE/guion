use super::*;

pub trait IButton<E> where E: Env {
    fn trigger(&self, l: Link<E>);
    fn trigger_mut(&mut self);
}

impl<'w,E,Text> IButton<E> for Button<'w,E,Text> where
    E: Env,
    Text: 'w,
{
    fn trigger(&self, l: Link<E>) {
        (self.trigger)(l)
    }
    fn trigger_mut(&mut self) {
        (self.trigger_mut)()
    }
}

traitcast_for!(IButton<E>);
