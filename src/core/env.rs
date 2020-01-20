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
    type ValidState: ValidState;
    //type Commit: Eq + Ord;
}

pub trait ValidState {
    fn valid() -> Self;
    fn rerender(&self) -> bool;
    fn relayout(&self) -> bool;
}