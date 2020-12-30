use super::*;

pub trait Filter<E>: Clone + Default + Sized where E: Env, EEFilter<E>: From<Self> {
    fn _filter(&self, dest: &Link<E>, e: &EventCompound<E>) -> Option<EventCompound<E>>;
    fn attach_path_prefix(self, prefix: E::WidgetPath) -> Self;
}

/// 
#[derive(Clone)]
pub struct StdFilter<E> where E: Env, EEFilter<E>: From<Self> {
    /// An empty path means no filter
    pub filter_path: E::WidgetPath,
    /// Absolute Bounds filter
    /// Note: is only filtered if filter_path is empty
    pub filter_bounds: bool,
}

impl<E> Filter<E> for StdFilter<E> where E: Env, EEFilter<E>: From<Self> {
    fn _filter(&self, dest: &Link<E>, e: &EventCompound<E>) -> Option<EventCompound<E>> {
        if !self.filter_path.is_empty() {
            dest.widget.resolves_by(self.filter_path.index(0))
                .map(#[inline] || EventCompound{
                    filter: StdFilter{
                        filter_path: self.filter_path.slice(1..),
                        filter_bounds: self.filter_bounds,
                    }.into(),
                    ..e.clone()
                } )
        }else if self.filter_bounds {
            //eprintln!("{:?} in bounds {:?}: {}",e.0,e.1,e.0.in_bounds(&e.1));
            e.filter_bounds()
        }else{
            Some(e.clone())
        }
    }

    fn attach_path_prefix(mut self, prefix: E::WidgetPath) -> Self {
        self.filter_path = prefix.attached_path(&self.filter_path);
        self
    }
}

impl<E> Default for StdFilter<E> where E: Env, EEFilter<E>: From<Self> {
    fn default() -> Self {
        Self{
            filter_bounds: true,
            filter_path: E::WidgetPath::empty(),
        }
    }
}
