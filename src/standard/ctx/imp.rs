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

impl<E,S> AsMut<S> for E where E: Context + AsMut<StandardCtx<S,E>>, S: ContextLayer<E> {

}