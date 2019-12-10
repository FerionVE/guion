use std::iter::once;
use std::iter::Once;

pub enum StyleVerb {
    ObjBorder(),
    ObjButton(),
    ObjList(),
    ObjTextBox(),
    ObjLabel(),
    ObjScroll(),
    
    DesignDefault(),
    DesignNormal(),
    DesignFlat(),

    Accent(u32),

    VariantDefault(),
    VariantNormal(),
    VariantOK(),
    VariantCaution(),
    VariantSecondary(),

    Hovered(bool),
    Selected(bool),
    Locked(bool),

    CursorDefault(),
    CursorArrow(),
    CursorIBeam(),
    CursorWait(),
    CursorCrosshair(),
    CursorWaitArrow(),
    CursorSizeNWSE(),
    CursorSizeNESW(),
    CursorSizeWE(),
    CursorSizeNS(),
    CursorSizeAll(),
    CursorNo(),
    CursorHand(),
}

impl IntoIterator for StyleVerb {
    type Item = StyleVerb;
    type IntoIter = Once<StyleVerb>;

    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}