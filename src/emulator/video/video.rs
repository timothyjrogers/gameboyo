pub struct VideoController {
    pixels: [bool; 160*144],
    LCDC: u8,
    LY: u8,
    LYC: u8,
    STAT: u8,
}

impl VideoController {
    pub fn new() -> Self {
        Self {
            pixels: [false; 160*144],
            LCDC: 0,
            LY: 0,
            LYC: 0,
            STAT: 0b10000000,
        }
    }
}