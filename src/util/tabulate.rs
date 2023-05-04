use std::sync::Arc;

use crate::env::Env;
use crate::newpath::{PathResolvusDyn, PathStack};
use crate::pathslice::{PathSliceOwned, NewPathStack, PathSliceRef};
use crate::queron::Queron;
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::id::WidgetID;

#[derive(Clone,Copy)]
pub enum TabulateDirection { //TODO trait
    Forward,
    Backward,
}

#[derive(Clone)]
pub enum TabulateOrigin<'a> {
    Resolve(PathSliceRef<'a>),
    Enter,
}

#[derive(Clone)]
pub enum TabulateResponse {
    Done((PathSliceOwned,WidgetID)),
    Leave,
}

pub fn tabi<E>(root_widget: &(impl Widget<E> + ?Sized), root_path: &mut NewPathStack, root_stack: &(impl Queron<E> + ?Sized), old_path: (PathSliceOwned,WidgetID), dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<(PathSliceOwned,WidgetID),E::Error> where E: Env { //TODO rename to tabulate_root
    assert!(root_path.left_slice().fetch().is_empty());
    let mut current: (PathSliceOwned,WidgetID) = old_path;
    let result = root_widget._tabulate(root_path, root_stack.erase(), TabulateOrigin::Resolve( current.0.as_slice() /*TODO strip_prefix*/ ), dir, root.fork(), ctx)?;
    match result {
        TabulateResponse::Done(p) => current = p,
        TabulateResponse::Leave => {
            let result = root_widget._tabulate(root_path, root_stack.erase(), TabulateOrigin::Enter, dir, root, ctx)?;
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
    Child(isize),
}

pub enum TabulateNextChildResponse {
    /// The tabulation target would be the widget itself
    This,
    /// The next tabulation target would be this child
    Child(isize),
    /// The tabulation would leave the widget
    Leave,
}

impl TabulateNextChildOrigin {
    pub fn child_or_this(c: Option<isize>) -> Self {
        match c {
            Some(v) => Self::Child(v),
            None => Self::This,
        }
    } 
}
