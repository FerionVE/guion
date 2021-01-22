use std::borrow::Borrow;

use crate::style::selectag::StyleSelectag;

use super::StyleSelectorAppend;

/*impl<S,T,U,E> StyleSelectorAppend<S,E> for T where
    T: StyleSelectorAppend<U,E>,
    S: IntoIterator<Item=U>,
{
    fn append(&mut self, selectag: S) {
        for i in selectag.into_iter() {
            self.append(i);
        }
    }
}*/

/*impl<S,T,E> StyleSelectorAppend<&'_ S,E> for T where
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

}*/
