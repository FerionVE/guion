use super::*;
use util::state::*;
use crate::event::key::Key; //TODO fix req of this import

impl<'w,L,R,V,E> Widget<'w,E> for SplitPane<'w,L,R,V,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    L: AsWidget<'w,E>+Statize+'w, L::Statur: Sized,
    R: AsWidget<'w,E>+Statize+'w, R::Statur: Sized,
    V: AtomState<f32>+Statize+'w, V::Statur: Sized,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) {
        let bounds = self.calc_bounds(&r.b,self.state.get()); 

        {
            if l.state().is_hovered(&self.id) {
                r.set_cursor(match self.orientation {
                    Orientation::Horizontal => StdCursor::SizeWE,
                    Orientation::Vertical => StdCursor::SizeNS,
                }.into());
            }
            r.slice_abs(&bounds[1])
                .with(&[StdVerb::ObjForeground])
                .fill_rect();
        }

        {
            let left = l.for_child(0).expect("Dead Path inside Pane");
            let mut r = r.slice_abs(&bounds[0]);
            r.render_widget(left);
        }
        {
            let right = l.for_child(1).expect("Dead Path inside Pane");
            let mut r = r.slice_abs(&bounds[2]);
            r.render_widget(right);
        }
        {
            //TODO render center
        }
    }
    fn _event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        let o = self.orientation;
        let mut bounds = self.calc_bounds(e.1,self.state.get()); 

        {
            let sliced = e.1.slice(&bounds[1]);
            if let Some(ee) = e.0.filter_cloned(&sliced) {
                //event hits the center
                //TODO receive mousemove here and let update f32
                //mousemove should also be sent to mousedown initiating widget with flag receive_outer_mousemove or such
                //get x/y of curser
                //subtract min constraints of left/right widget from left/right of widget's bounds (including self.handle_width/2)
                //limit (min/max) curser x to the shrinked bounds
                //into relative f32
                //enqueue mutate setting the AtomStateMut
            }

            if let Some(mm) = e.0.is_mouse_move() {
                //if mouse is down and was pressed on us
                if let Some(_) = l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],self.id.clone()) {
                    let cursor = l.state().cursor_pos().expect("TODO");
                    let mut cx = cursor.par(o);
                    let (mut wx0, ww) = e.1.par(o);
                    let mut wx1 = wx0 + ww as i32;

                    let l_min = l.for_child(0)
                        .expect("Dead Path inside Pane").size()
                        .as_std().par(o).min;
                    let r_min = l.for_child(1)
                        .expect("Dead Path inside Pane").size()
                        .as_std().par(o).min;

                    wx0 += (self.width/2) as i32;
                    wx1 -= (self.width/2) as i32;

                    let ewx0 = wx0 - l_min as i32;
                    let ewx1 = wx1 - r_min as i32;

                    cx = cx.min(ewx1-1).max(ewx0);
                    
                    if ewx1 > ewx0 {
                        let ww = wx1 - wx0;
                        cx = cx - wx0;
                        let fcx = (cx as f32)/(ww as f32);

                        l.mutate_closure(Box::new(move |mut w,_,_|{
                            let w = w.traitcast_mut::<dyn AtomStateMut<f32>>().unwrap();
                            w.set(fcx);
                        }));

                        bounds = self.calc_bounds(e.1,fcx);
                    }
                }
            }
        }
        {
            let mut left = l.for_child(0).expect("Dead Path inside Pane");
            let sliced = e.1 & &bounds[0];
            if let Some(ee) = e.0.filter_cloned(&sliced) {
                left.event((ee,&sliced,e.2));
            }
        }
        {
            let mut right = l.for_child(1).expect("Dead Path inside Pane");
            let sliced = e.1 & &bounds[2];
            if let Some(ee) = e.0.filter_cloned(&sliced) {
                right.event((ee,&sliced,e.2));
            }
        }
    }
    fn _size(&self, mut l: Link<E>) -> ESize<E> {
        let mut s = ESize::<E>::empty();
        l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(), self.orientation) ).expect("Dead Path inside Pane");
        s.add_space(self.width,self.orientation);
        s
    }
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        Ok(self.calc_bounds(b,self.state.get()))
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        self.childs.childs()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        self.childs.into_childs()
    }

    fn focusable(&self) -> bool {
        false
    }

    fn border(&self, b: &mut Border) {
        *b = Border::empty();
    }

    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        self.childs.child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        self.childs.into_child(i)
    }
}
impl<'w,L,R,V,E> WidgetMut<'w,E> for SplitPane<'w,L,R,V,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    L: AsWidgetMut<'w,E>+Statize+'w, L::Statur: Sized,
    R: AsWidgetMut<'w,E>+Statize+'w, R::Statur: Sized,
    V: AtomStateMut<f32>+Statize+'w, V::Statur: Sized,
{
    fn _set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        self.childs.childs_mut()
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        self.childs.into_childs_mut()
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        self.childs.child_mut(i)
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        self.childs.into_child_mut(i)
    }

    impl_traitcast!(
        dyn AtomState<f32> => |s| &s.state;
        dyn AtomStateMut<f32> => |s| &s.state;
    );
    impl_traitcast_mut!(
        dyn AtomState<f32> => |s| &mut s.state;
        dyn AtomStateMut<f32> => |s| &mut s.state;
    );
}

impl<'w,L,R,V,E> SplitPane<'w,L,R,V,E> where
    E: Env,
    V: AtomState<f32>+Statize+Sized+'w, V::Statur: Sized,
{
    fn calc_bounds(&self, b: &Bounds, v: f32) -> Vec<Bounds> {
        let handle_width = self.width.min(b.w());
        let o = self.orientation;
        let (x,w) = b.par(o);
        let (y,h) = b.unpar(o);
        let w0 = ((w as f32 - handle_width as f32)*v.max(0.0).min(1.0)) as u32;
        let w2 = w - w0 - handle_width;
        let x1 = x + w0 as i32;
        let x2 = x1 + handle_width as i32;
        let left = Bounds::from_ori(x, y, w0, h, o);
        let center = Bounds::from_ori(x1, y, handle_width, h, o);
        let right = Bounds::from_ori(x2, y, w2, h, o);
        vec![left,center,right]
    }
}