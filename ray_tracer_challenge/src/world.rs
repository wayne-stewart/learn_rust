use crate::light::Light;
use crate::light::lighting;
use crate::shape::Sphere;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::ray::Intersection;
use crate::color::Color;
use crate::tuple::Point;
use crate::tuple::Vector;
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

    pub fn get_object(&self, id: u32) -> Option<&Sphere> {
        self.objects.get(&id)
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.insert(light.id, light);
    }

    pub fn get_light(&self, id: u32) -> Option<&Light> {
        self.lights.get(&id)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::<Intersection>::with_capacity(self.objects.len() * 2);
        for (_key, obj) in self.objects.iter() {
            let mut obj_intersections = obj.intersects(&ray);
            intersections.append(&mut obj_intersections);
        }
        intersections.sort_by(|a,b| a.t.partial_cmp(&b.t).unwrap());
        return intersections;
    }

    pub fn shade_hit(&self, comps: &HitComputations) -> Color {
        let light = self.get_light(1).unwrap();
        lighting(
            &comps.object.material,
            &light,
            &comps.point,
            &comps.eyev,
            &comps.normalv)
    }
}


fn default_world() -> World {
    let mut world = World::new();

    let light = Light::point_light(1, point!(-10,10,-10), rgb!(1,1,1));
    world.add_light(light);

    let mut sphere1 = Sphere::new(1);
    sphere1.material.color = rgb!(0.8,1,0.6);
    sphere1.material.diffuse = 0.7;
    sphere1.material.specular = 0.2;
    world.add_object(sphere1);

    let mut sphere2 = Sphere::new(2);
    sphere2.transform = Matrix4x4::scaling(0.5,0.5,0.5);
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


pub struct HitComputations <'a> {
    object: &'a Sphere,
    t: f32,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool
}

fn prepare_computations<'a>(ray: &Ray, intersection: &Intersection<'a>) -> HitComputations<'a> {
    let point = ray.position(intersection.t);
    let mut normalv = intersection.object.normal_at(&point);
    let eyev = ray.direction.negate();
    let inside: bool;
    if normalv.dot(&eyev) < 0.0 {
        inside = true;
        normalv = normalv.negate();
    }
    else {
        inside = false;
    }
    HitComputations {
        object: &intersection.object,
        t: intersection.t,
        point: point.clone(),
        eyev,
        normalv,
        inside
    }
}

#[test]
fn prepare_computations_test() {
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let sphere = Sphere::new(1);
    let intersection = Intersection { object: &sphere, t: 4.0 };
    let comps = prepare_computations(&ray, &intersection);
    assert_eq!(comps.t, 4.0);
    assert_eq!(comps.point, point!(0,0,-1));
    assert_eq!(comps.eyev, vector!(0,0,-1));
    assert_eq!(comps.normalv, vector!(0,0,-1));
    assert_eq!(comps.inside, false);
}

#[test]
fn prepare_computations_intersection_inside_sphere_test() {
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let sphere = Sphere::new(1);
    let intersection = Intersection { object: &sphere, t: 1.0 };
    let comps = prepare_computations(&ray, &intersection);
    assert_eq!(comps.t, 1.0);
    assert_eq!(comps.point, point!(0,0,1));
    assert_eq!(comps.eyev, vector!(0,0,-1));
    assert_eq!(comps.normalv, vector!(0,0,-1));
    assert_eq!(comps.inside, true);
}


#[test]
fn shade_hit_test() {
    let mut world = default_world();

    // shading intersection from the outside
    let sphere = world.get_object(1).unwrap();
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let intersection = Intersection { object: &sphere, t: 4.0 };
    let comps = prepare_computations(&ray, &intersection);
    let color = world.shade_hit(&comps);
    assert_eq!(color, rgb!(0.38066, 0.47583, 0.2855));

    // shading intersection from the inside
    world.lights.clear();
    world.add_light(Light::point_light(1, point!(0,0.25,0), rgb!(1,1,1)));
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let sphere = world.get_object(2).unwrap();
    let intersection = Intersection {object: &sphere, t: 0.5 };
    let comps = prepare_computations(&ray, &intersection);
    let color = world.shade_hit(&comps);
    assert_eq!(color, rgb!(0.90498, 0.90498, 0.90498));
}