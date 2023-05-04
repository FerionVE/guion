use std::alloc::Layout;
use std::any::TypeId;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Add};
use std::ptr::NonNull;
use std::sync::Arc;

//pub mod test;

const _: () = {
    assert!(
        std::mem::size_of::<TypeId>() >= std::mem::align_of::<TypeId>()
        && std::mem::size_of::<TypeId>() >= 2
        && std::mem::size_of::<u8>() == 1 && std::mem::align_of::<u8>() == 1
    )
};

pub type NewPathStack<'a> = PathStack<'a>;

/// The PathStack stores an array of heterogenous types, which can later be queried from left-to-right in [`PathSliceRef`]
pub struct PathStackBase {
    start: NonNull<u8>,
    end: NonNull<u8>,
    base_layout: Layout,
}

pub struct PathStack<'a> {
    pos: NonNull<u8>,
    start: NonNull<u8>,
    end: NonNull<u8>,
    base: &'a mut PathStackBase,
    _p: PhantomData<&'a mut PathStackBase>,
}

/// The PathSlice allows decoding/matching a [`PathStack`]
#[derive(Clone, Copy)]
pub struct PathSliceRef<'a> {
    pos: NonNull<u8>,
    start: NonNull<u8>,
    end: NonNull<u8>,
    owner: Option<&'a Arc<PathSliceOwner>>,
    _p: PhantomData<&'a ()>,
}

/// The PathSlice allows decoding/matching a [`PathStack`]
#[derive(Clone, Copy)]
pub struct PathSliceRefFetched<'a> {
    slice: PathSliceRef<'a>,
    next: Option<TypeId>, // next prefetched (typeid and ptr right after id)
}

/// Owned PathSlice
#[derive(Clone)]
pub struct PathSliceOwned {
    pos: NonNull<u8>,
    start: NonNull<u8>,
    end: NonNull<u8>,
    owner: Arc<PathSliceOwner>,
}

pub struct PathSliceOwner {
    start: NonNull<u8>,
    base_layout: Layout,
}

impl PathStackBase {
    pub fn new_desktop() -> Self {
        Self::new(16 << 20, 4096)
    }

    /// `base_align` must be power of two.
    pub fn new(size: usize, base_align: usize) -> Self {
        let size = size.max(64);
        let align = typeid_layout().align().max(base_align);
        Self::from_base_layout(Layout::from_size_align(size, align).expect("Invalid layout"))
    }

    /// The align of `base_layout` must be larger or equal to [`TypeId`] align.
    pub fn from_base_layout(base_layout: Layout) -> Self {
        let start_ptr = unsafe { std::alloc::alloc(base_layout) };

        let Some(start) = NonNull::new(start_ptr) else {
            panic!("alloc failed");
        };

        let end_ptr = unsafe { start_ptr.add(base_layout.size()) };

        let end = unsafe { NonNull::new_unchecked(end_ptr) };

        Self {
            base_layout,
            start,
            end,
        }
    }

    pub fn path_stack<'s>(&'s mut self) -> PathStack<'s> where Self: 's {
        PathStack {
            pos: self.start,
            start: self.start,
            end: self.end,
            base: self,
            _p: PhantomData,
        }
    }

    // /// [`PathSliceRef::to_owned`] should be used to get a owned path slice.
    // pub fn clone_with_size_align(&self, size: usize, base_align: usize) -> Self {
    //     let mut new = Self::new(size, base_align);
    //     new.push_slice_impl(self.left_slice_ref());
    //     new
    // }

    // pub fn clone_with_current_size_align(&self) -> Self {
    //     let mut new = Self::from_base_layout(self.base_layout);

    //     unsafe {
    //         let len = self.pos.as_ptr().offset_from(self.start.as_ptr()) as usize;
    //         std::ptr::copy_nonoverlapping(self.start.as_ptr(), new.start.as_ptr(), len);
    //         new.pos = NonNull::new_unchecked( new.start.as_ptr().add(len) );
    //     }

    //     new
    // }

    // pub fn clear(mut self) -> Self {
    //     self.pos = self.start;
    //     self
    // }
}

