use crate::widget::Widget;
use crate::widget::env::Context;
use crate::widget::env::Env;
use std::any::Any;

pub trait WidgetRef<E> where E: Env {
    fn get(&self, c: &E::Ctx) -> &dyn Widget<E>;
    fn get_mut(&mut self, c: &mut E::Ctx) -> &dyn Widget<E>;
}