
pub struct Framebuffer {
    width: i32,
    height: i32,
    buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Framebuffer {
        assert!(width != 0 && height != 0);

        Framebuffer{
            width,
            height,
            buffer: vec![0u32; (width * height) as usize],
        }
    }

    pub fn write(&mut self, row: i32, col: i32, color: u32) {
        assert!(row < self.height);
        assert!(col < self.width);

        let pixel = row * self.width + col;
        self.buffer[pixel as usize] = color;
    }

    pub fn buffer(&self) -> &Vec<u32> { &self.buffer }
}
