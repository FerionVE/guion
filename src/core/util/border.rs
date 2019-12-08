pub struct Border {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Border {
    #[inline]
    pub fn new(left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self{
            left,
            right,
            top,
            bottom,
        }
    }
    #[inline]
    pub fn empty() -> Self {
        Self::new(0,0,0,0)
    }
    #[inline]
    pub fn width(&self) -> u32 {
        self.left + self.right
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.top + self.bottom
    }
}