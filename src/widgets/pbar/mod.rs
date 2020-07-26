use super::*;

pub mod widget;

pub struct ProgressBar<'w,E,Stil> where E: Env {
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub value: f32,
    pub orientation: Orientation,
}

impl<'w,E,Stil> ProgressBar<'w,E,Stil> where E: Env {
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

unsafe impl<'w,E,Stil> Statize<E> for ProgressBar<'w,E,Stil> where E: Env, Stil: StatizeSized<E>+'w, {
    type Statur = ProgressBar<'static,E,Stil::StaturSized>;
}

