
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
pub struct Display {
    pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]
}

impl Display {
    pub fn new() -> Display {
        Display { pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT] }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = !self.pixels[y][x]
    }

    pub fn clear(&mut self, v: bool) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                self.pixels[y][x] = v;
            }
        }
    }
}