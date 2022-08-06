use crate::queron::Queron;
use crate::queron::query::Query;
use crate::root::RootRef;
use crate::widget::stack::{WithCurrentWidget, QueryCurrentWidget};

use super::*;

#[derive(Clone,Copy)]
pub enum TabulateDirection { //TODO trait
    Forward,
    Backward,
}

pub enum TabulateOrigin<E> where E: Env {
    Resolve(E::WidgetPath),
    Enter,
}

pub enum TabulateResponse<E> where E: Env {
    Done(E::WidgetPath),
    Leave,
}

pub fn tabi<E>(root_widget: &(impl Widget<E> + ?Sized), root_stack: &(impl Queron<E> + ?Sized), old_path: E::WidgetPath, dir: TabulateDirection, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<E::WidgetPath,E::Error> where E: Env { //TODO rename to tabulate_root
    let widget_data = QueryCurrentWidget.query_in(root_stack).unwrap();

    let mut current = old_path.clone();
    let result = root_widget._tabulate(&root_stack, TabulateOrigin::Resolve( old_path.strip_prefix(&widget_data.path).unwrap() ), dir, root.fork(), ctx)?;
    match result {
        TabulateResponse::Done(p) => current = p,
        TabulateResponse::Leave => {
            let result = root_widget._tabulate(&root_stack, TabulateOrigin::Enter, dir, root, ctx)?;
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
