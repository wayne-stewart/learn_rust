use crate::light::Light;
use crate::shape::Sphere;
use crate::tuple::Vector;
use std::collections::HashMap;

pub struct World {
    objects: HashMap,
    lights: HashMap
}

impl World {
    pub fn add_object(obj: Sphere) {
        
    }

    pub fn add_light(light: Light) {

    }

    pub fn intersect(ray: Vector) {

    }
}
