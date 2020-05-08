use super::*;
use std::{ops::Deref, borrow::Borrow};

/// WidgetIdent is combined WidgetID and Path, and can be found in event to make it comparable by the ID and resolvable by the Path
#[derive(Clone)]
pub struct WidgetIdent<E> where E: Env {
    pub id: E::WidgetID,
    pub path: E::WidgetPath,
}

impl<E> WidgetIdent<E> where E: Env {
    pub fn is(&self, w: E::WidgetID) -> bool {
        self.id == w //TODO AsID trait
    }
}

impl<E> PartialEq for WidgetIdent<E> where E: Env {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<E> AsRef<E::WidgetPath> for WidgetIdent<E> where E: Env {
    fn as_ref(&self) -> &E::WidgetPath {
        &self.path
    }
}

impl<E> Deref for WidgetIdent<E> where E: Env {
    type Target = E::WidgetPath;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<E> RefClonable for WidgetIdent<E> where E: Env {
    fn refc(&self) -> Self {
        Self {
            id: self.id.clone(),
            path: self.path.refc(),
        }
    }
}
