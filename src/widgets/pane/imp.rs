use crate::core::util::bounded_widget::ABoundedWidget;
use crate::core::env::Env;

pub struct Pane<E> where E: Env {
    id: E::WidgetID,
    childs: Vec<ABoundedWidget<E>>,
    render: bool,
    parent: Option<E::WidgetID>,
}

impl<E> super::Pane<E> for Pane<E> where E: Env + 'static {
    type C = ABoundedWidget<E>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn childs(&self) -> &[Self::C] {
        &self.childs[..]
    }

    fn render(&self) -> bool {
        self.render
    }
    fn set_render(&mut self, v: bool) {
        self.render=v;
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent=v;
    }
}