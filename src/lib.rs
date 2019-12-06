pub mod core;
pub mod widgets;
pub mod standard;

pub mod macro_prelude {
    pub use crate::widgets::button::IButton;
    pub use crate::widgets::pane::IPane;
    pub use crate::core::widget::*;
    pub use crate::core::widget::handler::*;
    pub use crate::core::widget::handler::*;
    pub use crate::core::ctx::Context;
}