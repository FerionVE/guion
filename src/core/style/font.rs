pub trait Font {
    type PP: PreprocessedString;

    fn len_in_pixels(&self, s: &str) -> u32;

    fn preprocess(&self, s: &str) -> Self::PP;
}

pub trait PreprocessedString {
    type C: PreprocessedChar;

    fn chars(&self) -> [Self::C];
    fn back(&self) -> String;
}

pub trait PreprocessedChar {
    fn offset(&self) -> u32;
    fn char(&self) -> char;
}