use std::convert::Infallible;
use std::intrinsics::copy_nonoverlapping;
use std::io::{Cursor, Write};
use std::panic;

pub trait Stor {
    type Err;

    fn write<B>(&mut self, b: B) -> Result<(),Self::Err> where B: AsRef<[u8]>;
}

impl Stor for Vec<u8> {
    type Err = Infallible;

    #[inline]
    fn write<B>(&mut self, b: B) -> Result<(),Self::Err> where B: AsRef<[u8]> {
        self.extend_from_slice(b.as_ref());
        Ok(())
    }
}

impl Stor for (Box<[u8]>,usize) {
    type Err = ();

    #[inline]
    fn write<B>(&mut self, b: B) -> Result<(),Self::Err> where B: AsRef<[u8]> {
        let b = b.as_ref();

        if self.1 + b.len() > self.0.len() {
            return Err(())
        }

        unsafe {
            copy_nonoverlapping(b.as_ptr(), self.0.as_mut_ptr().add(self.1), b.len());
        }

        self.1 += b.len();

        Ok(())
    }
}
