use std::ops::Index;
use super::*;

mod border;
mod size_axis;

impl Index<Orientation> for StdGonstraints {
    type Output = StdGonstraintAxis;
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

impl AsRef<Self> for StdGonstraintAxis {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
