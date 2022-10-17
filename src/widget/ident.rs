//! WidgetIdent is combined [`WidgetID`] and [`Path`](WidgetPath), and can be found in event to make it comparable by the ID and resolvable by the Path

// /// WidgetIdent is combined [`WidgetID`] and [`Path`](WidgetPath), and can be found in event to make it comparable by the ID and resolvable by the Path
// #[derive(Clone)]
// pub struct WidgetIdent<E> where E: Env {
//     pub id: E::WidgetID,
//     pub path: E::WidgetPath,
// }

// impl<E> WidgetIdent<E> where E: Env {
//     #[inline]
//     pub fn is(&self, other_widget: E::WidgetID) -> bool {
//         self.id == other_widget //TODO AsID trait
//     }
//     #[deprecated="this resolves the widget"]
//     /// Resolves the Widget
//     #[inline]
//     pub fn from_path(path: E::WidgetPath, stor: &E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Self,E::Error> {
//         if let Some(id) = path._dest_widget() {
//             Ok(Self{id,path})
//         }else{
//             stor.with_widget(
//                 path.clone(),
//                 #[inline] |widget,_|
//                     widget.map(#[inline] |widget| Self{id: widget.id(), path: path.clone()} ), //TODO with_widget resolve should also yield new path
//                 ctx,
//             )
//         }
//     }
// }

// impl<E> PartialEq for WidgetIdent<E> where E: Env {
//     #[inline]
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

// impl<E> AsRef<E::WidgetPath> for WidgetIdent<E> where E: Env {
//     #[inline]
//     fn as_ref(&self) -> &E::WidgetPath {
//         &self.path
//     }
// }

// impl<E> Deref for WidgetIdent<E> where E: Env {
//     type Target = E::WidgetPath;
//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.path
//     }
// }

// impl<E> RefClonable for WidgetIdent<E> where E: Env {
//     #[inline]
//     fn refc(&self) -> Self {
//         Self {
//             id: self.id.clone(),
//             path: self.path.clone(),
//         }
//     }
// }

// impl<E> Debug for WidgetIdent<E> where E: Env {
//     #[inline]
//     fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
//         Ok(())
//     }
// }
