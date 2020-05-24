use super::*;

pub mod imp;

pub struct ProgressBar<E> where E: Env {
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub value: f32,
    pub orientation: Orientation,
}

impl<E> ProgressBar<E> where E: Env {
    pub fn new(id: E::WidgetID, o: Orientation) -> Self {
        Self {
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
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

unsafe impl<E> Statize<E> for ProgressBar<E> where E: Env {
    type Statur = Self;
}

