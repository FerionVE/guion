use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub mod context;
pub use context::*;

pub trait Env: Sized + Clone {
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: PartialEq + Clone;
    type Commit: Eq + Ord;
    type Ctx: Context<Self>;
}

