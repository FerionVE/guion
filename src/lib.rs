//#![doc(html_logo_url = "https://git.mkg20001.io/ferionve/pm/raw/master/logos/ferionve.svg")]
/// core traits, functionality and utils
pub mod core;
/// standard widgets
pub mod widgets;
/// standard components like the StandardHandler context handler
pub mod standard;

#[doc(hidden)]
pub(crate) mod macro_prelude {
    //pub(crate) use crate::widgets::template::ITemplate;
    //pub use crate::widgets::null::INull;
    //pub use crate::widgets::*;
    //pub use button::IButton;
    //pub use pane::IPane;
    //pub use empty::IEmpty;
    //pub use label::ILabel;
    pub use crate::core::*;
    pub use env::*;
    pub use backend::*;
    pub use widget::*;
    pub use ctx::*;
    pub use ctx::aliases::*;
    pub use render::widgets::*;
    pub use state::handler::*;
    pub use event::*;
    pub use event::variants::*;
}
