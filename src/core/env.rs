use crate::core::ctx::widgets::Widgets;
use super::*;

pub trait Env: Sized + 'static {
    type Backend: Backend<Self>;
    type Context: Context;
    type Storage: Widgets<Self>;
    type Queue: Queue<Self>;
    ///regularly just dyn Widget
    type DynWidget: DynWidget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type WidgetPath: WidgetPath<Self>;
    //type Commit: Eq + Ord;
}