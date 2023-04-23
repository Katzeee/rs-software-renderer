use crate::buffer_attachment::*;
use crate::line::*;
use crate::math::*;
use crate::shader::lerp;
pub struct Renderer {
    height: usize,
    width: usize,
    color_attachment: BufferAttachment<Vec3<u8>>,
    depth_attachment: BufferAttachment<f32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height: height,
            width: width,
            color_attachment: BufferAttachment::new(width, height, Vec3::new(0, 0, 0)),
            depth_attachment: BufferAttachment::new(width, height, f32::MAX),
        }
    }

    pub fn get_color_attachment(&self) -> &BufferAttachment<Vec3<u8>> {
        &self.color_attachment
    }

    pub fn draw_line(&mut self, line: &Line) {
        let mut x0 = (((line.start.position.x + 1.) / 2.) * self.width as f32) as i32;
        let mut y0 = (((line.start.position.y + 1.) / 2.) * self.height as f32) as i32;
        let x1 = (((line.end.position.x + 1.) / 2.) * self.width as f32) as i32;
        let y1 = (((line.end.position.y + 1.) / 2.) * self.height as f32) as i32;
        if x0 == x1 {
            for i in y0..y1 {
                self.color_attachment
                    .set(x0 as usize, i as usize, line.start.attrib.color.to_u8());
            }
        } else if y0 == y1 {
            for i in x0..x1 {
                let t: f32 = i as f32 / (x1 - x0) as f32;
                let color = lerp(line.start.attrib.color, line.end.attrib.color, t);
                self.color_attachment
                    .set(i as usize, y0 as usize, color.to_u8());
            }
        } else {
            let dx: i32 = (x1 - x0).abs();
            let dy: i32 = (y1 - y0).abs();
            let sx: i32 = if x0 < x1 { 1 } else { -1 };
            let sy: i32 = if y0 < y1 { 1 } else { -1 };
            let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
            let mut e2: i32;
            loop {
                if x0 == x1 || y0 == y1 {
                    break;
                }
                self.color_attachment
                    .set(x0 as usize, y0 as usize, line.start.attrib.color.to_u8());
                e2 = err;
                if e2 > -dx {
                    err -= dy;
                    x0 += sx;
                }
                if e2 < dy {
                    err += dx;
                    y0 += sy;
                }
            }
        }
        // DDA
        /*
        let mut k: f32 = ((line.end.position.y) - (line.start.position.y)) as f32
            / ((line.end.position.x) - (line.start.position.x)) as f32;
        dbg!(k);
        if k.abs() < 1.0 {
            let mut y0: f32 = line.start.position.y as f32;
            for i in line.start.position.x..line.end.position.x {
                *self.color_attachment.get(i, y0 as usize) = Vec3::new(255, 0, 0);
                y0 = y0 + k;
            }
        } else {
            let mut x0: f32 = line.start.position.x as f32;
            k = 1.0 / k;
            for i in line.start.position.y..line.end.position.y {
                *self.color_attachment.get(x0 as usize, i) = Vec3::new(255, 0, 0);
                x0 = x0 + k;
            }
        }
        */
    }

    pub fn clear(&mut self) {
        self.color_attachment.clear(Vec3::new(0, 0, 0));
    }
}
