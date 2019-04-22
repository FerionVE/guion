/*! 
 * a Widget is just a generic Widget which is somewhere on a tree structure or part of a tree structure
 * 
 * a Widget holds a Props with the generic properties
 */

use std::any::Any;
use piston_window::G2d;
use piston_window::Input;
use piston_window::Event;
use crate::core::props::Props;

pub trait Widget {
    /*macro derived fns*/
    fn props(&self) -> &Props;
    fn props_mut(&mut self) -> &mut Props;
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;

    /*fn window_event(&mut self, e: &Event) {
        unimplemented!()
        //filter event for widget
    }*/

    fn widget_event(&mut self, e: &Event);

    /**
     * alter focus state of widget
     * 
     * focus: 
     *      Some(       : Widget should get focus
     * 
     *          dir     : get focus in forward direction
     *          !dir    : get focus in backward direction
     *      )
     *      None        : Widget should lose focus
     * 
     * return: if the focus request was accepted (ex. non-focusable widgets like labels always return false)
     */
    fn set_focus(&mut self, focus: Option<bool>) -> bool;
}