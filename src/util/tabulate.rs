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
