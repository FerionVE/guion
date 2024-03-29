use std::sync::Arc;

use crate::env::Env;
use crate::newpath::{PathResolvusDyn, PathStack};
use crate::queron::Queron;
use crate::root::RootRef;
use crate::widget::Widget;

#[derive(Clone,Copy)]
pub enum TabulateDirection { //TODO trait
    Forward,
    Backward,
}

#[derive(Clone)]
pub enum TabulateOrigin<'a,E> where E: Env {
    Resolve(&'a (dyn PathResolvusDyn<E>+'a)),
    Enter,
}

#[derive(Clone)]
pub enum TabulateResponse<E> where E: Env {
    Done(Arc<dyn PathResolvusDyn<E>>),
    Leave,
}

pub fn tabi<E>(root_widget: &(impl Widget<E> + ?Sized), root_path: &(impl PathStack<E> + ?Sized), root_stack: &(impl Queron<E> + ?Sized), old_path: Arc<dyn PathResolvusDyn<E>>, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Arc<dyn PathResolvusDyn<E>>,E::Error> where E: Env { //TODO rename to tabulate_root
    assert!(root_path.inner().is_none());
    let mut current: Arc<dyn PathResolvusDyn<E>> = old_path;
    let result = root_widget._tabulate(root_path, root_stack, TabulateOrigin::Resolve( &*current /*TODO strip_prefix*/ ), dir, root.fork(), ctx)?;
    match result {
        TabulateResponse::Done(p) => current = p,
        TabulateResponse::Leave => {
            let result = root_widget._tabulate(root_path, root_stack, TabulateOrigin::Enter, dir, root, ctx)?;
            match result {
                TabulateResponse::Done(p) => current = p,
                TabulateResponse::Leave => {},
            }
        },
    }
    Ok(current)
}

/// In determining the next target inside a [`Widget`], this describes the origin
pub enum TabulateNextChildOrigin {
    /// Widget entered (previous focused was outside)
    Enter,
    /// The previous focused was the widget itself
    This,
    /// Previous focused was inside specific child
    Child(usize),
}

pub enum TabulateNextChildResponse {
    /// The tabulation target would be the widget itself
    This,
    /// The next tabulation target would be this child
    Child(usize),
    /// The tabulation would leave the widget
    Leave,
}

impl TabulateNextChildOrigin {
    pub fn child_or_this(c: Option<usize>) -> Self {
        match c {
            Some(v) => Self::Child(v),
            None => Self::This,
        }
    } 
}
