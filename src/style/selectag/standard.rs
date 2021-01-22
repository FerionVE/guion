use std::iter::once;
use std::iter::Once;
use std::marker::PhantomData;
use crate::{env::Env, border::Border, aliases::ESColor};

use super::StyleSelectag;
/// selectors enable/disable specific parts of styles.  
/// Style implementations may ignore selectors.  
#[non_exhaustive]
#[derive(Clone)]
pub enum StdSelectag<E> where E: Env {
    ObjDefault,
    ObjBackground,
    ObjForeground,
    ObjText,
    ObjBox,
    ObjBorder,
    ObjActive,
    
    ObjButton,
    ObjList,
    ObjTextBox,
    ObjLabel,
    ObjScroll,
    
    DesignDefault,
    DesignNormal,
    DesignFlat,

    BorderDefault,
    /// pick the widget outer border for requesting border size
    BorderOuter,
    /// pick the visual border for requesting border size
    BorderVisual,

    Accent(u32),

    VariantDefault,
    VariantNormal,
    VariantOK,
    VariantCaution,
    VariantSecondary,

    Hovered(bool),
    Focused(bool),
    Pressed(bool),
    Locked(bool),

    _P(PhantomData<E>),
}

impl<E> IntoIterator for StdSelectag<E> where E: Env {
    type Item = StdSelectag<E>;
    type IntoIter = Once<StdSelectag<E>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}

impl<E> StyleSelectag<E> for StdSelectag<E> where E: Env {

}
impl<E> StyleSelectag<E> for &'_ StdSelectag<E> where E: Env {

}
impl<E> StyleSelectag<E> for &'_ [StdSelectag<E> ]where E: Env {

}
impl<E> StyleSelectag<E> for &'_ [&'_ StdSelectag<E>] where E: Env {

}
