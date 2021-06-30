use crate::*;

use super::SelectedStyle;

pub trait StdAttributes<E>: SelectedStyle<E> where E: Env {
    fn bg_color(&self) -> ESColor<E>;
}
