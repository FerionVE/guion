use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::AddAssign;
use std::ops::Add;
use qwutils::imp::option::OptionExt;
use super::*;

impl<R> AddAssign<R> for SizeAxis where R: AsRef<SizeAxis> {
    #[inline]
    fn add_assign(&mut self, r: R) {
        let r = r.as_ref();
        self.min += r.min;
        self.preferred += r.preferred;
        self.max.add_to_lossy(r.max);
    }
}
impl<R> Add<R> for SizeAxis where R: AsRef<SizeAxis> {
    type Output=SizeAxis;
    #[inline]
    fn add(mut self, r: R) -> Self::Output {
        self += r;
        self
    }
}
impl<'a,R> Add<R> for &'a SizeAxis where R: AsRef<SizeAxis> {
    type Output=SizeAxis;
    #[inline]
    fn add(self, r: R) -> Self::Output {
        self.clone() + r
    }
}

impl<R> BitAndAssign<R> for SizeAxis where R: AsRef<SizeAxis> {
    #[inline]
    fn bitand_assign(&mut self, r: R) {
        let o = r.as_ref();

        self.min = self.min.max(o.min);
        self.preferred = self.preferred.max(o.preferred);
        self.max.with_mut_if_saturating( o.max, #[inline] |s,o| *s = (*s).min(*o) );

        self.max.map(#[inline] |m| self.preferred = self.preferred.min(m) );
        self.preferred = self.preferred.max(self.min);
    }
}
impl<R> BitAnd<R> for SizeAxis where R: AsRef<SizeAxis> {
    type Output=SizeAxis;
    #[inline]
    fn bitand(mut self, r: R) -> Self::Output {
        self &= r;
        self
    }
}
impl<'a,R> BitAnd<R> for &'a SizeAxis where R: AsRef<SizeAxis> {
    type Output=SizeAxis;
    #[inline]
    fn bitand(self, r: R) -> Self::Output {
        self.clone() & r
    }
}