use piston_window::Motion;
use std::any::Any;
use crate::core::props::Props;
use piston_window::Event::Loop;
use piston_window::Event;
use piston_window::Input::Button;
use piston_window::Event::Input;
use piston_window::ButtonArgs;
use piston_window::ButtonState;
use piston_window::Button as ButtonKey;
use crate::core::elem::Widget;

pub struct GenericPanel {
    childs: Vec<Box<dyn Widget>>,
    p: Props,

    pressed_buttons: Vec<ButtonArgs>,
    focus_child: usize,
    focus_self: bool,
    cx: f64, cy: f64,
}

impl Widget for GenericPanel {
    fn props(&self) -> &Props {&self.p}
    fn props_mut(&mut self) -> &mut Props {&mut self.p}
    fn as_any(&self) -> &Any {self}
    fn as_any_mut(&mut self) -> &mut Any {self}

    

    fn widget_event(&mut self, e: &Event) {
        match e {
            Input(i) => match i {
                Button(b) => self.evt_button(b),
                _ => {},
            },
            Loop(i) => match i {
                _ => {},
            },
            _ => {},
        }
    }

    fn set_focus(&mut self, focus: Option<bool>) -> bool {
        if let Some(dir) = focus {
            if !self.focus_self {
                //start from first/last element with focusing, else just step
            }
            //iterate from 0 or from end (dir) until one is focused
        } else {
            //catch a may-logic-error, else remove the assert
            debug_assert!(self.focus_self);
            //disable focus of currently focused child
            self.focus_self=false;
        }
        unimplemented!()
    }
}

impl GenericPanel {
    pub fn focus(&mut self, focus: usize) {
        let wg = &mut self.childs[focus];

        for b in &self.pressed_buttons {
            wg.widget_event( &Input(Button(*b)) );
        }
        self.pressed_buttons.clear();
    
        self.focus_child = focus;
    }

    fn evt_button(&mut self, e: &ButtonArgs) {
        if e.state == ButtonState::Release {
            //just remove it
            self.pressed_buttons.retain(|f| e != f );
        } else {
            let e2 = ButtonArgs {
                state: ButtonState::Release,
                scancode: e.scancode,
                button: e.button,
            };

            if self.pressed_buttons.iter().find(|f| e2 == **f ).is_none() {
                self.pressed_buttons.push(e2);
            }
        }

        if let ButtonKey::Mouse(_) = e.button {
            //now we need to store somewhere the current cursor pos
            //iterate through widgets from top to bottom and stop by the first inside the bounds
        }else{
            let wg = &mut self.childs[self.focus_child];
            //redirect
            wg.widget_event( &Input(Button(*e)) );
        }
    }

    fn evt_move(&mut self, e: &Motion) {
        
    }
}