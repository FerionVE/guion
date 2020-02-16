use crate::core::ctx::aliases::*;
use crate::core::*;
use ctx::*;

pub struct Template<E> where E: Env {
    id: E::WidgetID,
    parent: Option<E::WidgetID>,
    invalid: Option<u32>,
    style: Vec<StdVerb>,
}

impl<E> super::ITemplate<E> for Template<E> where E: Env + 'static, ESVariant<E>: StyleVariantSupport<StdVerb> { //TODO trait bounds also on struct and other impls (fix this on all widgets)
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&self.style[..])
    }

    fn invalid(&self) -> Option<u32> {
        self.invalid //return true if no invalid field is stored
    }
    fn set_invalid(&mut self, v: Option<u32>) {
        self.invalid = v;
    }
}

crate::impl_template!(Template<E>);