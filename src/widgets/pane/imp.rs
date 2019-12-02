use crate::core::util::lazout::size::Size;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::Env;

pub struct Pane<E> where E: Env {
    id: E::WidgetID,
    childs: Vec<BoundedWidget<E>>,
    render: bool,
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

    fn render_invalid(&self) -> bool {
        self.render
    }
    fn set_render_invalid(&mut self, v: bool) {
        self.render=v;
    }

    fn layout_invalid(&self) -> bool {
        unimplemented!()
    }
    fn set_layout_invalid(&mut self, v: bool) {
        unimplemented!()
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