#[derive(Debug, Clone)]
pub struct BufferAttachment<T> {
    buffer: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> BufferAttachment<T>
where
    T: Clone,
{
    pub fn new(width: i32, height: i32, value: T) -> Self {
        Self {
            buffer: vec![value; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        self.buffer[(y * self.width + x) as usize] = value;
    }

    pub fn get(&mut self, x: i32, y: i32) -> &mut T {
        &mut self.buffer[(y * self.width + x) as usize]
    }

    pub fn buffer(&self) -> &Vec<T> {
        &self.buffer
    }

    pub fn clear(&mut self, value: T) {
        self.buffer.fill(value);
    }
}
