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

pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<Light>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            lights: Vec::new()
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::<Intersection>::with_capacity(self.objects.len() * 2);
        for obj in self.objects.iter() {
            let mut obj_intersections = obj.intersects(&ray);
            intersections.append(&mut obj_intersections);
        }
        intersections.sort_by(|a,b| a.t.partial_cmp(&b.t).unwrap());
        return intersections;
    }

    pub fn shade_hit(&self, comps: &HitComputations) -> Color {
        let light = self.lights.first().unwrap();
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

    let light = Light::point_light(point!(-10,10,-10), rgb!(1,1,1));
    world.lights.push(light);

    let mut sphere1 = Sphere::new();
    sphere1.material.color = rgb!(0.8,1,0.6);
    sphere1.material.diffuse = 0.7;
    sphere1.material.specular = 0.2;
    world.objects.push(sphere1);

    let mut sphere2 = Sphere::new();
    sphere2.transform = Matrix4x4::scaling(0.5,0.5,0.5);
    world.objects.push(sphere2);

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
    let sphere = Sphere::new();
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
    let sphere = Sphere::new();
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
    let sphere = world.objects.first().unwrap();
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let intersection = Intersection { object: &sphere, t: 4.0 };
    let comps = prepare_computations(&ray, &intersection);
    let color = world.shade_hit(&comps);
    assert_eq!(color, rgb!(0.38066, 0.47583, 0.2855));

    // shading intersection from the inside
    world.lights.clear();
    world.lights.push(Light::point_light(point!(0,0.25,0), rgb!(1,1,1)));
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let sphere = world.objects.last().unwrap();
    let intersection = Intersection {object: &sphere, t: 0.5 };
    let comps = prepare_computations(&ray, &intersection);
    let color = world.shade_hit(&comps);
    assert_eq!(color, rgb!(0.90498, 0.90498, 0.90498));
}