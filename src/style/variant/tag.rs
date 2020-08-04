use std::iter::once;
use std::iter::Once;
use crate::{env::Env, border::Border, aliases::ESColor};
/// tags enable/disable specific parts of styles.  
/// Style implementations may ignore tags.  
#[non_exhaustive]
#[derive(Clone)]
pub enum StdTag<E> where E: Env {
    ObjDefault,
    ObjBackground,
    ObjForeground,
    ObjText,
    ObjBox,
    ObjBorder,
    ObjActive,
    
    ObjButton,
    ObjList,
    ObjTextBox,
    ObjLabel,
    ObjScroll,
    
    DesignDefault,
    DesignNormal,
    DesignFlat,

    BorderDefault,
    /// pick the widget outer border for requesting border size
    BorderOuter,
    /// pick the visual border for requesting border size
    BorderVisual,
    BorderSpecific(Border),

    BorderMultiplierDefault,
    BorderMultiplier(u32),

    ColorDefault,
    ColorSpecific(ESColor<E>),

    Accent(u32),

    VariantDefault,
    VariantNormal,
    VariantOK,
    VariantCaution,
    VariantSecondary,

    Hovered(bool),
    Focused(bool),
    Pressed(bool),
    Locked(bool),

    CursorDefault,
    CursorArrow,
    CursorIBeam,
    CursorWait,
    CursorCrosshair,
    CursorWaitArrow,
    CursorSizeNWSE,
    CursorSizeNESW,
    CursorSizeWE,
    CursorSizeNS,
    CursorSizeAll,
    CursorNo,
    CursorHand,
}

impl<E> IntoIterator for StdTag<E> where E: Env {
    type Item = StdTag<E>;
    type IntoIter = Once<StdTag<E>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}
