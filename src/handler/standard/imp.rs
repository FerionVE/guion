use crate::util::bounds::Bounds;
use crate::*;
use super::*;
use event::variants::*;
use state::standard::kbd::tabulate::tabulate;

impl<S,E> Handler<E> for StdHandler<S,E> where
    S: Handler<E>,
    E: Env,
    E::Context: AsRefMut<Self> + AsHandlerStateful<E> + 'static,
    EEvent<E>: StdVarSup<E>
{
    #[inline] 
    fn _render(l: Link<E>, r: &mut RenderLink<E>) {
        S::_render(l,r)
        //todo!()
    }
    #[inline] 
    fn _event(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        if let Some(_) = e.0.is::<MouseMove>() {
            l.as_mut().s.mouse.hovered = Some(l.ident());
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
                    //Self::_event_root(l.reference(),(Event::from(RootEvent::KbdUp{key: key.clone()}),e.1,e.2));
                    if let Some(id) = l.as_ref().s.kbd.focused.clone() {
                        if !l.widget.stor.has_widget(id.refc().path) {
                            //drop event if widget is gone
                            return;
                        }
                        l.as_mut().s.key.down(
                            key.clone(),
                            id.clone(),
                            e.2,
                            None,
                        );
                        //emit KbdDown event
                        let mut l = l.with_widget(id.refc().path).unwrap();
                        let wbounds = l.trace_bounds(e.1,false);
                        let event = KbdDown{
                            key: key.clone(),
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));
                        /*let event = KbdPress{
                            key: key.clone(),
                            down_widget: id.refc(),
                            down_ts: e.2,
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));*/
                        l._event_root((Event::from(RootEvent::KbdPress{key}),e.1,e.2));
                    }
                    //l._event_root((Event::from(RootEvent::KbdPress{key}),e.1,e.2));
                },
                RootEvent::KbdUp{key} => {
                    let old = l.as_mut().s.key.up(key);
                    if let Some(p) = old {
                        let event = KbdUp{
                            key: p.key,
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = l.as_ref().s.kbd.focused.clone() {
                            if let Ok(mut l) = l.with_widget(id.path) {
                                let wbounds = l.trace_bounds(e.1,false);
                                l._event_root((Event::from(event.clone()),&wbounds,e.2));
                            }
                        }
                        //drop up if widget is gone TODO check if this is correct
                        if let Ok(mut l) = l.with_widget(p.down.refc().path) {
                            let wbounds = l.trace_bounds(e.1,false);
                            l._event_root((Event::from(event),&wbounds,e.2));
                        }
                    }
                },
                RootEvent::KbdPress{key} => {
                    let old = l.as_mut().s.key.get(key.clone());
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        let event = KbdPress{
                            key: p.key.clone(),
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = l.as_ref().s.kbd.focused.clone() {
                            if let Ok(mut l) = l.with_widget(id.path.refc()) {
                                let wbounds = l.trace_bounds(e.1,false);
                                l._event_root((Event::from(event.clone()),&wbounds,e.2));

                                if key == EEKey::<E>::TAB && l.widget._tabulate_by_tab() {
                                    let reverse = l.state().is_pressed(&[EEKey::<E>::SHIFT]).is_some();
                                    let path = tabulate::<E>(l.widget.stor,id.path,reverse);
                                    //better way than this hack to get the ident
                                    l.as_mut().s.kbd.focused = Some(WidgetIdent::from_path(path,l.widget.stor).expect("TODO"));
                                }
                            }
                        }
                    }
                }
                RootEvent::MouseDown{key} => {
                    Self::_event_root(l.reference(),(Event::from(RootEvent::MouseUp{key: key.clone()}),e.1,e.2));
                    //unfocus previously focused widget
                    Self::unfocus(l.ctx,l.widget.stor,e.1,e.2);

                    //the currently hovered widget
                    let pos = l.as_ref().s.mouse.pos.expect("TODO");
                    let hovered = l.as_ref().s.mouse.hovered.clone().expect("TODO");

                    l.as_mut().s.key.down(key.clone(),hovered.clone(),e.2,Some(pos));

                    if let Ok(mut w) = l.with_widget(hovered.path) {
                        let wbounds = w.trace_bounds(e.1,false);
                        //focus the hovered if it should by mouse down
                        if w.widget._focus_on_mouse_down() {
                            Self::focus(w.reference(),e.2,e.1,&wbounds);
                        }

                        w._event_root((Event::from(MouseDown{key,pos}),&wbounds,e.2))
                    };
                }
                RootEvent::MouseUp{key} => {
                    let old = l.as_mut().s.key.up(key);
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        if let Some(pos) = l.as_ref().s.mouse.pos {
                            let hovered = l.as_ref().s.mouse.hovered.clone().unwrap();
                            let event = MouseUp{
                                key: p.key,
                                pos: pos,
                                down_pos: p.cursor.expect("TODO"), //fails if a invalid press was inserted into the state
                                down_widget: p.down.refc(),
                                down_ts: p.ts
                            };
                            if hovered != p.down {
                                if let Ok(mut l) = l.with_widget(hovered.path) {
                                    let wbounds = l.trace_bounds(e.1,false);
                                    l._event_root((Event::from(event.clone()),&wbounds,e.2));
                                }
                            }
                            if let Ok(mut l) = l.with_widget(p.down.refc().path) {
                                //event dropped if widget gone
                                let wbounds = l.trace_bounds(e.1,false);
                                l._event_root((Event::from(event),&wbounds,e.2));
                            }
                        }
                    }
                }
                RootEvent::MouseMove{pos} => {
                    //set pos
                    l.as_mut().s.mouse.pos = Some(pos);
                    //previous hovered widget
                    if let Some(p) = l.as_mut().s.mouse.hovered.take() {
                        let mut l = l.with_widget(p.path)
                            .expect("Lost Widget");
                        let wbounds = l.trace_bounds(e.1,false);
                        l._event_root((Event::from(MouseLeave{}),&wbounds,e.2));
                    }
                    //hover state will be updated as the event passes through the widget tree
                    l._event_root((Event::from(MouseMove{pos}),e.1,e.2));
                    if let Some(p) = l.as_ref().s.mouse.hovered.clone() {//TODO optimize clone
                        let mut l = l.with_widget(p.path)
                            .expect("Lost Widget");
                        let wbounds = l.trace_bounds(e.1,false);
                        l._event_root((Event::from(MouseEnter{}),&wbounds,e.2));
                    }
                    
                }
                RootEvent::MouseLeaveWindow{} => {
                    if let Some(p) = l.as_ref().s.mouse.hovered.clone() {//TODO optimize clone
                        let mut l = l.with_widget(p.path)
                            .expect("Lost Widget");
                        let wbounds = l.trace_bounds(e.1,false);
                        l._event_root((Event::from(MouseLeave{}),&wbounds,e.2));
                    }
                    let mouse = &mut l.as_mut().s.mouse;
                    mouse.pos = None;
                    mouse.hovered = None;
                }
                RootEvent::WindowMove{pos,size} => {
                    l._event_root((Event::from(WindowMove{pos,size}),&e.1,e.2))
                }
                RootEvent::WindowResize{size} => {
                    l._event_root((Event::from(WindowResize{size}),&e.1,e.2))
                }
                RootEvent::TextInput{text} => {
                    if let Some(id) = l.as_ref().s.kbd.focused.clone() {
                        if let Ok(mut l) = l.with_widget(id.path) {
                            let wbounds = l.trace_bounds(e.1,false);
                            l._event_root((Event::from(TextInput{text}),&wbounds,e.2));
                        }
                    }
                }
                RootEvent::MouseScroll{x,y} => {
                    let hovered = l.as_ref().s.mouse.hovered.clone().expect("TODO");
                    
                    if let Ok(mut w) = l.with_widget(hovered.path) {
                        let wbounds = w.trace_bounds(e.1,false);
                        w._event_root((Event::from(MouseScroll{x,y}),&wbounds,e.2))
                    };
                }
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