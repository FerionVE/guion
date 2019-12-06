use crate::core::lazout::calc::calc_bounds;
use crate::core::util::bounds::Bounds;
use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::*;
use crate::core::widget::handler::HandlerFns;
use crate::core::widget::link::Link;
use std::any::Any;
use crate::core::widget::Widget;
use crate::core::env::*;
use crate::core::render::*;
use crate::core::event::Event;
use crate::core::lazout::Orientation;

pub mod imp;
pub mod o;

pub use imp::*;
pub use o::*;

pub trait IPane<E> where E: Env {

    fn id(&self) -> E::WidgetID;

    fn cached(&mut self) -> Option<&mut Option<Vec<Bounds>>> {
        None
    }

    fn childs(&self) -> &[E::WidgetID];

    fn orientation(&self) -> Orientation;
    fn set_orientation(&mut self, v: Orientation);

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}