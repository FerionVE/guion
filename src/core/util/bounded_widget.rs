use crate::core::util::bounds::Bounds;
use crate::core::env::Env;

pub trait IBoundedWidget<E>: Clone where E: Env {
    fn bounds(&self) -> &Bounds;
    fn id(&self) -> E::WidgetID;

    fn into_a(&self) -> BoundedWidget<E> {
        BoundedWidget{
            bounds: self.bounds().clone(),
            id: self.id()
        }
    }
}

#[derive(Clone)]
pub struct BoundedWidget<E> where E: Env {
    pub bounds: Bounds,
    pub id: E::WidgetID,
}

impl<E> IBoundedWidget<E> for BoundedWidget<E> where E: Env {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
}