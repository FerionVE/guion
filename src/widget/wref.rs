use crate::widget::Widget;
use crate::widget::env::Context;
use crate::widget::env::Env;
use std::any::Any;

impl<E> Widget<E> for E::WidgetID where E: Env {
    
}