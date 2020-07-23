use super::*;

pub mod widget;

pub struct ProgressBar<E,Stil> where E: Env {
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdTag>,
    pub value: f32,
    pub orientation: Orientation,
}

impl<E,Stil> ProgressBar<E,Stil> where E: Env {
    pub fn new(id: E::WidgetID, o: Orientation) -> Self {
        Self {
            id,
            size: Size::empty().into(),
            style: vec![],
            value: 0.0,
            orientation: o,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }

    pub fn with_value(mut self, v: f32) -> Self {
        self.value = v;
        self
    }
}

unsafe impl<E,Stil> Statize<E> for ProgressBar<E,Stil> where E: Env {
    type Statur = Self;
}

