use std::default::Default;
use std::f32::consts::PI;

use crate::math::*;

pub struct Camera {
    near: f32,
    far: f32,
    fovy: f32,
    aspect: f32,
    pos: Vec3<f32>,
    target: Vec3<f32>,
    up: Vec3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 50.0,
            fovy: 45.0,
            aspect: 1.,
            pos: Vec3::new(0., 0., 5.),
            target: Vec3::from(0.),
            up: Vec3::new(0., 1., 0.),
        }
    }
}

impl Camera {
    #[rustfmt::skip]
    pub fn get_projection_matrix(&self) -> Mat4<f32> {
        let fovy_radians = PI * self.fovy / 180.;
        Mat4::from_array(&[
            1. / (self.aspect * (fovy_radians / 2.).tan()), 0., 0., 0.,
            0., 1. / (fovy_radians / 2.).tan(), 0., 0.,
            0., 0., - (self.far + self.near) / (self.far - self.near), - 2. * self.near * self.far / (self.far - self.near),
            0., 0., -1., 0.,
        ])
    }

    #[rustfmt::skip]
    pub fn get_view_matrix(&self) -> Mat4<f32> {
        let z = (self.pos - self.target).normalize();
        let x = self.up.cross(z).normalize();
        let y = z.cross(x).normalize();
        Mat4::new([
            Vec4::from_vec3(x, 0.), 
            Vec4::from_vec3(y, 0.), 
            Vec4::from_vec3(z, 0.), 
            Vec4::new(0., 0., 0., 1.)
        ]).transpose() * Mat4::translate(-self.pos.x, -self.pos.y, -self.pos.z)
    }
}
