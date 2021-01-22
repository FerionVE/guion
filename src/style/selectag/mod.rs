use super::selector::StyleSelector;

pub mod standard;
pub mod imp;

pub trait StyleSelectag<E>: Clone {

}

pub trait StyleSelectagInto<S,E>: StyleSelectag<E> where S: StyleSelector<E> {
    fn into_selector(self) -> S;
}
