use crate::core::ctx::Context;

pub struct Empty<E> where E: Context {
    id: E::WidgetID,
    parent: Option<E::WidgetID>,
}

impl<E> super::IEmpty<E> for Empty<E> where E: Context + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn invalid(&self) -> bool {
        false
    }
    fn set_invalid(&mut self, _v: bool) {
        
    }

    fn parent(&self) -> Option<E::WidgetID> {
        self.parent.clone()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent = v;
    }
}

crate::impl_empty!(Empty<E>);