pub mod core;
pub mod widgets;
pub mod standard;

pub mod macro_prelude {
    pub use crate::widgets::button::IButton;
    pub use crate::widgets::pane::IPane;
    pub use crate::widgets::empty::IEmpty;
    pub use crate::widgets::label::ILabel;
    pub(crate) use crate::widgets::template::ITemplate;
    pub use crate::core::widget::*;
    pub use crate::core::widget::handler::*;
    pub use crate::core::ctx::Context;
}