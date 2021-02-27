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

pub fn tabi<E>(mut root: Link<E>, path: E::WidgetPath, dir: TabulateDirection) -> Result<E::WidgetPath,GuionError<E>> where E: Env {
    let mut current = path.clone();
    let result = root._tabulate(TabulateOrigin::Resolve( path.strip_prefix(&root.path()).unwrap() ),dir)?;
    match result {
        TabulateResponse::Done(p) => current = p,
        TabulateResponse::Leave => {
            let result = root._tabulate(TabulateOrigin::Enter,dir)?;
            match result {
                TabulateResponse::Done(p) => current = p,
                TabulateResponse::Leave => {},
            }
        },
    }
    Ok(current)
}

/// in determining the next target inside a widget, this describes the origin
pub enum TabulateNextChildOrigin {
    /// widget entered (previous focused was outside)
    Enter,
    /// the previous focused was the widget itself
    This,
    /// previous focused was inside specific child
    Child(usize),
}

pub enum TabulateNextChildResponse {
    /// the tabulation target would be the widget itself
    This,
    /// the next tabulation target would be this child
    Child(usize),
    /// the tabulation would leave the widget
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
