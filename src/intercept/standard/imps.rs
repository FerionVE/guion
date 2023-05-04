use crate::aliases::EEvent;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::intercept::InterceptBuilder;
use crate::newpath::PathResolvusDyn;
use crate::pathslice::PathSliceRef;
use crate::state::StdState;
use crate::state::standard::key::StdPressedKey;
use crate::util::bounds::Offset;
use crate::widget::id::WidgetID;

use super::StdIntercept;

impl<S,E> StdState<E> for StdIntercept<S,E> where
    S: InterceptBuilder<E>,
    E: Env,
    EEvent<E>: StdVarSup<E>
{
    type K = StdPressedKey<E>;
    #[inline]
    fn hovered(&self) -> Option<(PathSliceRef,WidgetID)> { //TODO eventually WidgetIdent return in trait
        self.state.mouse.hovered.as_ref().map(|(a,v)| (a.as_slice(),*v) )
    }
    #[inline]
    fn selected(&self) -> Option<(PathSliceRef,WidgetID)> {
        self.state.kbd.focused.as_ref().map(|(a,v)| (a.as_slice(),*v) )
    }
    #[inline]
    fn pressed(&self) -> &[Self::K] {
        &self.state.key.pressed[..]
    }
    #[inline]
    fn cursor_pos(&self) -> Option<Offset> {
        self.state.mouse.pos
    }
    
}

// impl<S,E> DynState<E> for StdHandler<S,E> where
//     S: HandlerBuilder<E>,
//     E: Env,
//     E::WidgetID: Eq + Hash,
//     EEvent<E>: StdVarSup<E>
// {
//     fn remote_state_or_default<T>(&self, i: E::WidgetID) -> T where T: Default + Clone + 'static {
//         self.state.remote_states
//             .get(&(i,TypeId::of::<T>()))
//             .map_or_else(T::default,#[inline] |v|
//                 v
//                 .downcast_ref::<T>()
//                 .unwrap()
//                 .clone()
//             )
//     }
//     fn push_remote_state<T>(&mut self, i: E::WidgetID, v: T) where T: 'static {
//         self.state.remote_states
//             .insert((i,TypeId::of::<T>()),Box::new(v)); //TODO do not realloc always
//     }
// }
