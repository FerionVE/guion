use crate::core::ctx::aliases::*;
use crate::core::*;
use crate::core::ctx::widgets::Widgets;
use event::variants::LostFocus;

pub mod tabulate;

pub struct KbdState<E> where E: Env {
    pub pressed: Vec<KbdPressedKey<E>>,
    pub focused: Option<E::WidgetPath>,
}

pub struct KbdPressedKey<E> where E: Env {
    pub key: EEKey<E>,
    ///the widget which was selected (focused) where the key press started
    pub id: E::WidgetPath,
    ///the time the key press started
    pub ts: u64,
}

impl<E> KbdState<E> where E: Env {
    #[inline]
    pub fn down(&mut self, key: EEKey<E>, id: E::WidgetPath, ts: u64) -> Option<KbdPressedKey<E>> {
        let old = self.up(key.clone());
        self.pressed.push(
            KbdPressedKey{
                key,
                id,
                ts,
            }
        );
        old
    }
    #[inline]
    pub fn up(&mut self, key: EEKey<E>) -> Option<KbdPressedKey<E>> {
        //self.pressed.retain(#[inline] |e| e.key != key );
        for (i,k) in self.pressed.iter().enumerate() {
            if k.key == key {
                return Some(self.pressed.remove(i));
            }
        }
        None
    }
    #[inline]
    pub fn get(&self, key: EEKey<E>) -> Option<&KbdPressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: EEKey<E>) -> Option<&mut KbdPressedKey<E>> {
        self.pressed.iter_mut().find(#[inline] |i| i.key == key )
    }
    /*#[inline]
    pub fn unfocus(ctx: &mut E::Context, deref_to_self: impl Fn(&mut E::Context) -> &mut Self, root: &E::Storage, root_bounds: &Bounds, ts: u64) 
        where EEvent<E>: StdVarSup<E>,
    {
        if let Some(p) = deref_to_self(ctx).focused.take() {
            if let Ok(w) = root.widget(p.slice()) {
                let bounds = root.trace_bounds(p.slice()).unwrap();
                ctx.link(w)._event_root((Event::from(LostFocus{}),&bounds,ts));
            }
        }
    }
    #[inline]
    pub fn focus(&mut self, l: &mut Link<E>, bounds: &Bounds, ts: u64) {
        
    }*/

    pub fn new() -> Self {
        Self{
            pressed: Vec::new(),
            focused: None,
        }
    }
}

impl<E> PressedKey<E> for KbdPressedKey<E> where E: Env {
    fn key(&self) -> &EEKey<E> {
        &self.key
    }
    fn widget(&self) -> &E::WidgetID {
        self.id.id()
    }
    fn timestamp(&self) -> u64 {
        self.ts
    }
}