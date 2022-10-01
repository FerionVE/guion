// pub trait Query<E>: Clone + 'static where E: 'static {
//     type Out<'b>: 'b;
//     type Builder<'b>: Clone + 'b;

//     fn query_in<'b,S>(&self, stack: &'b S) -> Self::Out<'b> where S: Queron<E> + ?Sized + 'b {
//         self._query_direct(stack)
//     }

//     fn _query_direct<'b,S>(&self, stack: &'b S) -> Self::Out<'b> where S: Queron<E> + ?Sized + 'b {
//         let mut builder = self.new_builder();
//         let qstack = QueryStack::new(self, &mut builder);
//         stack._query(qstack);
//         self.end_builder(builder)
//     }

//     fn new_builder<'b>(&self) -> Self::Builder<'b>;
//     fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Self::Out<'b>; // newstyle neeeds without Option but with default
// }

use super::*;

pub mod stdprops;
pub mod variants;

pub mod size;
