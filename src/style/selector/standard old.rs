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
    pub border: BorderPtr, 
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
    #[inline]
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
            border: BorderPtr::Default,
            border_mul: 1,
            color_specific: None,
        }
    }
}

impl<E> StyleVariant for StdStyleVariant<E> where E: Env {
    
}

impl<E> StyleVariantSupport<StdSelectag<E>> for StdStyleVariant<E> where E: Env {
    #[inline]
    fn attach(&mut self, v: StdSelectag<E>) {
        match v {
            StdSelectag::ObjDefault => self.obj = Obj::Default,
            StdSelectag::ObjBackground => self.obj = Obj::Background,
            StdSelectag::ObjForeground => self.obj = Obj::Foreground,
            StdSelectag::ObjText => self.obj = Obj::Text,
            StdSelectag::ObjBox => self.obj = Obj::Box,
            StdSelectag::ObjBorder => self.obj = Obj::Border,
            StdSelectag::ObjActive => self.obj = Obj::Active,
            StdSelectag::ObjButton => self.obj = Obj::Button,
            StdSelectag::ObjList => self.obj = Obj::List,
            StdSelectag::ObjTextBox => self.obj = Obj::TextBox,
            StdSelectag::ObjLabel => self.obj = Obj::Label,
            StdSelectag::ObjScroll => self.obj = Obj::Scroll,
            StdSelectag::DesignDefault => self.design = Design::Default,
            StdSelectag::DesignNormal => self.design = Design::Normal,
            StdSelectag::DesignFlat => self.design = Design::Flat,
            StdSelectag::Accent(v) => self.accent = v,
            StdSelectag::VariantDefault => self.variance = Variance::Default,
            StdSelectag::VariantNormal => self.variance = Variance::Normal,
            StdSelectag::VariantOK => self.variance = Variance::OK,
            StdSelectag::VariantCaution => self.variance = Variance::Caution,
            StdSelectag::VariantSecondary => self.variance = Variance::Secondary,
            StdSelectag::Hovered(v) => self.hovered = v,
            StdSelectag::Focused(v) => self.focused = v,
            StdSelectag::Pressed(v) => self.pressed = v,
            StdSelectag::Locked(v) => self.locked = v,
            StdSelectag::CursorDefault => self.cursor = StdCursor::Default,
            StdSelectag::CursorArrow => self.cursor = StdCursor::Arrow,
            StdSelectag::CursorIBeam => self.cursor = StdCursor::IBeam,
            StdSelectag::CursorWait => self.cursor = StdCursor::Wait,
            StdSelectag::CursorCrosshair => self.cursor = StdCursor::Crosshair,
            StdSelectag::CursorWaitArrow => self.cursor = StdCursor::WaitArrow,
            StdSelectag::CursorSizeNWSE => self.cursor = StdCursor::SizeNWSE,
            StdSelectag::CursorSizeNESW => self.cursor = StdCursor::SizeNESW,
            StdSelectag::CursorSizeWE => self.cursor = StdCursor::SizeWE,
            StdSelectag::CursorSizeNS => self.cursor = StdCursor::SizeNS,
            StdSelectag::CursorSizeAll => self.cursor = StdCursor::SizeAll,
            StdSelectag::CursorNo => self.cursor = StdCursor::No,
            StdSelectag::CursorHand => self.cursor = StdCursor::Hand,
            StdSelectag::BorderDefault => self.border = BorderPtr::Default,
            StdSelectag::BorderOuter => self.border = BorderPtr::Outer,
            StdSelectag::BorderVisual => self.border = BorderPtr::Visual,
            StdSelectag::BorderSpecific(v) => self.border = BorderPtr::Specific(v),
            StdSelectag::BorderMultiplierDefault => self.border_mul = 1,
            StdSelectag::BorderMultiplier(v) => self.border_mul = v,
            StdSelectag::BorderMultiply(v) => self.border_mul *= v,
            StdSelectag::ColorDefault => self.color_specific = None,
            StdSelectag::ColorSpecific(v) => self.color_specific = Some(v),
        }        
    }
}

impl<E,T> AddAssign<T> for StdStyleVariant<E> where Self: StyleVariantSupport<T>, T: Clone, E: Env {
    #[inline]
    fn add_assign(&mut self, v: T) {
        self.attach(v)
    }
}

impl<E,T> StyleVariantSupport<&[T]> for StdStyleVariant<E> where Self: StyleVariantSupport<T>, T: Clone, E: Env {
    #[inline]
    fn attach(&mut self, selectors: &[T]) {
        for t in selectors {
            self.attach(t.clone());
        }
    }
}

impl<E> StyleVariantSupport<()> for StdStyleVariant<E> where E: Env {
    #[inline]
    fn attach(&mut self, _: ()) {}
}
impl<E> StyleVariantSupport<&()> for StdStyleVariant<E> where E: Env {
    #[inline]
    fn attach(&mut self, _: &()) {}
}

impl<E> StyleVariantGetStdCursor for StdStyleVariant<E> where E: Env {
    #[inline]
    fn cursor(&self) -> StdCursor {
        self.cursor
    }
}

/*impl<E> Clone for StdStyleVariant<E> where E: Env {
    fn clone(&self) -> Self {
        Self{
            obj: self.obj.clone(),
            design: self.design.clone(),
            accent: self.accent.clone(),
            variance: self.variance.clone(),
            hovered: self.hovered.clone(),
            focused: self.focused.clone(),
            pressed: self.pressed.clone(),
            locked: self.locked.clone(),
            cursor: self.cursor.clone(),
            border_ptr: self.border_ptr.clone(),
            border_mul: self.border_mul.clone(),
            color_specific: self.color_specific.clone(),
        }
    }
}*/
