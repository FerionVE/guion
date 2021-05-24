use crate::*;
use super::*;

impl<S,E> Handler<E> for StdHandler<S,E> where
    S: Handler<E>,
    E: Env,
    E::Context: AsRefMut<Self> + CtxStdState<E> + 'static,
    EEvent<E>: StdVarSup<E>
{
    #[inline] 
    fn _render(l: Link<E>, r: &mut ERenderer<'_,E>) {
        S::_render(l,r)
        //todo!()
    }
    #[inline] 
    fn _event_direct(mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        if let Some(_) = e.event.is::<MouseMove>() {
            (l.as_mut() as &mut Self).s.mouse.hovered = Some(l.ident());
        }
        S::_event_direct(l,e)
        //todo!()
    }
    #[inline] 
    fn _send_event(l: Link<E>, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,E::Error> {
        /*if let Some(_) = e.0.is::<MouseMove>() {
            (l.as_mut() as &mut Self).s.mouse.hovered = Some(l.ident());
        }*/
        S::_send_event(l,e,child)
    }
    //#[inline] 
    fn _event_root(mut l: Link<E>, e: &EventCompound<E>) -> EventResp { //TODO BUG handle sudden invalidation of hovered widget
        assert!(l.path().is_empty());
        if let Some(ee) = e.event.is::<RootEvent<E>>() {
            let mut passed = false;
            match ee {
                RootEvent::KbdDown{key} => {
                    //Self::_event_root(l.reference(),(Event::from(RootEvent::KbdUp{key: key.clone()}),e.1,e.2));
                    if let Some(id) = (l.as_ref() as &Self).s.kbd.focused.clone() {
                        if !l.widget.stor.has_widget(id.refc().path) {
                            //drop event if widget is gone
                            return false;
                        }
                        (l.as_mut() as &mut Self).s.key.down(
                            key.clone(),
                            id.clone(),
                            e.ts,
                            None,
                        );
                        //emit KbdDown event
                        let event = KbdDown{
                            key: key.clone(),
                        };
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(event)),
                            id.refc().path
                        ).unwrap();
                        /*let event = KbdPress{
                            key: key.clone(),
                            down_widget: id.refc(),
                            down_ts: e.2,
                        };
                        l._event_root((Event::from(event),&wbounds,e.2));*/
                        passed |= l._event_root(&e.with_event(Event::from(RootEvent::KbdPress{key})));
                    }
                    //l._event_root((Event::from(RootEvent::KbdPress{key}),e.1,e.2));
                },
                RootEvent::KbdUp{key} => {
                    let old = (l.as_mut() as &mut Self).s.key.up(key);
                    if let Some(p) = old {
                        let event = KbdUp{
                            key: p.key,
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (l.as_ref() as &Self).s.kbd.focused.clone() {
                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(event.clone())),
                                id.path,
                            ).unwrap_or(false);
                        }
                        //drop up if widget is gone TODO check if this is correct
                        passed |=  l.send_event(
                            &e.default_filter().with_event(Event::from(event.clone())),
                            p.down.refc().path,
                        ).unwrap_or(false);
                    }
                },
                RootEvent::KbdPress{key} => {
                    let old = (l.as_mut() as &mut Self).s.key.get(key.clone());
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        let event = KbdPress{
                            key: p.key.clone(),
                            down_widget: p.down.refc(),
                            down_ts: p.ts,
                        };
                        if let Some(id) = (l.as_ref() as &Self).s.kbd.focused.clone() {
                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(event.clone())),
                                id.path.refc(),
                            ).unwrap_or(false);
                            let mut do_tab = false;
                            if let Ok(l) = l.with_widget(id.path.refc()) {
                                if key == EEKey::<E>::TAB && l.widget._tabulate_by_tab() {
                                    do_tab = true;
                                }
                            }
                            if do_tab {
                                let reverse = l.state().is_pressed(&[EEKey::<E>::SHIFT]).is_some();
                                let dir = if reverse {TabulateDirection::Backward} else {TabulateDirection::Forward};
                                let path = tabi(l.reference(),id.path,dir).expect("TODO");
                                //better way than this hack to get the ident
                                (l.as_mut() as &mut Self).s.kbd.focused = Some(WidgetIdent::from_path(path,l.widget.stor).expect("TODO"));
                            }
                        }
                    }
                }
                RootEvent::MouseDown{key} => {
                    passed |= Self::_event_root(l.reference(),&e.with_event(Event::from(RootEvent::MouseUp{key: key.clone()})));
                    //unfocus previously focused widget
                    passed |= Self::unfocus(l.reference(),e.bounds,e.ts);

                    //the currently hovered widget
                    if let Some(pos) = (l.as_ref() as &Self).s.mouse.pos {
                        if let Some(hovered) = (l.as_ref() as &Self).s.mouse.hovered.clone() {
                            (l.as_mut() as &mut Self).s.key.down(key.clone(),hovered.clone(),e.ts,Some(pos));

                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(MouseDown{key,pos})),
                                hovered.path.refc(),
                            ).unwrap_or(false);


                            //passed |= Self::focus(l,hovered.path.refc(),e.1,e.2).unwrap_or(false);

                            let focus = if let Ok(w) = l.with_widget(hovered.path.refc()) {
                                w.widget._focus_on_mouse_down()
                            }else{
                                false
                            };

                            if focus {
                                passed |= Self::focus(l,hovered.path,e.bounds,e.ts).unwrap();
                            }
                        }
                    }
                }
                RootEvent::MouseUp{key} => {
                    let old = (l.as_mut() as &mut Self).s.key.up(key);
                    //TODO send up event to the widget which downed it
                    if let Some(p) = old {
                        if let Some(pos) = (l.as_ref() as &Self).s.mouse.pos {
                            let event = MouseUp{
                                key: p.key,
                                pos,
                                down_pos: p.cursor.expect("TODO"), //fails if a invalid press was inserted into the state
                                down_widget: p.down.refc(),
                                down_ts: p.ts
                            };
                            if let Some(hovered) = (l.as_ref() as &Self).s.mouse.hovered.clone() {
                                if hovered != p.down {
                                    passed |= l.send_event(
                                        &e.default_filter().with_event(Event::from(event.clone())),
                                        hovered.path,
                                    ).unwrap_or(false);
                                }
                            }
                            //event dropped if widget gone
                            passed |= l.send_event(
                                &e.default_filter().with_event(Event::from(event)),
                                p.down.refc().path,
                            ).unwrap_or(false);
                        }
                    }
                }
                RootEvent::MouseMove{pos} => {
                    //set pos
                    (l.as_mut() as &mut Self).s.mouse.pos = Some(pos);
                    //previous hovered widget
                    if let Some(p) = (l.as_mut() as &mut Self).s.mouse.hovered.take() {
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseLeave{})),
                            p.path,
                        ).unwrap_or(false);
                    }
                    //hover state will be updated as the event passes through the widget tree
                    passed |= l._event_root(&e.with_event(Event::from(MouseMove{pos})));
                    if let Some(p) = (l.as_ref() as &Self).s.mouse.hovered.clone() {//TODO optimize clone
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseEnter{})),
                            p.path,
                        ).unwrap_or(false);
                    }
                    
                }
                RootEvent::MouseLeaveWindow{} => {
                    if let Some(p) = (l.as_ref() as &Self).s.mouse.hovered.clone() {//TODO optimize clone
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseLeave{})),
                            p.path,
                        ).unwrap_or(false);
                    }
                    let mouse = &mut (l.as_mut() as &mut Self).s.mouse;
                    mouse.pos = None;
                    mouse.hovered = None;
                }
                RootEvent::WindowMove{pos,size} => {
                    passed |= l._event_root(&e.with_event(Event::from(WindowMove{pos,size})))
                }
                RootEvent::WindowResize{size} => {
                    passed |= l._event_root(&e.with_event(Event::from(WindowResize{size})))
                }
                RootEvent::TextInput{text} => {
                    if let Some(id) = (l.as_ref() as &Self).s.kbd.focused.clone() {
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(TextInput{text})),
                            id.path,
                        ).unwrap_or(false);
                    }
                }
                RootEvent::MouseScroll{x,y} => {
                    if let Some(hovered) = (l.as_ref() as &Self).s.mouse.hovered.clone() {
                        passed |= l.send_event(
                            &e.default_filter().with_event(Event::from(MouseScroll{x,y})),
                            hovered.path,
                        ).unwrap_or(false);
                    }
                }
            }
            passed
        }else{
            S::_event_root(l,e)
        }
    }
    #[inline] 
    fn _size(l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        //todo!();
        S::_size(l,e)
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
