use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::ctx::Context;

pub struct Pane<E> where E: Context {
    id: E::WidgetID,
    childs: Vec<BoundedWidget<E>>,
    invalid: bool,
    parent: Option<E::WidgetID>,
}

impl<E> super::Pane<E> for Pane<E> where E: Context + 'static {
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
        unimplemented!()
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent=v;
    }
}