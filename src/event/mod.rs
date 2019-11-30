pub trait Event: Clone where Self: Sized {
    ///split Self into some known cases to handle
    fn case(self) -> Events<Self>;

    fn filter(self, subbounds: (u32,u32,u32,u32)) -> Option<Self>;
    fn filter_cloned(&self, subbounds: (u32,u32,u32,u32)) -> Option<Self>; 
}

pub enum Events<E> where E: Event {
    //filtered events...
    ///If the event is not of the generic cases
    Any(E),
}