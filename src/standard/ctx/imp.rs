use crate::core::ctx::id::WidgetID;
use crate::core::ctx::ctx_meta::ContextMeta;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::*;
use crate::core::lazout::size::Size;
use super::*;
///NOTE that E is not the current Context but the underlying
impl<E,S> ContextLayer<E> for StandardCtx<S,E> where E: Context + AsMut<Self>, S: ContextLayer<E> {

}

impl<T,S,E> AsMut<T> for StandardCtx<S,E> where S: ContextLayer<E> + AsMut<T>, E: Context {
    fn as_mut(&mut self) -> &mut T {
        let sup: &mut S = &mut self.sup;
        <S as AsMut<T>>::as_mut(sup)
    }
}