
use crate::tuple::Point;
use crate::tuple::Vector;
use crate::point;
use crate::vector;
use crate::matrix;
use crate::matrix::Matrix4x4;
use crate::shape::Sphere;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin.add(&self.direction.multiplyf(t))
    }

    pub fn transform(&self, m: &Matrix4x4) -> Ray {
        Ray::new(
            m.multiply_tuple(&self.origin),
            m.multiply_tuple(&self.direction)
        )
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
fn ray_transform_test() {
    let ray = Ray::new(point!(1,2,3), vector!(0,1,0));

    // translation only affects origin since it is a point
    // translation does not effect the direction because it is a vector
    let translation = Matrix4x4::translation(3.0,4.0,5.0);
    let ray2 = ray.transform(&translation);
    assert_eq!(ray2.origin, point!(4,6,8));
    assert_eq!(ray2.direction, vector!(0,1,0));

    // scaling affects both origin and direction
    let transform = Matrix4x4::scaling(2.0,3.0,4.0);
    let ray3 = ray.transform(&transform);
    assert_eq!(ray3.origin, point!(2,6,12));
    assert_eq!(ray3.direction, vector!(0,3,0));
}

#[test]
fn sphere_transform_test() {
    let mut sphere = Sphere::new();
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