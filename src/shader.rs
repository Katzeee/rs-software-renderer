use std::f32::consts::PI;
use std::ops::{Add, Mul};

use crate::math::*;

#[derive(Clone, Copy)]
pub struct Attributes {
    pub color: Vec3<f32>, // cannot use u8 because cannot lerp u8
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_color(&mut self, color: Vec3<f32>) -> &mut Self {
        self.color = color;
        self
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            color: Vec3::from(255.),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3<f32>,
    pub attrib: Attributes,
}

impl Vertex {
    pub fn new(position: Vec3<f32>) -> Self {
        Self {
            position: position,
            attrib: Attributes::default(),
        }
    }
    pub fn set_attr(&mut self, attr: Attributes) -> &mut Self {
        self.attrib = attr;
        self
    }
}

pub fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: Mul<f32, Output = T> + Add<Output = T>,
{
    return start * (1. - t) + end * t;
}
