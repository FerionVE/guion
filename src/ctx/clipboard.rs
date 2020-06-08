use super::*;

//TODO move mod

pub trait CtxClipboardAccess<E> where E: Env {
    fn clipboard_set_text(&mut self, v: &str);
    fn clipboard_get_text(&mut self) -> Option<String>;
}
