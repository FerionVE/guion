use std::ops::{Deref, DerefMut};

use super::*;

pub type AWidget<'a,E> = WCow<'a,dyn Widget<E> + 'a>;
