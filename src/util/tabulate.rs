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

pub fn tabi<E>(mut root: Link<E>, sub_path: E::WidgetPath, dir: TabulateDirection) -> Result<E::WidgetPath,GuionError<E>> where E: Env {
    let mut current = sub_path.clone(); //TODO sub_path to absolute path WidgetPath::strip_prefix
    let result = root._tabulate(TabulateOrigin::Resolve(sub_path),dir)?;
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
