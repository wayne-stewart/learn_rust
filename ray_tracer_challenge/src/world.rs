use crate::light::Light;
use crate::shape::Sphere;
use crate::tuple::Point;
use crate::tuple::Vector;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::ray::Intersection;
use crate::point;
use crate::vector;
use crate::rgb;
use std::collections::HashMap;

pub struct World {
    objects: HashMap<u32, Sphere>,
    lights: HashMap<u32, Light>
}

impl World {
    pub fn new() -> World {
        World {
            objects: HashMap::new(),
            lights: HashMap::new()
        }
    }

    pub fn add_object(&mut self, obj: Sphere) {
        self.objects.insert(obj.id, obj);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.insert(light.id, light);
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::<Intersection>::with_capacity(self.objects.len() * 2);
        for (key, obj) in self.objects.iter() {
            let mut obj_intersections = obj.intersects(&ray);
            intersections.append(&mut obj_intersections);
        }
        intersections.sort_by(|a,b| a.t.partial_cmp(&b.t).unwrap());
        return intersections;
    }
}


fn default_world() -> World {
    let mut world = World::new();

    let light = Light::point_light(1, point!(-10,10,-10), rgb!(1,1,1));

    let mut sphere1 = Sphere::new(1);
    sphere1.material.color = rgb!(0.8,1,0.6);
    sphere1.material.diffuse = 0.7;
    sphere1.material.specular = 0.2;

    let mut sphere2 = Sphere::new(2);
    sphere2.transform = Matrix4x4::scaling(0.5,0.5,0.5);

    world.add_light(light);
    world.add_object(sphere1);
    world.add_object(sphere2);
    
    return world;
}

#[test]
fn intersect_test() {
    let world = default_world();
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let xs = world.intersect(&ray);
    assert_eq!(4, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(4.5, xs[1].t);
    assert_eq!(5.5, xs[2].t);
    assert_eq!(6.0, xs[3].t);
}