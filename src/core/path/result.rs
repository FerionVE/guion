use super::*;

pub enum ResolveResult<'a,E> where E: Env {
    Hit(&'a E::DynWidget),
    Miss(),
    Link(E::WidgetPath),
}

pub enum ResolveResultMut<'a,E> where E: Env {
    Hit(&'a mut E::DynWidget),
    Miss(),
    Link(E::WidgetPath),
}

impl<'a,E> ResolveResultMut<'a,E> where E: Env {
    pub fn into_link(self) -> Option<E::WidgetPath> {
        match self {
            ResolveResultMut::Hit(_) => None,
            ResolveResultMut::Miss() => None,
            ResolveResultMut::Link(p) => Some(p),
        }
    }
    
    pub fn into_final(self) -> Option<Option<&'a mut E::DynWidget>> {
        match self {
            ResolveResultMut::Link(_) => None,
            ResolveResultMut::Hit(r) => Some(Some(r)),
            ResolveResultMut::Miss() => Some(None),
        }
    }

    pub fn is_final(&self) -> bool {
        match self {
            ResolveResultMut::Link(_) => false,
            ResolveResultMut::Hit(_) => true,
            ResolveResultMut::Miss() => true,
        }
    }
}