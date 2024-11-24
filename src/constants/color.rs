// Colors
pub enum Color {
    Black,
    DarkGray,
    LightGray,
    White,
}

impl Color {
    /// Returns the `u32` representation of the color
    pub fn to_u32(&self) -> u32 {
        match self {
            Color::Black => 0x000000,
            Color::DarkGray => 0x444444,
            Color::LightGray => 0xAAAAAA,
            Color::White => 0xFFFFFF,
        }
    }
}
