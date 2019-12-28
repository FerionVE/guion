use crate::core::ctx::widgets::Widgets;
use super::*;

pub trait Env: Sized + 'static {
    type Backend: Backend<Self>;
    type Context: Context + Widgets<Self>;
    ///regularly just dyn Widget
    type DynWidget: DynWidget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type SubWidgetID: WidgetID;
    type Commit: Eq + Ord;
}