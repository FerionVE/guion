use std::borrow::Cow;
use std::ops::Range;

pub enum TextUpdate<'a> {
    RemoveChars(Range<usize>),
    PushChars(usize,Cow<'a,str>),
    Replace(Range<usize>,Cow<'a,str>),
}
