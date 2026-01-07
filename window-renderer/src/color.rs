pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const WHITE: &'static Color = &Color{ r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF };
pub const BLACK: &'static Color = &Color{ r: 0x00, g: 0x00, b: 0x00, a: 0xFF };
pub const RED: &'static Color = &Color{ r: 0xFF, g: 0x00, b: 0x00, a: 0xFF };
pub const BLUE: &'static Color = &Color{ r: 0x00, g: 0x00, b: 0xFF, a: 0xFF };
pub const GREEN:  &'static Color = &Color{ r: 0x00, g: 0xFF, b: 0x0, a: 0xFF };
