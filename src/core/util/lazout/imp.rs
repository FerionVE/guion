impl Index<Orientation> for Size {
    type Output = LazoutDir;

    fn index(&self, i: Orientation) -> &Self::Output {
        match i {
            Orientation::Horizontal() => &self.x,
            Orientation::Vertical() => &self.y,
        }
    }
}

