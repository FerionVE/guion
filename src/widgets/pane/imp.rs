use super::*;

impl<E,T> Widget<E> for T where T: IPane<E> + 'static, E: Env + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        IPane::id(self)
    }
    #[inline]
    fn _handler(&self) -> HandlerFns<E> {
        HandlerFns{
            render: render::<T,E>,
            event: event::<T,E>,
            size: size::<T,E>,
        }
    }
    #[inline]
    fn invalid(&self) -> bool {
        IPane::invalid(self)
    }
    fn set_invalid(&mut self, v: bool) {
        IPane::set_invalid(self,v)
    }
    #[inline]
    fn parent(&self) -> Option<&E::WidgetID> {
        IPane::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        IPane::set_parent(self,v)
    }
    #[inline]
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a> {
        Box::new(
            IPane::childs(self)
            .iter()
            .cloned()
        )
    }
    
    #[inline] fn as_any(&self) -> &dyn Any {self}
    #[inline] fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: E::Renderer) {
    let o = l.me::<W>().orientation();

    let c = childs::<W,E>(&l);

    let b = c.iter()
        .map(|c| 
            l.widget(c)
            .expect("Lost Widget")
            .handler()
            .size(&mut l)
        )
        .collect::<Vec<_>>();

    let b = calc_bounds(r.bounds_abs().size, &b[..], o);

    for (cc,bb) in c.iter().zip(b.iter()) {
        l.widget(cc)
            .expect("Pane contains lost Widget")
            .handler()
            .render( &mut *l, r.slice(bb) );
    }

}

fn event<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

fn size<W: IPane<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    let o = l.me::<W>().orientation();

    let mut s = Size::empty();
    
    for c in childs::<W,E>(&l) {
        let cs = l.widget(&c)
            .expect("Lost Widget")
            .handler()
            .size(&mut l);
        
        s.add(&cs, o)
    }

    s
}
#[inline]
fn childs<W: IPane<E> + 'static, E: Env + 'static>(l: &Link<E>) -> Vec<E::WidgetID> {
    l.me::<W>().childs().to_owned()
}