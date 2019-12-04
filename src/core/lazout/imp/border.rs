use std::ops::AddAssign;
use std::ops::Add;
use qwutils::imp::option::OptionExt;
use super::*;

impl<R> AddAssign<R> for Size where R: AsRef<Border> {
    #[inline]
    fn add_assign(&mut self, r: R) {
        let r = r.as_ref();
        self.x += r.left+r.right;
        self.y += r.top+r.bottom;
    }
}
impl<R> Add<R> for Size where R: AsRef<Border> {
    type Output=Size;
    #[inline]
    fn add(mut self, r: R) -> Self::Output {
        self += r;
        self
    }
}
impl<'a,R> Add<R> for &'a Size where R: AsRef<Border> {
    type Output=Size;
    #[inline]
    fn add(self, r: R) -> Self::Output {
        self.clone() + r
    }
}

impl AddAssign<u32> for SizeAxis {
    #[inline]
    fn add_assign(&mut self, r: u32) {
        self.min += r;
        self.preferred += r;
        self.max.add_to(r);
    }
}
impl Add<u32> for SizeAxis {
    type Output=SizeAxis;
    #[inline]
    fn add(mut self, r: u32) -> Self::Output {
        self += r;
        self
    }
}
impl<'a> Add<u32> for &'a SizeAxis {
    type Output=SizeAxis;
    #[inline]
    fn add(self, r: u32) -> Self::Output {
        self.clone() + r
    }
}