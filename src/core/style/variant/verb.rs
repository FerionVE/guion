use std::iter::once;
use std::iter::Once;
/// verbs enable/disable specific parts of styles.  
/// Style implementations may ignore verbs.  
#[non_exhaustive]
#[derive(Copy,Clone)]
pub enum StdVerb {
    ObjDefault,
    ObjBackground,
    ObjForeground,
    ObjBox,
    ObjBorder,
    
    ObjButton,
    ObjList,
    ObjTextBox,
    ObjLabel,
    ObjScroll,
    
    DesignDefault,
    DesignNormal,
    DesignFlat,

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

impl IntoIterator for StdVerb {
    type Item = StdVerb;
    type IntoIter = Once<StdVerb>;

    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}