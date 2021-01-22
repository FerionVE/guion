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

impl<E> StyleVariantSupport<StdSelector<E>> for StdStyleVariant<E> where E: Env {
    #[inline]
    fn attach(&mut self, v: StdSelector<E>) {
        match v {
            StdSelector::ObjDefault => self.obj = Obj::Default,
            StdSelector::ObjBackground => self.obj = Obj::Background,
            StdSelector::ObjForeground => self.obj = Obj::Foreground,
            StdSelector::ObjText => self.obj = Obj::Text,
            StdSelector::ObjBox => self.obj = Obj::Box,
            StdSelector::ObjBorder => self.obj = Obj::Border,
            StdSelector::ObjActive => self.obj = Obj::Active,
            StdSelector::ObjButton => self.obj = Obj::Button,
            StdSelector::ObjList => self.obj = Obj::List,
            StdSelector::ObjTextBox => self.obj = Obj::TextBox,
            StdSelector::ObjLabel => self.obj = Obj::Label,
            StdSelector::ObjScroll => self.obj = Obj::Scroll,
            StdSelector::DesignDefault => self.design = Design::Default,
            StdSelector::DesignNormal => self.design = Design::Normal,
            StdSelector::DesignFlat => self.design = Design::Flat,
            StdSelector::Accent(v) => self.accent = v,
            StdSelector::VariantDefault => self.variance = Variance::Default,
            StdSelector::VariantNormal => self.variance = Variance::Normal,
            StdSelector::VariantOK => self.variance = Variance::OK,
            StdSelector::VariantCaution => self.variance = Variance::Caution,
            StdSelector::VariantSecondary => self.variance = Variance::Secondary,
            StdSelector::Hovered(v) => self.hovered = v,
            StdSelector::Focused(v) => self.focused = v,
            StdSelector::Pressed(v) => self.pressed = v,
            StdSelector::Locked(v) => self.locked = v,
            StdSelector::CursorDefault => self.cursor = StdCursor::Default,
            StdSelector::CursorArrow => self.cursor = StdCursor::Arrow,
            StdSelector::CursorIBeam => self.cursor = StdCursor::IBeam,
            StdSelector::CursorWait => self.cursor = StdCursor::Wait,
            StdSelector::CursorCrosshair => self.cursor = StdCursor::Crosshair,
            StdSelector::CursorWaitArrow => self.cursor = StdCursor::WaitArrow,
            StdSelector::CursorSizeNWSE => self.cursor = StdCursor::SizeNWSE,
            StdSelector::CursorSizeNESW => self.cursor = StdCursor::SizeNESW,
            StdSelector::CursorSizeWE => self.cursor = StdCursor::SizeWE,
            StdSelector::CursorSizeNS => self.cursor = StdCursor::SizeNS,
            StdSelector::CursorSizeAll => self.cursor = StdCursor::SizeAll,
            StdSelector::CursorNo => self.cursor = StdCursor::No,
            StdSelector::CursorHand => self.cursor = StdCursor::Hand,
            StdSelector::BorderDefault => self.border = BorderPtr::Default,
            StdSelector::BorderOuter => self.border = BorderPtr::Outer,
            StdSelector::BorderVisual => self.border = BorderPtr::Visual,
            StdSelector::BorderSpecific(v) => self.border = BorderPtr::Specific(v),
            StdSelector::BorderMultiplierDefault => self.border_mul = 1,
            StdSelector::BorderMultiplier(v) => self.border_mul = v,
            StdSelector::BorderMultiply(v) => self.border_mul *= v,
            StdSelector::ColorDefault => self.color_specific = None,
            StdSelector::ColorSpecific(v) => self.color_specific = Some(v),
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
