use crate::core::ctx::Context;

pub struct Label<E> where E: Context {
    id: E::WidgetID,
    parent: Option<E::WidgetID>,
    invalid: bool,
}

impl<E> super::ILabel<E> for Label<E> where E: Context + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
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

crate::impl_label!(Label<E>);