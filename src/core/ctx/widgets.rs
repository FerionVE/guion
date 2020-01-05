use super::*;

pub trait Widgets<E>: 'static where E: Env {
    fn resolve(&self, i: WPSlice<E>) -> ResolveResult<E>;
    fn resolve_mut(&mut self, i: WPSlice<E>) -> ResolveResultMut<E>;
    #[inline]
    fn widget(&self, i: WPSlice<E>) -> Option<&E::DynWidget> {
        let mut r = self.resolve(i);
        loop {
            match r {
                ResolveResult::Link(p) => r = self.resolve(p.slice()),
                ResolveResult::Hit(r) => return Some(r),
                ResolveResult::Miss() => return None,
            }
        }
    }
    #[inline]
    fn widget_mut<'a>(&'a mut self, i: WPSlice<E>) -> Option<&'a mut E::DynWidget> {
        //let mut r = Some(self.resolve_mut(i));
        /*while let ResolveResultMut::Link(p) = r {
            r = self.resolve_mut(p.slice());
        }
        unimplemented!()*/
        /*loop {
            if r.as_ref().unwrap().is_final() {
                return r.take().unwrap().into_final().unwrap();
            }
            if let Some(p) = r.take().unwrap().into_link() {
                r = None;
                r = Some(self.resolve_mut(p.slice()));
            }
        }*/
        /*match self.resolve_mut(i) {
            ResolveResultMut::Link(p) => self.widget_mut(p.slice()),
            ResolveResultMut::Final(r) => r,
        }*/
        let mut final_path = i.unslice(); //TODO
        loop {
            match self.resolve_mut(final_path.slice()) {
                ResolveResultMut::Link(p) => final_path = p,
                ResolveResultMut::Hit(_) => break,
                ResolveResultMut::Miss() => return None,
            }
        }
        self.resolve_mut(final_path.slice()).into_final().unwrap()
    }

    #[inline]
    fn has_widget(&self, i: WPSlice<E>) -> bool {
        self.widget(i).is_some()
    }

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}
}

pub fn resolve_in_root<'a,E: Env>(w: &'a E::DynWidget, p: WPSlice<E>) -> ResolveResult<'a,E> {
    unimplemented!()
}

pub fn resolve_in_root_mut<'a,E: Env>(w: &'a mut E::DynWidget, p: WPSlice<E>) -> ResolveResultMut<'a,E> {
    unimplemented!()
}