impl PathStack<'_> {
    /// Push a path fragment onto the stack
    pub fn with<'s,T>(&'s mut self, value: T) -> PathStack<'s> where T: Copy + 'static, Self: 's {
        Self {
            pos: self.push_impl(value),
            start: self.start,
            end: self.end,
            base: unsafe{std::mem::transmute(&mut *self.base)},
            _p: PhantomData,
        }
    }

    #[deprecated]
    pub fn with_old<'s,T,R>(&'s mut self, value: T, inner: impl FnOnce(&mut PathStack<'_>)->R) -> R where T: Copy + 'static, Self: 's {
        let mut s = self.with(value);
        inner(&mut s)
    }

    pub fn fork<'s>(&'s mut self) -> PathStack<'s> where Self: 's {
        Self {
            pos: self.pos,
            start: self.start,
            end: self.end,
            base: unsafe{std::mem::transmute(&mut *self.base)},
            _p: PhantomData,
        }
    }

    fn push_impl<T>(&mut self, value: T) -> NonNull<u8> where T: Copy + 'static {
        unsafe {
            let (t_off, next_off) = t_offsets::<T>();
            
            let id_ptr = self.pos.as_ptr();

            if (self.end.as_ptr().offset_from(id_ptr) as usize) < next_off {
                panic!("PathStack overflow");
            }

            let t_ptr = id_ptr.add(t_off);

            std::ptr::write(id_ptr.cast::<TypeId>(), TypeId::of::<T>());
            std::ptr::write(t_ptr.cast::<T>(), value);

            NonNull::new_unchecked( id_ptr.add(next_off) )
        }
        //the PathStack needs to be sound at this point, before and after calling inner fn, and before and after this fn
    }

    pub fn push_slice<'s,R>(&'s mut self, slice: PathSliceRef<'_>) -> PathStack<'s> where Self: 's {
        Self {
            pos: self.push_slice_impl(slice),
            start: self.start,
            end: self.end,
            base: unsafe{std::mem::transmute(&mut *self.base)},
            _p: PhantomData,
        }
    }

    fn push_slice_impl(&mut self, slice: PathSliceRef<'_>) -> NonNull<u8> {
        unsafe {
            let old_pos = self.pos.as_ptr();
            let slice_pos = slice.pos.as_ptr();
            let len = slice.end.as_ptr().offset_from(slice_pos);

            if len > self.end.as_ptr().offset_from(old_pos) {
                panic!("PathStack overflow");
            }
            debug_assert!(len >= 0);
            debug_assert!(len as usize % typeid_layout().align() == 0);

            std::ptr::copy_nonoverlapping(slice_pos, old_pos, len as usize);

            NonNull::new_unchecked( old_pos.offset(len) )
        }
    }

    /// Get the left part of the stack as slice.
    pub fn left_slice<'s>(self: &&'s mut Self) -> PathSliceRef<'s> where Self: 's {
        PathSliceRef { 
            start: self.start,
            pos: self.start,
            end: self.pos,
            owner: None,
            _p: PhantomData,
        }
    }
    
    /// Get the left part of the stack as slice. Use [`left_slice`] on mutable references or [`left_slice`] inbetween [`with_pushed`].
    pub fn left_slice_ref<'s>(&'s self) -> PathSliceRef<'s> where Self: 's {
        PathSliceRef { 
            start: self.start,
            pos: self.start,
            end: self.pos,
            owner: None,
            _p: PhantomData,
        }
    }

    /// Get the left part of the stack as slice.
    pub fn left_slice2<'s>(&'s mut self) -> (PathSliceRef<'s>,&'s mut Self) where Self: 's {
        (
            self.left_slice(),
            self,
        )
    }

    // pub fn one_shot<R>(&mut self, inner: impl FnOnce(&mut PathStackOneShot)->R) -> R {
    //     let one_shot = unsafe { std::mem::transmute(self) };
    //     inner(one_shot)
    // }
}

impl<'s,'a,T> Add<T> for &'s mut PathStack<'a> where 'a: 's, T: Copy + 'static {
    type Output = PathStack<'s>;

    fn add(self, rhs: T) -> Self::Output {
        self.with(rhs)
    }
}

// /// PathStackOneShot is layout-transparent to PathStack
// #[repr(transparent)]
// pub struct PathStackOneShot(PathStack);

// impl PathStackOneShot {
//     pub fn push<T>(&mut self, value: T) where T: Copy + 'static {
//         self.push_impl(value)
//     }

