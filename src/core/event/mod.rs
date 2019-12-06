use crate::core::env::Env;
use crate::core::util::bounds::Bounds;

pub trait Event<E>: Sized + Clone where E: Env<Event=Self> {
    ///split Self into some known cases to handle
    fn case(self) -> Events<E>;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self>; 

    fn consuming(&self) -> bool;
}

pub enum Events<E> where E: Env {
    MouseMove(u32,u32,E::Event),
    MouseDown(u32,E::Event),
    MouseUp(u32,E::Event),

    KeyDown(u32,E::Event),
    KeyUp(u32,E::Event),
    KeyPress(u32,E::Event),

    //Resize(u32,u32,E),

    DragStart(E::Event),
    DragStop(E::Event),

    DropHover(E::WidgetID,E::Event),
    DropDo(E::WidgetID,E::Event),

    //filtered events...
    ///If the event is not of the generic cases
    Any(E::Event),
}
//TODO move to drag handler feature module
pub enum DragItem<E> where E: Env {
    Widget(E::WidgetID),
    Text(String),
    File(String),
}