use crate::core::util::bounds::Bounds;

pub trait Event: Clone where Self: Sized {
    ///split Self into some known cases to handle
    fn case(self) -> Events<Self>;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self>; 

    fn consuming(&self) -> bool;
}

pub enum Events<E> where E: Event {
    MouseMove(u32,u32,E),
    MouseDown(u32,E),
    MouseUp(u32,E),

    KeyDown(u32,E),
    KeyUp(u32,E),
    KeyPress(u32,E),

    //Resize(u32,u32,E),

    //filtered events...
    ///If the event is not of the generic cases
    Any(E),
}