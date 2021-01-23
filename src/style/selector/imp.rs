use std::borrow::Borrow;

use crate::style::selectag::StyleSelectag;

use super::StyleSelectorAppend;

impl<S,T,E> StyleSelectorAppend<&'_ S,E> for T where
    T: StyleSelectorAppend<S,E>,
    S: StyleSelectag<E>,
{
    fn append(&mut self, selectag: &S) {
        self.append((*selectag).clone())
    }
}
impl<'a,S,T,E> StyleSelectorAppend<&'a [S],E> for T where
    T: StyleSelectorAppend<S,E>,
    S: StyleSelectag<E>,
{
    fn append(&mut self, selectag: &[S]) {
        for i in selectag {
            self.append((*i).clone());
        }
    }

}
