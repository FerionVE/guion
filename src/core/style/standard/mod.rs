use super::*;
use cursor::StdCursor;

#[non_exhaustive]
#[derive(Clone)]
pub struct StdStyleVariant {
    pub obj: Obj,
    pub design: Design,
    pub accent: u32,
    pub variance: Variance,
    pub hovered: bool,
    pub selected: bool,
    pub locked: bool,
    pub cursor: StdCursor,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum Obj {
    Default,
    Background,
    Border,
    Button,
    List,
    TextBox,
    Label,
    Scroll,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum Design {
    Default,
    Normal,
    Flat,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum Variance {
    Default,
    Normal,
    OK,
    Caution,
    Secondary,
}

impl Default for StdStyleVariant {
    fn default() -> Self {
        Self{
            obj: Obj::Default,
            design: Design::Default,
            accent: 0,
            variance: Variance::Default,
            hovered: false,
            selected: false,
            locked: false,
            cursor: StdCursor::Default,
        }
    }
}

impl StyleVariant for StdStyleVariant {
    
}

impl StyleVariantSupport<StdVerb> for StdStyleVariant {
    fn _with(&mut self, v: StdVerb) {
        match v {
            StdVerb::ObjDefault => self.obj = Obj::Default,
            StdVerb::ObjBackground => self.obj = Obj::Background,
            StdVerb::ObjBorder => self.obj = Obj::Border,
            StdVerb::ObjButton => self.obj = Obj::Button,
            StdVerb::ObjList => self.obj = Obj::List,
            StdVerb::ObjTextBox => self.obj = Obj::TextBox,
            StdVerb::ObjLabel => self.obj = Obj::Label,
            StdVerb::ObjScroll => self.obj = Obj::Scroll,
            StdVerb::DesignDefault => self.design = Design::Default,
            StdVerb::DesignNormal => self.design = Design::Normal,
            StdVerb::DesignFlat => self.design = Design::Flat,
            StdVerb::Accent(v) => self.accent = v,
            StdVerb::VariantDefault => self.variance = Variance::Default,
            StdVerb::VariantNormal => self.variance = Variance::Normal,
            StdVerb::VariantOK => self.variance = Variance::OK,
            StdVerb::VariantCaution => self.variance = Variance::Caution,
            StdVerb::VariantSecondary => self.variance = Variance::Secondary,
            StdVerb::Hovered(v) => self.hovered = v,
            StdVerb::Selected(v) => self.selected = v,
            StdVerb::Locked(v) => self.locked = v,
            StdVerb::CursorDefault => self.cursor = StdCursor::Default,
            StdVerb::CursorArrow => self.cursor = StdCursor::Arrow,
            StdVerb::CursorIBeam => self.cursor = StdCursor::IBeam,
            StdVerb::CursorWait => self.cursor = StdCursor::Wait,
            StdVerb::CursorCrosshair => self.cursor = StdCursor::Crosshair,
            StdVerb::CursorWaitArrow => self.cursor = StdCursor::WaitArrow,
            StdVerb::CursorSizeNWSE => self.cursor = StdCursor::SizeNWSE,
            StdVerb::CursorSizeNESW => self.cursor = StdCursor::SizeNESW,
            StdVerb::CursorSizeWE => self.cursor = StdCursor::SizeWE,
            StdVerb::CursorSizeNS => self.cursor = StdCursor::SizeNS,
            StdVerb::CursorSizeAll => self.cursor = StdCursor::SizeAll,
            StdVerb::CursorNo => self.cursor = StdCursor::No,
            StdVerb::CursorHand => self.cursor = StdCursor::Hand,
        }        
    }
}

impl StyleVariantGetStdCursor for StdStyleVariant {
    #[inline]
    fn cursor(&self) -> StdCursor {
        self.cursor
    }
}