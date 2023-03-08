use std::ops::{BitOrAssign, BitOr};

#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct Invalidation {
    pub render: bool,
    pub layout: bool,
}

impl Invalidation {
    pub fn new() -> Self {
        Self {
            render: true,
            layout: true,
        }
    }

    pub fn valid() -> Self {
        Self {
            render: false,
            layout: false,
        }
    }

    pub fn rerender(mut self) -> Self {
        self.render = true;
        self
    }

    pub fn relayout(mut self) -> Self {
        self.render = true;
        self.layout = true;
        self
    }
}

impl BitOrAssign<Self> for Invalidation {
    fn bitor_assign(&mut self, rhs: Self) {
        self.render |= rhs.render;
        self.layout |= rhs.layout;
        self.render |= self.layout;
    }
}

impl BitOr<Self> for Invalidation {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
