use crate::core::*;
use lazout::calc::calc_bounds;
use util::bounds::Bounds;
use lazout::size::Size;
use widget::link::Link;
use widget::Widget;
use ctx::*;
use render::*;
use lazout::Orientation;

pub mod imp;
pub mod o;

pub use imp::*;
pub use o::*;

pub trait IPane<E>: Widget<E> where E: Env {

    fn id(&self) -> E::WidgetID;

    fn style(&self) -> &E::Style;

    fn cached(&mut self) -> Option<&mut Option<Vec<Bounds>>> {
        None
    }

    fn childs(&self) -> &[E::WidgetID];

    fn orientation(&self) -> Orientation;
    fn set_orientation(&mut self, v: Orientation);

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}