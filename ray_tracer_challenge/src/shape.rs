use crate::tuple::Point;
use crate::tuple::Vector;
use crate::point;
use crate::vector;
use crate::matrix;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::world::Intersection;
use crate::material::Material;
use crate::math;

enum ShapeType {
    Sphere,
    Plane
}

pub struct Shape {
    shape_type: ShapeType,
    pub material: Material,
    pub transform: Matrix4x4
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape {
            shape_type: ShapeType::Sphere,
            material: Material::new(),
            transform: matrix::MATRIX_4X4_IDENTITY
        }
    }

    /*
        planes are oriented in xz by default
        normal is in
    */
    pub fn plane() -> Shape {
        Shape {
            shape_type: ShapeType::Plane,
            material: Material::new(),
            transform: matrix::MATRIX_4X4_IDENTITY
        }
    }

    pub fn intersects<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        // transform the ray using the sphere transform before anything
        let local_ray = ray.transform(&self.transform.inverse());
        match self.shape_type {
            ShapeType::Sphere => sphere_intersects(&self, &local_ray),
            ShapeType::Plane => plane_intersects(&self, &local_ray)
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let inverted_transform = self.transform.inverse();
        let transposed_inverted_transform = inverted_transform.transpose();
        let local_point = inverted_transform.multiply_tuple(&world_point);
    
        let local_normal = match self.shape_type {
            ShapeType::Sphere => sphere_normal_at(&self, &local_point),
            ShapeType::Plane => plane_normal_at(&self, &world_point)
        };

        let mut world_normal = transposed_inverted_transform.multiply_tuple(&local_normal);
        // hack to avoid problems with translations in the transform
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

fn sphere_intersects<'a>(shape: &'a Shape, local_ray: &Ray) -> Vec<Intersection<'a>> {

    // vector from sphere center to ray origin
    let sphere_to_ray = local_ray.origin.subtract(&point!(0,0,0));
    let a = local_ray.direction.dot(&local_ray.direction);
    let b = local_ray.direction.dot(&sphere_to_ray) * 2.0;
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
            Intersection { object: &shape, t: t1 },
            Intersection { object: &shape, t: t2 }]
    }
}

fn sphere_normal_at(_shape: &Shape, local_point: &Point) -> Vector {
    local_point.subtract(&point!(0,0,0))
}

fn plane_normal_at(_shape: &Shape, _local_point: &Point) -> Vector {
    vector!(0,1,0)
}

fn plane_intersects<'a>(shape: &'a Shape, local_ray: &Ray) -> Vec<Intersection<'a>> {
    // test if ray is coplanar
    // since planes are in xz, any y must be 0
    if local_ray.direction.y.abs() < math::EPSILON {
        return Vec::new();
    }
    else {
        let t = -local_ray.origin.y / local_ray.direction.y;
        vec![Intersection{ object: &shape, t: t }]
    }
}

#[test]
fn ray_sphere_intersects_test() {
    let sphere = Shape::sphere();

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
fn sphere_normal_at_origin_test() {
    let sphere = Shape::sphere();
    let sqrt3div3 = 3_f32.sqrt()/3.0;

    assert_eq!(sphere.normal_at(&point!(1,0,0)), vector!(1,0,0));
    assert_eq!(sphere.normal_at(&point!(0,1,0)), vector!(0,1,0));
    assert_eq!(sphere.normal_at(&point!(0,0,1)), vector!(0,0,1));
    let n = sphere.normal_at(&point!(sqrt3div3, sqrt3div3, sqrt3div3));
    assert_eq!(n, vector!(sqrt3div3,sqrt3div3,sqrt3div3));
    assert_eq!(n, n.normalize());
}

#[test]
fn sphere_normal_at_transformed_test() {
    let mut sphere = Shape::sphere();

    // translated off origin test
    sphere.transform = Matrix4x4::translation(0.0, 1.0, 0.0);
    let n = sphere.normal_at(&point!(0, 1.70711, -0.70711));
    assert_eq!(n, vector!(0, 0.70711, -0.70711));

    // scaled (squashed) and rotated test
    sphere.transform = Matrix4x4::scaling(1.0, 0.5, 1.0).multiply(&Matrix4x4::rotation_z(std::f32::consts::PI));
    let n = sphere.normal_at(&point!(0, 2_f32.sqrt()/2.0, -2_f32.sqrt()/2.0));
    assert_eq!(n, vector!(0, 0.97014, -0.24254));
}

#[test]
fn plane_normal_at_test() {
    let plane = Shape::plane();
    let n1 = plane_normal_at(&plane, &point!(0,0,0));
    let n2 = plane_normal_at(&plane, &point!(10,0,-10));
    let n3 = plane_normal_at(&plane, &point!(-5,0,150));
    assert_eq!(n1, vector!(0,1,0));
    assert_eq!(n2, vector!(0,1,0));
    assert_eq!(n3, vector!(0,1,0));
}

#[test]
fn plane_intersects_test() {
    // default plane lies in xz
    let plane = Shape::plane();

    // ray parallel to plane
    let ray = Ray::new(point!(0,10,0), vector!(0,0,1));
    let xs = plane_intersects(&plane, &ray);
    assert_eq!(0, xs.len());

    // ray is coplanar ( in the plane )
    // infinite intersections so we don't return any intersections
    let ray = Ray::new(point!(0,0,0), vector!(0,0,1));
    let xs = plane_intersects(&plane, &ray);
    assert_eq!(0, xs.len());

    // ray intersecting from above
    let ray = Ray::new(point!(0,1,0), vector!(0,-1,0));
    let xs = plane_intersects(&plane, &ray);
    assert_eq!(1, xs.len());
    assert_eq!(1.0, xs[0].t);

    // ray intersecting from below
    let ray = Ray::new(point!(0,-1,0), vector!(0,1,0));
    let xs = plane_intersects(&plane, &ray);
    assert_eq!(1, xs.len());
    assert_eq!(1.0, xs[0].t);
}