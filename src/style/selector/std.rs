use crate::*;

use super::StyleSelectors;

pub trait AppendStdSelector<E>: StyleSelectors<E> where E: Env {
    fn text_box(self) -> Self;
}
