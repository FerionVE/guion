use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use super::*;
use event::variants::*;

impl<S,E> Handler<E> for StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self> + 'static, EEvent<E>: StdVarSup<E> {
    #[inline] 
    fn _render(l: Link<E>, r: &mut RenderLink<E>) -> bool {
        S::_render(l,r)
        //todo!()
    }
    #[inline] 
    fn _event(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        if let Some(_) = e.0.is::<MouseMove>() {
            l.as_mut().s.mouse.hovered = Some(l.widget.path.clone());
        }
        S::_event(l,e);
        //todo!()
    }
    #[inline] 
    fn _event_root(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //todo!()
        if let Some(ee) = e.0.is::<RootEvent<E>>() {
            match ee {
                RootEvent::KbdDown{key} => {
                    Self::_event_root(l.reference(),(Event::from(RootEvent::KbdUp{key: key.clone()}),e.1,e.2));
                    if let Some(id) = l.as_ref().s.kbd.focused.clone() {
                        if !l.widget.stor.has_widget(id.slice()) {
                            //drop event if widget is gone
                            return;
                        }
                        l.as_mut().s.kbd.down(
                            key.clone(),
                            id.clone(),
                            e.2,
                        );
                        //emit KbdDown event
                        let mut l = l.with_widget(id.slice()).unwrap();
                        let wbounds = l.trace_bounds(e.1,false);
                        let event = KbdDown{
                            key,
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));
                    }
                },
                RootEvent::KbdUp{key} => {
                    let old = l.as_mut().s.kbd.up(key);
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        //drop up if widget is gone TODO check if this is correct
                        if let Ok(mut l) = l.with_widget(p.id.slice()) {
                            let wbounds = l.trace_bounds(e.1,false);
                            let event = KbdUp{
                                key: p.key,
                                down_widget: p.id.clone(),
                                down_ts: p.ts,
                            };
                            l._event_root((Event::from(event),&wbounds,e.2));
                        }
                    }
                },
                RootEvent::MouseDown{key} => {
                    Self::_event_root(l.reference(),(Event::from(RootEvent::MouseUp{key: key.clone()}),e.1,e.2));
                    //unfocus previously focused widget
                    Self::unfocus(l.ctx,l.widget.stor,e.1,e.2);

                    //the currently hovered widget
                    let pos = l.as_ref().s.mouse.pos.expect("TODO");
                    let hovered = l.as_ref().s.mouse.hovered.clone().expect("TODO");

                    l.as_mut().s.mouse.down(key.clone(),pos,hovered.clone(),e.2);

                    if let Ok(mut w) = l.with_widget(hovered.slice()) {
                        let wbounds = w.trace_bounds(e.1,false);
                        //focus the hovered if it should by mouse down
                        if w.widget()._focus_on_mouse_down() {
                            Self::focus(w.reference(),e.2,e.1,&wbounds);
                        }

                        w._event_root((Event::from(MouseDown{key,pos}),&wbounds,e.2))
                    };
                }
                RootEvent::MouseUp{key} => {
                    let old = l.as_mut().s.mouse.up(key);
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        let pos = l.as_ref().s.mouse.pos.expect("TODO");
                        if let Ok(mut l) = l.with_widget(p.id.slice()) {
                            //event dropped if widget gone
                            let wbounds = l.trace_bounds(e.1,false);
                            let event = MouseUp{
                                key: p.key,
                                pos: pos,
                                down_pos: p.start,
                                down_widget: p.id.clone(),
                                down_ts: p.ts
                            };
                            l._event_root((Event::from(event),&wbounds,e.2));
                        }
                    }
                }
                RootEvent::MouseMove{dest} => {
                    //set pos
                    l.as_mut().s.mouse.pos = Some(dest);
                    //previous hovered widget
                    let previous = l.as_mut().s.mouse.hovered.take();
                    //hover state will be updated as the event passes through the widget tree
                    l._event_root((Event::from(MouseMove{dest}),e.1,e.2));
                    if let Some(p) = l.as_ref().s.mouse.hovered.clone() {//TODO optimize clone
                        let mut l = l.with_widget(p.slice())
                            .expect("Lost Widget");
                        let wbounds = l.trace_bounds(e.1,false);
                        l._event_root((Event::from(MouseEnter{dest}),&wbounds,e.2));
                    }
                    if let Some(p) = previous {
                        let mut l = l.with_widget(p.slice())
                            .expect("Lost Widget");
                        let wbounds = l.trace_bounds(e.1,false);
                        l._event_root((Event::from(MouseLeave{dest}),&wbounds,e.2));
                    }
                }
                _ => {}
            }
        }else{
            S::_event_root(l,e);
        }
    }
    #[inline] 
    fn _size(l: Link<E>) -> ESize<E> {
        //todo!();
        S::_size(l)
    }
}

/*impl<S,E> AsHandler<Self,E> for StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: Context<E,Handler=Self> {
    fn as_mut(c: &mut E::Context) -> &mut Self {
        c._handler_mut()
    }
    fn as_ref(c: &E::Context) -> &Self {
        c._handler()
    }
}

impl<S,E> AsHandler<S,E> for StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: Context<E,Handler=Self> {
    fn as_mut(c: &mut E::Context) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &E::Context) -> &S {
        &c._handler().sup
    }
}*/

/*impl<S,C,T> AsHandler<S,C> for T where S: Handler<C>, C: Context, C::Handler: AsHandler<StdHandler<S,C>,C> + 'static {
    fn as_mut(c: &mut C) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &C) -> &S {
        &c._handler().sup
    }
}*/