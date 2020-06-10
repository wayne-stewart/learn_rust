
use crate::tuple;
use crate::tuple::Tuple;
use crate::point;
use crate::vector;

type Point = Tuple;
type Vector = Tuple;

pub struct Ray {
    origin: Point,
    direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }
}

pub struct Sphere {
    center: Point,
    radius: f32
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { 
            center: point!(0,0,0),
            radius: 1.0
        }
    }

    pub fn intersects(&self, ray: &Ray) -> (bool, f32, f32) {
        // vector from sphere center to ray origin
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return (false, 0.0, 0.0);
        }
        else {
            let dsq = discriminant.sqrt();
            let t1 = (-b - dsq) / (2.0 * a);
            let t2 = (-b + dsq) / (2.0 * a);
            return (true, t1, t2);
        }
    }
}

#[test]
fn position_test() {
    let ray = Ray::new(point!(2,3,4), vector!(1,0,0));
    assert_eq!(ray.position(0.0), point!(2,3,4));
    assert_eq!(ray.position(1.0), point!(3,3,4));
    assert_eq!(ray.position(-1.0), point!(1,3,4));
    assert_eq!(ray.position(2.5), point!(4.5,3,4));
}

#[test]
fn ray_sphere_intersects_test() {
    let sphere = Sphere::new();

    // intersect sphere at two points passing through center
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let (b, t1, t2) = sphere.intersects(&ray);
    assert_eq!(true, b);
    assert_eq!(4.0, t1);
    assert_eq!(6.0, t2);

    // intersect sphere at a tangent
    let ray = Ray::new(point!(0,1,-5), vector!(0,0,1));
    let (b, t1, t2) = sphere.intersects(&ray);
    assert_eq!(true, b);
    assert_eq!(5.0, t1);
    assert_eq!(5.0, t2);

    // ray misses the sphere
    let ray = Ray::new(point!(0,2,-5), vector!(0,0,1));
    let (b, t1, t2) = sphere.intersects(&ray);
    assert_eq!(false, b);

    // ray starts at center of sphere, intersects forward and backward
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let (b, t1, t2) = sphere.intersects(&ray);
    assert_eq!(true, b);
    assert_eq!(-1.0, t1);
    assert_eq!(1.0, t2);

    // ray starts in front of sphere, intersects backward
    let ray = Ray::new(point!(0,0,5), vector!(0,0,1));
    let (b, t1, t2) = sphere.intersects(&ray);
    assert_eq!(true, b);
    assert_eq!(-6.0, t1);
    assert_eq!(-4.0, t2);
}