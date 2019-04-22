use piston_window::Event::Loop;
use piston_window::Event;
use piston_window::Input::Button;
use piston_window::Event::Input;
use piston_window::ButtonArgs;
use piston_window::ButtonState;
use piston_window::Button as ButtonKey;
use crate::core::elem::Widget;

pub trait WidgetsAccess {
    fn len(&self) -> usize;
    fn widget_at_mut(&mut self) -> &mut dyn Widget;
}


pub struct ChildWidgets<T> {
    access: &T
    /* we have to store all pressed and unreleased buttons to send a release for on unfocus */
    
}

impl<T> ChildWidgets<T> where T: WidgetsAccess {
    
}