//     pub fn push_slice(&mut self, slice: PathSliceRef<'_>) {
//         self.push_slice_impl(slice)
//     }

//     /// Get the left part of the stack as slice.
//     pub fn left_slice<'a>(self: &&'a mut Self) -> PathSliceRef<'a> {
//         PathSliceRef { 
//             start: self.start,
//             pos: self.start,
//             end: self.pos,
//             owner: None,
//             _p: PhantomData,
//         }
//     }
// }

// impl Deref for PathStackOneShot {
//     type Target = PathStack;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for PathStackOneShot {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl<'a> &'a mut PathStack {

// }

type MaxAlign = u128;

fn typeid_layout() -> Layout {
    Layout::new::<TypeId>().align_to(Layout::new::<MaxAlign>().align()).unwrap()
}

fn t_offsets<T>() -> (usize,usize) where T: Sized { 
    let id = typeid_layout();
    let t = Layout::new::<T>();
    if t.align() > id.align() {
        panic!("Encode type exceeds max align");
    }
    let (id_t,t_off) = id.extend(t).unwrap();
    let (_,next_off) = id_t.extend(id).unwrap();
    (t_off,next_off)
}

impl Drop for PathStackBase {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.start.as_ptr(), self.base_layout); }
    }
}

//TODO assert that PathStack is unwind-unsafe

/// SAFETY:
/// /// - end must be >= pos
unsafe fn prefetch_type_id(pos: NonNull<u8>, end: NonNull<u8>) -> Option<TypeId> {
    let pos = pos.as_ptr() as *const u8;
    let end = end.as_ptr() as *const u8;

    debug_assert!(end.offset_from(pos) >= 0);

    // debug_assert_eq!(
    //     (end.offset_from(pos) as usize) < align_offset(pos, id_layout.align()) + id_layout.size(),
    //     end.offset_from(pos) == 0
    // );

    if 
        end.offset_from(pos) == 0
        // (end.offset_from(pos) as usize) < align_offset(pos, id_layout.align()) + id_layout.size()
    {
        return None;
    }

    let type_id = *pos.cast::<TypeId>();

    Some(type_id)
}

impl<'a> PathSliceRef<'a> {
    /// This fetches the next TypeId and whether it's at the end. As the operation don't properly optimize yet, it should be used before doing multiple queries.
    pub fn fetch(self) -> PathSliceRefFetched<'a> {
        PathSliceRefFetched {
            next: unsafe { prefetch_type_id(self.pos, self.end) },
            slice: self,
        }
    }
    /// Clone the PathSlice into owned.
    pub fn to_owned(self) -> PathSliceOwned {
        if let Some(owner) = self.owner {
            return PathSliceOwned {
                pos: self.pos,
                start: self.start,
                end: self.end,
                owner: owner.clone(),
            };
        }

        let len = unsafe { self.end.as_ptr().offset_from(self.start.as_ptr()) as usize };

        let mut owned = PathSliceOwned::uninitialized(typeid_layout().align(), len);

        unsafe {
            owned.pos = NonNull::new_unchecked( owned.start.as_ptr().offset(self.pos.as_ptr().offset_from(self.start.as_ptr())) );
            std::ptr::copy_nonoverlapping(self.pos.as_ptr(), owned.start.as_ptr(), len);
        }

        owned
    }
    /// This moves to cursor to 0 and reveals the absolute path.
    pub fn absolute(self) -> PathSliceRef<'a> {
        PathSliceRef {
            start: self.start,
            pos: self.start,
            end: self.end,
            owner: self.owner,
            _p: PhantomData,
        }
    }
    /// This moves to cursor to 0 and reveals the absolute path until the previous cursor position.
    pub fn absolute_left_part(self) -> PathSliceRef<'a> {
        PathSliceRef {
            start: self.start,
            pos: self.start,
            end: self.pos,
            owner: self.owner,
            _p: PhantomData,
        }
    }
}

