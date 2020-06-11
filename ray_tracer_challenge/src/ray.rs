
use crate::tuple;
use crate::tuple::Tuple;
use crate::point;
use crate::vector;
use crate::matrix;
use crate::matrix::Matrix4x4;

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

    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray::new(
            m * self.origin,
            m * self.direction
        )
    }
}

pub struct Sphere {
    id: u32,
    center: Point,
    radius: f32,
    pub transform: Matrix4x4
}

impl Sphere {
    pub fn new(id: u32) -> Sphere {
        Sphere { 
            id,
            center: point!(0,0,0),
            radius: 1.0,
            transform: matrix::MATRIX_4X4_IDENTITY
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Vec<Intersection> {
        // transform the ray using the sphere transform before anything
        let ray = ray.transform(self.transform.inverse());
        // vector from sphere center to ray origin
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Vec::new();
        }
        else {
            let dsq = discriminant.sqrt();
            let t1 = (-b - dsq) / (2.0 * a);
            let t2 = (-b + dsq) / (2.0 * a);
            vec![
                Intersection { id: self.id, t: t1 },
                Intersection { id: self.id, t: t2 }]
        }
    }
}

pub struct Intersection {
    id: u32,
    t: f32
}

pub fn hit(xs: Vec<Intersection>) -> Option<f32> {
    if xs.len() == 0 {
        None
    }
    else {
        // try to find the lowest positive value
        let mut r: f32 = f32::MAX;
        for x in xs {
            if x.t >= 0.0 && x.t < r {
                r = x.t;
            }
        }
        if r == f32::MAX {
            None
        }
        else {
            Some(r)
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
    let sphere = Sphere::new(1);

    // intersect sphere at two points passing through center
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    let intersections = sphere.intersects(&ray);
    assert_eq!(2, intersections.len());
    assert_eq!(4.0, intersections[0].t);
    assert_eq!(6.0, intersections[1].t);

    // intersect sphere at a tangent
    let ray = Ray::new(point!(0,1,-5), vector!(0,0,1));
    let intersections = sphere.intersects(&ray);
    assert_eq!(2, intersections.len());
    assert_eq!(5.0, intersections[0].t);
    assert_eq!(5.0, intersections[1].t);

    // ray misses the sphere
    let ray = Ray::new(point!(0,2,-5), vector!(0,0,1));
    let intersections = sphere.intersects(&ray);
    assert_eq!(0, intersections.len());

    // ray starts at center of sphere, intersects forward and backward
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let intersections = sphere.intersects(&ray);
    assert_eq!(2, intersections.len());
    assert_eq!(-1.0, intersections[0].t);
    assert_eq!(1.0, intersections[1].t);

    // ray starts in front of sphere, intersects backward
    let ray = Ray::new(point!(0,0,5), vector!(0,0,1));
    let intersections = sphere.intersects(&ray);
    assert_eq!(2, intersections.len());
    assert_eq!(-6.0, intersections[0].t);
    assert_eq!(-4.0, intersections[1].t);
}

#[test]
fn hit_test() {
    let intersections = vec![
        Intersection { id: 1, t: 1.0 },
        Intersection { id: 1, t: 2.0 }
    ];
    assert_eq!(1.0, hit(intersections).unwrap());

    let intersections = vec![
        Intersection { id: 1, t: -1.0 },
        Intersection { id: 1, t: 1.0 }
    ];
    assert_eq!(1.0, hit(intersections).unwrap());

    let intersections = vec![
        Intersection { id: 1, t: -1.0 },
        Intersection { id: 1, t: -2.0 }
    ];
    assert_eq!(None, hit(intersections));

    let intersections = vec![
        Intersection { id: 1, t: 5.0 },
        Intersection { id: 1, t: 7.0 },
        Intersection { id: 1, t: -3.0 },
        Intersection { id: 1, t: 2.0 }
    ];
    assert_eq!(2.0, hit(intersections).unwrap());
}

#[test]
fn ray_transform_test() {
    let ray = Ray::new(point!(1,2,3), vector!(0,1,0));

    // translation only affects origin since it is a point
    // translation does not effect the direction because it is a vector
    let translation = Matrix4x4::translation(3.0,4.0,5.0);
    let ray2 = ray.transform(translation);
    assert_eq!(ray2.origin, point!(4,6,8));
    assert_eq!(ray2.direction, vector!(0,1,0));

    // scaling affects both origin and direction
    let transform = Matrix4x4::scaling(2.0,3.0,4.0);
    let ray3 = ray.transform(transform);
    assert_eq!(ray3.origin, point!(2,6,12));
    assert_eq!(ray3.direction, vector!(0,3,0));
}

#[test]
fn sphere_transform_test() {
    let mut sphere = Sphere::new(1);
    let ray = Ray::new(point!(0,0,-5), vector!(0,0,1));
    
    // first validate the default transform exists
    assert_eq!(sphere.transform, matrix::MATRIX_4X4_IDENTITY);

    // test casting a ray affected by scaling
    sphere.transform = Matrix4x4::scaling(2.0,2.0,2.0);
    let intersections = sphere.intersects(&ray);
    assert_eq!(2, intersections.len());
    assert_eq!(3.0, intersections[0].t);
    assert_eq!(7.0, intersections[1].t);

    // test casting a ray affected by translation
    sphere.transform = Matrix4x4::translation(5.0,0.0,0.0);
    let intersections = sphere.intersects(&ray);
    assert_eq!(0, intersections.len());
}