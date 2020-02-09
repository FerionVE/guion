use crate::core::ctx::aliases::*;
use crate::core::*;
use ctx::*;

pub struct Null<E> where E: Env {
    id: E::WidgetID,
    parent: Option<E::WidgetID>,
    style: Vec<StyleVerb>,
}

impl<E> Null<E> where E: Env {
    pub fn new(id: E::WidgetID, parent: Option<E::WidgetID>, style: Vec<StyleVerb>) -> Self {
        Self{
            id,
            parent,
            style
        }
    }
}

impl<E> super::INull<E> for Null<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn style(&self, s: &mut EStyle<E>) {
        s.attach(&self.style[..])
    }

    fn invalid(&self) -> bool {
        true
    }
    fn set_invalid(&mut self, v: bool) {
        
    }

    fn parent(&self) -> Option<E::WidgetID> {
        self.parent.clone()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent = v;
    }
}

crate::impl_null!(Null<E>);