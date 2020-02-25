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
    fn _event(l: Link<E>, e: EEvent<E>) {
        S::_event(l,e);
        //todo!()
    }
    #[inline] 
    fn _event_root(mut l: Link<E>, e: EEvent<E>) {
        //todo!()
        if let Some(ee) = e.is::<RootEvent<E>>() {
            match ee {
                RootEvent::KbdDown{key,ts} => {
                    let senf: &mut Self = l.as_mut();
                    let id = senf.s.kbd.focused.clone().expect("TODO");
                    senf.s.kbd.down(
                        key,
                        id,
                        ts,
                    );
                },
                RootEvent::KbdUp{key,ts} => {
                    l.as_mut().s.kbd.up(key);
                },
                RootEvent::MouseDown{key, root_bounds,ts} => {
                    if let Some(p) = l.as_ref().s.kbd.focused.clone() {
                        if let Ok(w) = l.with_widget(p.slice()) {
                            if w.widget()._focus_on_mouse_down() {

                            }
                        }
                    }
                    let pos = l.as_ref().s.mouse.pos.expect("TODO");
                    S::_event_root(l,Event::from(MouseDown{key,pos,current_bounds: root_bounds,ts}))
                }
                RootEvent::MouseMove{dest,root_bounds,ts} => {
                    let previous = l.as_mut().s.mouse.hovered.take();
                    S::_event_root(
                        l.reference(),
                        Event::from(MouseMove{dest,current_bounds: root_bounds,ts})
                    );
                    if let Some(p) = l.as_ref().s.mouse.hovered.clone() {//TODO optimize clone
                        l.with_widget(p.slice())
                            .expect("Lost Widget")
                            ._event_root(
                                Event::from(MouseEnter{dest,ts})
                            );
                    }
                    if let Some(p) = previous {
                        l.with_widget(p.slice())
                            .expect("Lost Widget")
                            ._event_root(
                                Event::from(MouseLeave{dest,ts})
                            );
                    }
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