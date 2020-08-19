use std::ops::Index;
use super::*;

mod border;
mod size_axis;

impl Index<Orientation> for Size {
    type Output = SizeAxis;
    #[inline]
    fn index(&self, i: Orientation) -> &Self::Output {
        match i {
            Orientation::Horizontal => &self.x,
            Orientation::Vertical => &self.y,
        }
    }
}

/*impl Index<Orientation> for Weight {
    type Output = WeightAxis;
    #[inline]
    fn index(&self, i: Orientation) -> &Self::Output {
        match i {
            Orientation::Horizontal => &self.x,
            Orientation::Vertical => &self.y,
        }
    }
}*/

impl AsRef<Self> for SizeAxis {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

/*qwutils::asref_op!(;;SizeAxis;Add=add;AddAssign=add_assign;r: {
    self.min += r.min;
    self.preferred += r.preferred;
    self.max.add_to_lossy(r.max);
});*/
