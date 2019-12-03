use std::ops::Index;
use super::*;

impl Index<Orientation> for Size {
    type Output = SizeAxis;

    fn index(&self, i: Orientation) -> &Self::Output {
        match i {
            Orientation::Horizontal() => &self.x,
            Orientation::Vertical() => &self.y,
        }
    }
}

impl Index<Orientation> for Weight {
    type Output = WeightAxis;

    fn index(&self, i: Orientation) -> &Self::Output {
        match i {
            Orientation::Horizontal() => &self.x,
            Orientation::Vertical() => &self.y,
        }
    }
}
