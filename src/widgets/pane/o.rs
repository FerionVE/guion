use crate::core::widget::Widget;
use crate::core::lazout::Orientation;
use std::marker::PhantomData;
use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::Env;

pub struct Pane<E> where E: Env {
    id: E::WidgetID,
    childs: Vec<E::WidgetID>,
    invalid: bool,
    parent: Option<E::WidgetID>,
    orientation: Orientation,
}

impl<E> super::IPane<E> for Pane<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn childs(&self) -> &[E::WidgetID] {
        &self.childs[..]
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

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent = v;
    }
}

impl<E> Widget<E> for Pane<E> where E: Env + 'static {
    crate::impl_pane!(Pane<E>,E);
}