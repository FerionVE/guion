use super::*;

pub trait Routing<E>: Sized where E: Env {
    fn prepare(&mut self, w: &Link<'_,E>, e: &EventCompound);
    fn filter(&self, i: usize, w: &Link<'_,E>, e: &EventCompound) -> Option<Self>;
}

pub enum StdRouting<E> where E: Env {
    Path{path: E::WidgetPath},
    BoundFilter{cache: Option<Vec<Bounds>>},
}

impl<E> Routing<E> for StdRouting<E> where E: Env {
    fn prepare(&mut self, w: &Link<'_,E>, e: &EventCompound) -> Result<(),()> {
        match self {
            Self::Path{..} => {},
            Self::BoundFilter{cache} => {
                *cache = Some(w.child_bounds(e.1,e.3)?);
            },
        }
        Ok(())
    }
    fn filter(&self, i: usize, w: &Link<'_,E>, e: &EventCompound) -> Result<Option<(Self,EventCompound)>,()> {
        match self {
            Self::Path { path } => {
                if path.is_empty() {return Ok(None);}
                Ok(w.widget.child(i).unwrap().resolves_by(path.index(0))
                    .map(|| (Self::Path{path: path.slice(1..)},e.clone()) ))
            }
            Self::BoundFilter { cache } => {
                Ok(e.0.filter(cache.as_ref().unwrap()[i])
                    .map(|ee| (Self::BoundFilter{cache: None},(ee,e.1,e.2,e.3)) ))
            }
        }
    }
}