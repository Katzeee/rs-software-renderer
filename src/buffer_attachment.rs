#[derive(Debug, Clone)]
pub struct BufferAttachment<T> {
    buffer: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> BufferAttachment<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            buffer: vec![value; width * height],
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.buffer[y * self.width + x] = value;
    }

    pub fn get(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.buffer[y * self.width + x]
    }

    pub fn buffer(&self) -> &Vec<T> {
        &self.buffer
    }

    pub fn clear(&mut self, value: T) {
        self.buffer.fill(value);
    }
}
