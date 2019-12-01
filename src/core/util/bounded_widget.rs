use crate::core::util::bounds::Bounds;
use crate::core::env::Env;

pub trait BoundedWidget<E>: Clone where E: Env {
    fn bounds(&self) -> &Bounds;
    fn id(&self) -> E::WidgetID;

    fn into_a(&self) -> ABoundedWidget<E> {
        ABoundedWidget{
            bounds: self.bounds().clone(),
            id: self.id()
        }
    }
}

#[derive(Clone)]
pub struct ABoundedWidget<E> where E: Env {
    pub bounds: Bounds,
    pub id: E::WidgetID,
}

impl<E> BoundedWidget<E> for ABoundedWidget<E> where E: Env {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
}