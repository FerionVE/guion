//TODO implement this stuff

use crate::core::ctx::id::WidgetID;
use crate::core::style::font::PreprocessedText;

pub struct TextBoxState {
    pp: PreprocessedText,
    id: WidgetID,
}

impl ITextBox {
    /// So you are the currently cached/selected TextBox any you invalidated, so please fix the state
    /// return if revalidating was successful, else state will be resetted
    /// 
    /// This function exists to make selections of mutating textboxes (e.g. CRDTs) maintable
    fn revalidate_state(&mut self, s: &mut Option<TextBoxState>) -> bool {
        false
    }
}