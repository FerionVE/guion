use std::ops::Range;

use super::update::TextUpdate;


// TODO impl holds bytepos and cursor_stick_y
pub trait TxtCurSel<E>: Clone + Default /*+Send+Sync*/ {
    fn typ(&self) -> TxtCurSelBytePos;
    fn is_cursor(&self) -> bool;
    fn is_selection(&self) -> bool;

    fn caret(&self) -> usize;

    fn unselect(&mut self);

    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_apply(&mut self, op: TextUpdate<'_>) {
        match op {
            TextUpdate::RemoveChars(range) => self.sync_remove_chars(range),
            TextUpdate::RemoveCharsOld { off, n } => self.sync_remove_chars_old(off, n),
            TextUpdate::PushChars(off, chars) => self.sync_push_chars(off, chars.as_ref()),
            TextUpdate::Replace(chars) => self.sync_replace(chars.as_ref()),
        }
    }

    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_remove_chars(&mut self, range: Range<usize>);
    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_remove_chars_old(&mut self, off: usize, n: usize);
    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_push_chars(&mut self, off: usize, chars: &str);
    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_replace(&mut self, s: &str);

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
