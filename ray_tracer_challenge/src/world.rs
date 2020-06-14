use crate::light::Light;
use crate::light::lighting;
use crate::shape::Shape;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::color::Color;
use crate::tuple::Point;
use crate::tuple::Vector;
use crate::point;
use crate::vector;
use crate::rgb;
use crate::math;

pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<Light>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            lights: Vec::new()
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = intersect(&self, &ray);
        match hit(&intersections) {
            None => Color::BLACK,
            Some(intersection) => {
                let comps = prepare_computations(&ray, &intersection);
                let color = shade_hit(&self, &comps);
                return color;
            }
        }
    }
}

struct HitComputations <'a> {
    object: &'a Shape,
    t: f32,
    point: Point,
    over_point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool
}

pub struct Intersection<'a> {
    pub object: &'a Shape,
    pub t: f32
}

fn default_world() -> World {
    let mut world = World::new();

    let light = Light::point_light(point!(-10,10,-10), rgb!(1,1,1));
    world.lights.push(light);

    let mut sphere1 = Shape::sphere();
    sphere1.material.color = rgb!(0.8,1,0.6);
    sphere1.material.diffuse = 0.7;
    sphere1.material.specular = 0.2;
    world.objects.push(sphere1);

    // this sphere is inside the first one, scaled down by half
    let mut sphere2 = Shape::sphere();
    sphere2.transform = Matrix4x4::scaling(0.5,0.5,0.5);
    world.objects.push(sphere2);

    return world;
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
        over_point: point.add(&normalv.multiplyf(math::EPSILON)),
        eyev,
        normalv,
        inside
    }
}

fn intersect<'a>(world: &'a World, ray: &'a Ray) -> Vec<Intersection<'a>> {
    let mut intersections = Vec::<Intersection>::with_capacity(world.objects.len() * 2);
    for obj in world.objects.iter() {
        let mut obj_intersections = obj.intersects(&ray);
        intersections.append(&mut obj_intersections);
    }
    intersections.sort_by(|a,b| a.t.partial_cmp(&b.t).unwrap());
    return intersections;
}

pub fn hit<'a>(xs: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    if xs.len() == 0 {
        None
    }
    else {
        // try to find the lowest positive value
        let mut result = xs.first();
        let mut r = f32::MAX;
        for x in xs {
            if x.t >= 0.0 && x.t < r {
                r = x.t;
                result = Some(&x);
            }
        }
        if r == f32::MAX {
            None
        }
        else {
            result
        }
    }
}


fn shade_hit(world: &World, comps: &HitComputations) -> Color {
    let mut result: Color = Color::BLACK;
    for light in world.lights.iter() {
        let in_shadow = is_shadowed(&world, &comps.over_point, &light);
        let color = lighting(
            &comps.object.material,
            &light,
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            in_shadow);
        result = result.add(&color);
    }
    return result;
}

fn is_shadowed(world: &World, point: &Point, light: &Light) -> bool {
    let v = light.position.subtract(&point);
    let distance = v.magnitude();
    let direction = v.normalize();
    let ray = Ray::new(point.clone(), direction);
    let intersections = intersect(&world, &ray);
    match hit(&intersections) {
        None => false,
        Some(h) => {
            if h.t < distance {
                true
            }
            else {
                false
            }
        }
    }
    
}

#[test]
fn intersect_test() {
    let world = default_world();
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let xs = intersect(&world, &ray);
    assert_eq!(4, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(4.5, xs[1].t);
    assert_eq!(5.5, xs[2].t);
    assert_eq!(6.0, xs[3].t);
}

#[test]
fn hit_test() {
    let sphere = Shape::sphere();

    let intersections = vec![
        Intersection { object: &sphere, t: 1.0 },
        Intersection { object: &sphere, t: 2.0 }
    ];
    assert_eq!(1.0, hit(&intersections).unwrap().t);

    let intersections = vec![
        Intersection { object: &sphere, t: -1.0 },
        Intersection { object: &sphere, t: 1.0 }
    ];
    assert_eq!(1.0, hit(&intersections).unwrap().t);

    let intersections = vec![
        Intersection { object: &sphere, t: -1.0 },
        Intersection { object: &sphere, t: -2.0 }
    ];
    match hit(&intersections) {
        None => { },
        Some(_) => panic!("hit should have retuned None!")
    }

    let intersections = vec![
        Intersection { object: &sphere, t: 5.0 },
        Intersection { object: &sphere, t: 7.0 },
        Intersection { object: &sphere, t: -3.0 },
        Intersection { object: &sphere, t: 2.0 }
    ];
    assert_eq!(2.0, hit(&intersections).unwrap().t);
}

#[test]
fn prepare_computations_test() {
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let sphere = Shape::sphere();
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
    let sphere = Shape::sphere();
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
    let color = shade_hit(&world, &comps);
    assert_eq!(color, rgb!(0.38066, 0.47583, 0.2855));

    // shading intersection from the inside
    world.lights.clear();
    world.lights.push(Light::point_light(point!(0,0.25,0), rgb!(1,1,1)));
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let sphere = world.objects.last().unwrap();
    let intersection = Intersection {object: &sphere, t: 0.5 };
    let comps = prepare_computations(&ray, &intersection);
    let color = shade_hit(&world, &comps);
    assert_eq!(color, rgb!(0.90498, 0.90498, 0.90498));
}

#[test]
fn color_at_test() {
    let mut world = default_world();

    let ray = Ray::new(point!(0,0,-5), vector!(0,1,0));
    let color = world.color_at(&ray);
    assert_eq!(color, Color::BLACK);

    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let color = world.color_at(&ray);
    assert_eq!(color, rgb!(0.38066, 0.47583, 0.2855));

    let outer = world.objects.first_mut().unwrap();
    outer.material.ambient = 1.0;
    let inner = world.objects.last_mut().unwrap();
    inner.material.ambient = 1.0;
    let ray = Ray::new(point!(0,0,0.75), vector!(0,0,-1));
    let color = world.color_at(&ray);
    assert_eq!(color, world.objects.last().unwrap().material.color);
}

#[test]
fn is_shadowed_test() {
    let w = default_world();
    let light = w.lights.first().unwrap();

    let p = point!(0,10,0);
    let in_shadow = is_shadowed(&w, &p, &light);
    assert_eq!(false, in_shadow);

    let p = point!(10,-10,10);
    let in_shadow = is_shadowed(&w, &p, &light);
    assert_eq!(true, in_shadow);

    let p = point!(-20,20,-20);
    let in_shadow = is_shadowed(&w, &p, &light);
    assert_eq!(false, in_shadow);

    let p = point!(-2,2,-2);
    let in_shadow = is_shadowed(&w, &p, &light);
    assert_eq!(false, in_shadow);
}
