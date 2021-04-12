use crate::style::selector::StyleSelectorAppend;

use super::{StyleSelectag, StyleSelectagInto};

impl<S,T,E> StyleSelectagInto<S,E> for T where
    T: StyleSelectag<E>,
    S: StyleSelectorAppend<T,E>
{
    #[inline]
    fn into_selector(self) -> S {
        StyleSelectorAppend::from(self)
    }
}

impl<S,E> StyleSelectag<E> for &S where S: StyleSelectag<E> {

}
impl<S,E> StyleSelectag<E> for &[S] where S: StyleSelectag<E> {
    
}
