use crate::core::*;
use lazout::size::Size;
use util::bounded_widget::BoundedWidget;
use ctx::*;

pub struct Pane<E> where E: Env {
    id: E::WidgetID,
    childs: Vec<BoundedWidget<E>>,
    invalid: bool,
    parent: Option<E::WidgetID>,
}

impl<E> super::Pane<E> for Pane<E> where E: Env + 'static {
    type C = BoundedWidget<E>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn childs(&self) -> &[Self::C] {
        &self.childs[..]
    }

    fn invalid(&self) -> bool {
        self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }
    
    fn size(&self) -> Size {
        todo!()
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent=v;
    }
}