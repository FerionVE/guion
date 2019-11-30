use crate::util::bounds::Bounds;

pub trait Event: Clone where Self: Sized {
    ///split Self into some known cases to handle
    fn case(self) -> Events<Self>;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self>; 
}

pub enum Events<E> where E: Event {
    //filtered events...
    ///If the event is not of the generic cases
    Any(E),
}