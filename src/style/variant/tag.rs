use std::iter::once;
use std::iter::Once;
use crate::border::Border;
/// tags enable/disable specific parts of styles.  
/// Style implementations may ignore tags.  
#[non_exhaustive]
#[derive(Copy,Clone)]
pub enum StdTag {
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

impl IntoIterator for StdTag {
    type Item = StdTag;
    type IntoIter = Once<StdTag>;

    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}