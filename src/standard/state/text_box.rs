//TODO implement this stuff

use crate::core::*;
use ctx::aliases::*;
use ctx::*;
use state::handler::*;

pub struct TextBoxState<E> where E: Env, ECHandler<E>: AsHandlerStateful<E> {
    pp: ESPPText<E>,
    id: E::WidgetID,
}

/*impl ITextBox { TODO impl
    /// So you are the currently cached/selected TextBox any you invalidated, so please fix the state
    /// return if revalidating was successful, else state will be resetted
    /// 
    /// This function exists to make selections of mutating textboxes (e.g. CRDTs) maintable
    fn revalidate_state(&mut self, s: &mut Option<TextBoxState>) -> bool {
        false
    }
}*/