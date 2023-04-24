use std::sync::Arc;

use crate::{buffer_attachment::*, line::*, math::*, shader::*, triangle::*};
pub struct Renderer {
    height: i32,
    width: i32,
    color_attachment: BufferAttachment<Vec3<u8>>,
    depth_attachment: BufferAttachment<f32>,
    pub should_draw_bound_box: bool,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            height: height,
            width: width,
            color_attachment: BufferAttachment::new(width, height, Vec3::new(0, 0, 0)),
            depth_attachment: BufferAttachment::new(width, height, f32::MAX),
            should_draw_bound_box: true,
        }
    }

    pub fn get_color_attachment(&self) -> &BufferAttachment<Vec3<u8>> {
        &self.color_attachment
    }

    pub fn draw_line(&mut self, line: &Line) {
        let (mut x0, mut y0) =
            self.ndc_to_screen_space(line.start.position.x, line.start.position.y);
        let (x1, y1) = self.ndc_to_screen_space(line.end.position.x, line.end.position.y);
        let dx: i32 = (x1 - x0).abs();
        let dy: i32 = (y1 - y0).abs();
        if x0 == x1 {
            for i in std::cmp::min(y0, y1)..std::cmp::max(y0, y1) {
                let t: f32 = i as f32 / dy as f32;
                let color = lerp(line.start.attrib.color, line.end.attrib.color, t);
                self.color_attachment.set(x0, i, color.to_u8());
            }
        } else if y0 == y1 {
            for i in std::cmp::min(x0, x1)..std::cmp::max(x0, x1) {
                let t: f32 = i as f32 / dx as f32;
                let color = lerp(line.start.attrib.color, line.end.attrib.color, t);
                self.color_attachment.set(i, y0, color.to_u8());
            }
        } else {
            let sx: i32 = if x0 < x1 { 1 } else { -1 };
            let sy: i32 = if y0 < y1 { 1 } else { -1 };
            let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
            let mut t = 0.0f32;
            let dt = 1.0 / (if dx > dy { dx } else { dy }) as f32;
            let mut e2: i32;
            loop {
                if x0 == x1 || y0 == y1 {
                    break;
                }
                let color = lerp(line.start.attrib.color, line.end.attrib.color, t).to_u8();
                t += dt;
                self.color_attachment.set(x0, y0, color);
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

    pub fn draw_triangle(&mut self, triangle: Triangle) {
        // self.draw_line(&Line::new(triangle[0], triangle[1]));
        // self.draw_line(&Line::new(triangle[0], triangle[2]));
        // self.draw_line(&Line::new(triangle[2], triangle[1]));

        // compute the screen space coord
        let (x0, y0) = self.ndc_to_screen_space(triangle[0].position.x, triangle[0].position.y);
        let (x1, y1) = self.ndc_to_screen_space(triangle[1].position.x, triangle[1].position.y);
        let (x2, y2) = self.ndc_to_screen_space(triangle[2].position.x, triangle[2].position.y);

        // compute aabb box
        let left = std::cmp::min(std::cmp::min(x0, x1), x2);
        let right = std::cmp::max(std::cmp::max(x0, x1), x2);
        let bottom = std::cmp::min(std::cmp::min(y0, y1), y2);
        let top = std::cmp::max(std::cmp::max(y0, y1), y2);

        // draw bbox
        if self.should_draw_bound_box {
            let red = *Attributes::new().set_color(Vec3::new(255., 0., 0.));
            let left_bottom = self.screen_to_ndc_space(left, bottom);
            let left_top = self.screen_to_ndc_space(left, top);
            let right_top = self.screen_to_ndc_space(right, top);
            let right_bottom = self.screen_to_ndc_space(right, bottom);
            self.draw_line(&Line::new(
                *Vertex::new(Vec3::new(left_bottom.0, left_bottom.1, 0.)).set_attr(red),
                *Vertex::new(Vec3::new(left_top.0, left_top.1, 0.)).set_attr(red),
            ));
            self.draw_line(&Line::new(
                *Vertex::new(Vec3::new(left_bottom.0, left_bottom.1, 0.)).set_attr(red),
                *Vertex::new(Vec3::new(right_bottom.0, right_bottom.1, 0.)).set_attr(red),
            ));
            self.draw_line(&Line::new(
                *Vertex::new(Vec3::new(left_top.0, left_top.1, 0.)).set_attr(red),
                *Vertex::new(Vec3::new(right_top.0, right_top.1, 0.)).set_attr(red),
            ));
            self.draw_line(&Line::new(
                *Vertex::new(Vec3::new(right_top.0, right_top.1, 0.)).set_attr(red),
                *Vertex::new(Vec3::new(right_bottom.0, right_bottom.1, 0.)).set_attr(red),
            ));
        }

        // draw triangle
        for x in left..right {
            for y in bottom..top {
                if self.in_triangle(
                    &[
                        Vec3::new(x0, y0, 0),
                        Vec3::new(x1, y1, 0),
                        Vec3::new(x2, y2, 0),
                    ],
                    Vec3::new(x, y, 0),
                ) {
                    self.color_attachment.set(x, y, Vec3::new(255, 0, 0));
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.color_attachment.clear(Vec3::new(0, 0, 0));
    }

    fn ndc_to_screen_space(&self, x: f32, y: f32) -> (i32, i32) {
        let x = (((x + 1.) / 2.) * (self.width - 1) as f32) as i32;
        let y = (((-y + 1.) / 2.) * (self.height - 1) as f32) as i32;
        (x, y)
    }

    fn screen_to_ndc_space(&self, x: i32, y: i32) -> (f32, f32) {
        let x = 2. * (x as f32) / self.width as f32 - 1.;
        let y = 1. - 2. * (y as f32) / self.height as f32;
        (x, y)
    }

    fn in_triangle(&self, v: &[Vec3<i32>; 3], p: Vec3<i32>) -> bool {
        let e1 = v[1] - v[0];
        let e2 = v[2] - v[1];
        let e3 = v[0] - v[2];
        // let p = Vec3::new(x, y, 0);
        (p - v[0]).cross(e1).z > 0 && (p - v[1]).cross(e2).z > 0 && (p - v[2]).cross(e3).z > 0
    }

    // fn barycentric() ->
    // fn interpolate
}
