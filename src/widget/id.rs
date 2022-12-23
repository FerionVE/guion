use std::num::NonZeroUsize;

// single WidgetID is unique, the NonZeroUsize starts at 256, so the first 255 values can be used for e.g. special routing modes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WidgetID(pub NonZeroUsize);
