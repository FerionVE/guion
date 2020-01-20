use crate::core::ctx::widgets::Widgets;
use super::*;

pub trait Env: Sized + 'static {
    type Backend: Backend<Self>;
    type Context: Context<Self>;
    type Storage: Widgets<Self>;
    ///regularly just dyn Widget
    type DynWidget: DynWidget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type WidgetPath: WidgetPath<Self>;
    //type Commit: Eq + Ord;
}