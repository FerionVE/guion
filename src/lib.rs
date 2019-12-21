#![doc(html_logo_url = "https://git.mkg20001.io/ferionve/pm/raw/master/logos/ferionve.svg")]
/// core traits, functionality and utils
pub mod core;
/// standard widgets
pub mod widgets;
/// standard components like the StandardHandler context handler
pub mod standard;

#[doc(hidden)]
pub mod macro_prelude {
    pub use crate::widgets::button::IButton;
    pub use crate::widgets::pane::IPane;
    pub use crate::widgets::empty::IEmpty;
    pub use crate::widgets::label::ILabel;
    pub(crate) use crate::widgets::template::ITemplate;
    pub use crate::core::widget::*;
    pub use crate::core::widget::handlez::*;
    pub use crate::core::ctx::*;
    pub use crate::core::ctx::aliases::*;
    pub use crate::core::render::widgets::*;
    pub use crate::core::state::handler::*;
    pub use crate::core::event::VariantSupport;
    pub use crate::core::event::variants::*;
}