use crate::core::ctx::aliases::*;
use crate::core::*;
use ctx::*;

pub struct Null<E> where E: Env {
    id: E::WidgetID,
    style: Vec<StdVerb>,
}

impl<E> Null<E> where E: Env {
    pub fn new(id: E::WidgetID, style: Vec<StdVerb>) -> Self {
        Self{
            id,
            style
        }
    }
}

impl<E> super::INull<E> for Null<E> where E: Env + 'static, ERenderer<E>: RenderStdWidgets<E>, ESVariant<E>: StyleVariantSupport<StdVerb> {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjBackground(),StdVerb::Accent(0)]);
        s.attach(&self.style[..]);
    }

    fn invalid(&self) -> Option<u32> {
        None
    }
    fn set_invalid(&mut self, v: Option<u32>) {
        
    }
}

crate::impl_null!(Null<E>);