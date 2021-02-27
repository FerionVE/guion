use super::*;

impl<'w,E,T> Widget<E> for Pane<'w,E,T> where
    E: Env,
    T: WidgetArray<E>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        self._render_impl(l,r)
    }
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        self._event_direct_impl(l,e)
    }
    fn _size(&self, l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        self._size_impl(l,e)
    }
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        self.child_bounds_impl(l,b,e,force)
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        self.childs.childs()
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        self.childs.into_childs()
    }

    fn focusable(&self) -> bool {
        false
    }

    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.childs.child(i)
    }
    fn into_child<'a>(self: Box<Self>, i: usize) -> Result<Resolvable<'a,E>,()> where Self: 'a {
        self.childs.into_child(i)
    }

    fn childs_mut(&mut self) -> Vec<Resolvable<E>> {
        self.childs.childs_mut()
    }
    fn child_mut(&mut self, i: usize) -> Result<Resolvable<E>,()> {
        self.childs.child_mut(i)
    }
    fn mutate(&mut self) -> Result<&mut dyn WidgetMut<E>,GuionError<E>> {
        Ok(self)
    }
}
impl<'w,E,T> WidgetMut<E> for Pane<'w,E,T> where 
    E: Env,
    T: WidgetArray<E>+'w,
{
    fn _set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
}

impl<'w,E,T> Pane<'w,E,T> where
    E: Env,
    T: WidgetArray<E>,
{
    pub fn _render_impl(&self, mut l: Link<E>, r: &mut RenderLink<E>) where
        E: Env,
    {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        let sizes = l.child_sizes(r.style()).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&r.bounds().size,&sizes,self.orientation); 

        for i in 0..self.childs() {
            let l = l.for_child(i).expect("Dead Path Inside Pane");
            let mut r = r.slice(&bounds[i]);
            r.render_widget(l);
        }
        //TODO FIX viewport
    }

    pub fn _event_direct_impl(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp where
        E: Env,
    {
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));
        
        let sizes = l.child_sizes(&e.style).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&e.bounds.size,&sizes,self.orientation);

        let mut passed = false;

        for i in 0..self.childs() {
            let mut l = l.for_child(i).expect("Dead Path Inside Pane");
            let sliced = e.slice_bounds(&bounds[i]);
            if let Some(ee) = sliced.filter(&l) {
                passed |= l.event_direct(&ee);
            }
        }

        passed
    }

    pub fn _size_impl(&self, mut l: Link<E>, e: &EStyle<E>) -> ESize<E> where
        E: Env,
    {
        let e = e.and(&self.style);
        let mut s = ESize::<E>::empty();
        l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(&e), self.orientation) ).expect("Dead Path inside Pane");
        s
    }

    pub fn child_bounds_impl(&self, mut l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> where
        E: Env,
    {
        let e = e.and(&self.style);
        let sizes = l.child_sizes(&e).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&b.size,&sizes,self.orientation); 

        Ok(bounds)
    }
}
