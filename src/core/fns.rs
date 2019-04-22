use piston_window::Input::*;
use piston_window::Event::*;
use crate::core::elem::Widget;

/**
 * fn for windows events to parse them to the simpler widget events for a widget
 * 
 */
pub trait HandleEvent {
    /**
     * trigger simple event handling for this event on this widget, returns true if the event was handled
     */
    fn handle(&self, w: &mut Widget) -> bool;
}

impl HandleEvent for piston_window::Event {
    fn handle(&self, w: &mut Widget) -> bool {
        match self {
            Input(e) => e.handle(w),
            Loop(e) => e.handle(w),
            _ => false,
        }
    }
}

impl HandleEvent for piston_window::Input {
    fn handle(&self, w: &mut Widget) -> bool {
        match self {
            /*Button(b) => unimplemented!(),
            Move(m) => unimplemented!(),
            Text(t) => unimplemented!(),
            Resize(w,h) => unimplemented!(),
            Focus(f) => unimplemented!(),
            Cursor(c) => unimplemented!(),
            FileDrag(d) => unimplemented!(),
            Close(c) => unimplemented!(),*/
            _ => false,
        }
    }
}

impl HandleEvent for piston_window::Loop {
    fn handle(&self, w: &mut Widget) -> bool {
        match self {
            /*Render(r) => unimplemented!(),
            AfterRender(a) => unimplemented!(),
            Update(u) => unimplemented!(),
            Idle(i) => unimplemented!(),*/
            _ => false,
        }
    }
}