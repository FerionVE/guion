use std::borrow::Cow;
use std::ops::Range;

pub enum TextUpdate<'a> {
    RemoveChars(Range<usize>),
    RemoveCharsOld{off:usize,n:usize},
    PushChars(usize,Cow<'a,str>),
    Replace(Cow<'a,str>),
}
