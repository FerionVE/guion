use crate::core::lazout::Orientation;
use crate::core::ctx::Context;
use crate::core::style::Style;

pub struct Pane<E> where E: Context {
    id: E::WidgetID,
    childs: Vec<E::WidgetID>,
    invalid: bool,
    parent: Option<E::WidgetID>,
    orientation: Orientation,
}

impl<E> super::IPane<E> for Pane<E> where E: Context + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn childs(&self) -> &[E::WidgetID] {
        &self.childs[..]
    }
    fn style(&self) -> E::Style {
        <E::Style as Style>::default()
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }
    fn set_orientation(&mut self, v: Orientation) {
        self.orientation = v;
    }

    fn invalid(&self) -> bool {
        self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }

    fn parent(&self) -> Option<E::WidgetID> {
        self.parent.clone()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent = v;
    }
}

crate::impl_pane!(Pane<E>);