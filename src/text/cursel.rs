use std::ops::Range;

// TODO impl holds bytepos and cursor_stick_y
pub trait TxtCurSel<E>: Default + Clone + Default /*+Send+Sync*/ {
    type Cachor: Clone + PartialEq + 'static;

    fn cachor(&self) -> Self::Cachor;
    
    fn typ(&self) -> TxtCurSelBytePos;
    fn is_cursor(&self) -> bool;
    fn is_selection(&self) -> bool;

    fn caret(&self) -> usize;

    fn unselect(&mut self);

    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_replace(&mut self, replace_range: Range<usize>, insert: &str);

    fn attempt_insert_text(&self, insert_len_bytes: usize, base_text_len: usize) -> (usize,Self);
    fn attempt_replace_text(&self, replacant_len_bytes: usize, base_text_len: usize) -> (Range<usize>,Self);
    fn attempt_backspace(&self, backspace_bytes: usize, base_text_len: usize) -> (Range<usize>,Self);
}

pub enum TxtCurSelBytePos {
    Cursor(usize),
    Selection(Range<usize>),
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
