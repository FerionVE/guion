use crate::core::ctx::aliases::*;
use crate::core::*;
use lazout::calc::calc_bounds;
use util::bounds::Bounds;
use lazout::size::Size;
use widget::link::Link;
use widget::Widget;
use ctx::*;
use lazout::Orientation;

pub mod imp;
pub mod o;

#[doc(inline)]
pub use imp::*;
#[doc(inline)]
pub use o::*;

pub trait IPane<E>: Widget<E> where E: Env {
    type Child: AsWPSlice<E>;

    fn id(&self) -> E::WidgetID;

    fn style(&self) -> &EStyle<E>;

    fn cached(&mut self) -> Option<&mut Option<Vec<Bounds>>> {
        None
    }

    fn childs(&self) -> &[Self::Child];

    fn orientation(&self) -> Orientation;
    fn set_orientation(&mut self, v: Orientation);

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}