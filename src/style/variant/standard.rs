use super::*;
use std::ops::AddAssign;

#[non_exhaustive]
#[derive(Clone)]
pub struct StdStyleVariant<E> where E: Env {
    pub obj: Obj,
    pub design: Design,
    pub accent: u32,
    pub variance: Variance,
    pub hovered: bool,
    pub focused: bool,
    pub pressed: bool,
    pub locked: bool,
    pub cursor: StdCursor,
    pub border_ptr: BorderPtr, 
    pub border_mul: u32,
    pub color_specific: Option<ESColor<E>>,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum Obj {
    Default,
    Background,
    Foreground,
    Text,
    Box,
    Active,
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

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum StdCursor {
    Default,
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNWSE,
    SizeNESW,
    SizeWE,
    SizeNS,
    SizeAll,
    No,
    Hand,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum BorderPtr {
    Default,
    Outer,
    Visual,
    Specific(Border),
}

impl<E> Default for StdStyleVariant<E> where E: Env {
    fn default() -> Self {
        Self{
            obj: Obj::Default,
            design: Design::Default,
            accent: 0,
            variance: Variance::Default,
            hovered: false,
            focused: false,
            pressed: false,
            locked: false,
            cursor: StdCursor::Default,
            border_ptr: BorderPtr::Default,
            border_mul: 1,
            color_specific: None,
        }
    }
}

impl<E> StyleVariant for StdStyleVariant<E> where E: Env {
    
}

impl<E> StyleVariantSupport<StdTag<E>> for StdStyleVariant<E> where E: Env {
    fn attach(&mut self, v: StdTag<E>) {
        match v {
            StdTag::ObjDefault => self.obj = Obj::Default,
            StdTag::ObjBackground => self.obj = Obj::Background,
            StdTag::ObjForeground => self.obj = Obj::Foreground,
            StdTag::ObjText => self.obj = Obj::Text,
            StdTag::ObjBox => self.obj = Obj::Box,
            StdTag::ObjBorder => self.obj = Obj::Border,
            StdTag::ObjActive => self.obj = Obj::Active,
            StdTag::ObjButton => self.obj = Obj::Button,
            StdTag::ObjList => self.obj = Obj::List,
            StdTag::ObjTextBox => self.obj = Obj::TextBox,
            StdTag::ObjLabel => self.obj = Obj::Label,
            StdTag::ObjScroll => self.obj = Obj::Scroll,
            StdTag::DesignDefault => self.design = Design::Default,
            StdTag::DesignNormal => self.design = Design::Normal,
            StdTag::DesignFlat => self.design = Design::Flat,
            StdTag::Accent(v) => self.accent = v,
            StdTag::VariantDefault => self.variance = Variance::Default,
            StdTag::VariantNormal => self.variance = Variance::Normal,
            StdTag::VariantOK => self.variance = Variance::OK,
            StdTag::VariantCaution => self.variance = Variance::Caution,
            StdTag::VariantSecondary => self.variance = Variance::Secondary,
            StdTag::Hovered(v) => self.hovered = v,
            StdTag::Focused(v) => self.focused = v,
            StdTag::Pressed(v) => self.pressed = v,
            StdTag::Locked(v) => self.locked = v,
            StdTag::CursorDefault => self.cursor = StdCursor::Default,
            StdTag::CursorArrow => self.cursor = StdCursor::Arrow,
            StdTag::CursorIBeam => self.cursor = StdCursor::IBeam,
            StdTag::CursorWait => self.cursor = StdCursor::Wait,
            StdTag::CursorCrosshair => self.cursor = StdCursor::Crosshair,
            StdTag::CursorWaitArrow => self.cursor = StdCursor::WaitArrow,
            StdTag::CursorSizeNWSE => self.cursor = StdCursor::SizeNWSE,
            StdTag::CursorSizeNESW => self.cursor = StdCursor::SizeNESW,
            StdTag::CursorSizeWE => self.cursor = StdCursor::SizeWE,
            StdTag::CursorSizeNS => self.cursor = StdCursor::SizeNS,
            StdTag::CursorSizeAll => self.cursor = StdCursor::SizeAll,
            StdTag::CursorNo => self.cursor = StdCursor::No,
            StdTag::CursorHand => self.cursor = StdCursor::Hand,
            StdTag::BorderDefault => self.border_ptr = BorderPtr::Default,
            StdTag::BorderOuter => self.border_ptr = BorderPtr::Outer,
            StdTag::BorderVisual => self.border_ptr = BorderPtr::Visual,
            StdTag::BorderSpecific(v) => self.border_ptr = BorderPtr::Specific(v),
            StdTag::BorderMultiplierDefault => self.border_mul = 1,
            StdTag::BorderMultiplier(v) => self.border_mul = v,
            StdTag::ColorDefault => self.color_specific = None,
            StdTag::ColorSpecific(v) => self.color_specific = Some(v),
        }        
    }
}

impl<E,T> AddAssign<T> for StdStyleVariant<E> where Self: StyleVariantSupport<T>, T: Clone, E: Env {
    fn add_assign(&mut self, v: T) {
        self.attach(v)
    }
}

impl<E,T> StyleVariantSupport<&[T]> for StdStyleVariant<E> where Self: StyleVariantSupport<T>, T: Clone, E: Env {
    fn attach(&mut self, tags: &[T]) {
        for t in tags {
            self.attach(t.clone());
        }
    }
}

impl<E> StyleVariantSupport<()> for StdStyleVariant<E> where E: Env {
    fn attach(&mut self, tags: ()) {}
}
impl<E> StyleVariantSupport<&()> for StdStyleVariant<E> where E: Env {
    fn attach(&mut self, tags: &()) {}
}

impl<E> StyleVariantGetStdCursor for StdStyleVariant<E> where E: Env {
    #[inline]
    fn cursor(&self) -> StdCursor {
        self.cursor
    }
}

