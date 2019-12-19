use crate::core::event::Variant;
use crate::core::util::bounds::Offset;

#[derive(Clone)]
pub struct KbdDown {
    pub key: u32,
}
#[derive(Clone)]
pub struct KbdUp {
    pub key: u32,
}

#[derive(Clone)]
pub struct MouseDown {
    pub key: u32,
}
#[derive(Clone)]
pub struct MouseUp {
    pub key: u32,
}

#[derive(Clone)]
pub struct MouseMove {
    pub dest: Offset,
}

#[derive(Clone)]
pub struct MouseEnter {
    pub dest: Offset,
}
#[derive(Clone)]
pub struct MouseLeave {
    pub dest: Offset,
}

impl Variant for KbdDown {}
impl Variant for KbdUp {}
impl Variant for MouseDown {}
impl Variant for MouseUp {}
impl Variant for MouseMove {}
impl Variant for MouseEnter {}
impl Variant for MouseLeave {}