impl<'a> PathSliceRefFetched<'a> {
    /// Query whether the PathSlice has no remaining fragments on the right.
    pub fn is_empty(self) -> bool {
        self.next.is_none()
    }
    /// Query the next path fragment.
    /// 
    /// On Success it will return a reference to the &T and the remaining PathSlice behind this fragment (cursor moved to the right).
    pub fn slice_forward<T>(self) -> PathSliceMatch<'a,T> where T: Copy + Sized + 'static {
        let Some(type_id) = self.next else {return PathSliceMatch::End};

        let (t_off,end_off) = t_offsets::<T>();

        unsafe {
            let old_pos = self.slice.pos.as_ptr() as *const u8;
            let end = self.slice.end.as_ptr() as *const u8;

            // debug_assert!(
            //     {
            //         let id_layout = Layout::new::<TypeId>();
            //         add_align2(old_pos, id_layout.align()).add(id_layout.size())
            //     }.offset_from(id_end) == 0
            // );

            if type_id == TypeId::of::<T>() {
                //debug_assert!( (end.offset_from(id_end) as usize) >= align_offset(id_end, t_layout.align()) + t_layout.size() );
                let t_ptr = old_pos.add(t_off);
                // debug_assert!(t_end <= end);

                let value = &*t_ptr.cast::<T>();

                let new_pos = NonNull::new_unchecked(old_pos.add(end_off) as *mut u8);

                let new_slice = PathSliceRef {
                    start: self.slice.start,
                    pos: new_pos,
                    end: self.slice.end,
                    owner: self.slice.owner,
                    _p: PhantomData,
                };

                PathSliceMatch::Match(value, new_slice)
            } else {
                PathSliceMatch::Mismatch
            }
        }
    }
    /// Get TypeId of next path fragment.
    pub fn next_type_id(self) -> Option<TypeId> {
        self.next
    }
    /// Clone the PathSlice into owned.
    pub fn to_owned(self) -> PathSliceOwned {
        self.slice.to_owned()
    }
    /// This moves to cursor to 0 and reveals the absolute path.
    pub fn absolute(self) -> PathSliceRef<'a> {
        self.slice.absolute()
    }
    /// This moves to cursor to 0 and reveals the absolute path until the previous cursor position.
    pub fn absolute_left_part(self) -> PathSliceRef<'a> {
        self.slice.absolute_left_part()
    }
}

/// The result of a PathSlice fragment query
pub enum PathSliceMatch<'a,T> {
    /// The fragment matches the queried type.
    /// 
    /// The reference to T and the remaining PathSlice behind this fragment (cursor moved to the right).
    Match(&'a T, PathSliceRef<'a>),
    /// The fragment doesn't match the queried type.
    Mismatch,
    /// The are no more fragments to the right.
    End,
}

impl PathSliceOwned {
    fn uninitialized(max_align: usize, size: usize) -> Self {
        Self::uninitialized_from_base_layout( Layout::from_size_align(size, max_align).expect("TODO") )
    }

    fn uninitialized_from_base_layout(base_layout: Layout) -> Self {
        let start_ptr = unsafe { std::alloc::alloc(base_layout) };

        let Some(start) = NonNull::new(start_ptr) else {
            panic!("alloc failed");
        };

        let end_ptr = unsafe { start_ptr.add(base_layout.size()) };

        let end = unsafe { NonNull::new_unchecked(end_ptr) };

        Self {
            start,
            pos: start,
            owner: Arc::new(PathSliceOwner {
                start,
                base_layout,
            }),
            end,
        }
    }

    pub fn as_slice<'a>(&'a self) -> PathSliceRef<'a> {
        PathSliceRef { 
            start: self.start,
            pos: self.pos,
            end: self.end,
            owner: Some(&self.owner),
            _p: PhantomData,
        }
    }
}

// impl Clone for PathSliceOwned {
//     fn clone(&self) -> Self {
//         let len = unsafe { self.end.as_ptr().offset_from(self.start.as_ptr()) as usize };

//         let mut owned = PathSliceOwned::uninitialized_from_base_layout(self.base_layout);

//         unsafe {
//             owned.pos = NonNull::new_unchecked( owned.start.as_ptr().offset(self.pos.as_ptr().offset_from(self.start.as_ptr())) );
//             std::ptr::copy_nonoverlapping(self.start.as_ptr(), owned.start.as_ptr(), len);
//         }

//         owned
//     }
// }

impl Drop for PathSliceOwner {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.start.as_ptr(), self.base_layout); }
    }
}
