use crate::core::ctx::Context;

pub struct Template<E> where E: Context {
    id: E::WidgetID,
    parent: Option<E::WidgetID>,
    invalid: bool,
    style: E::Style,
}

impl<E> super::ITemplate<E> for Template<E> where E: Context + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn style(&self) -> &E::Style {
        &self.style
    }

    fn invalid(&self) -> bool {
        self.invalid //return true if no invalid field is stored
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

crate::impl_template!(Template<E>);