use crate::core::ctx::id::WidgetID;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::*;
use crate::core::lazout::size::Size;
use super::*;
///NOTE that E is not the current Context but the underlying
impl<S> Handler for StandardCtx<S> where S: Handler {
    type Child = S;

    fn _child_mut(&mut self) -> &mut Self::Child {
        &mut self.sup
    }
    fn _child(&self) -> &Self::Child {
        &self.sup
    }
}