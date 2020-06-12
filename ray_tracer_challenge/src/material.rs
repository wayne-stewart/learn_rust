use crate::color::Color;
use crate::rgb;

pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: rgb!(1,1,1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }
}
