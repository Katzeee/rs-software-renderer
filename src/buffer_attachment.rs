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

    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            buffer: vec![value; width * height],
            width: width,
            height: height,
        }
    }

    pub fn clear(&mut self, value: T) {
        self.buffer.fill(value);
    }
}

// impl BufferAttachment<f32> {
//     pub fn new(width: usize, height: usize) -> Self {
//         Self {
//             buffer: vec![f32::MAX; width * height],
//             width: width,
//             height: height,
//         }
//     }
// }

// impl BufferAttachment<math::Vec3> {
//     pub fn new(width: usize, height: usize) -> Self {
//         Self { buffer: vec![] }
//     }
// }
