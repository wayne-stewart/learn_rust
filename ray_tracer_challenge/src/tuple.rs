use std::ops;
use crate::math::fequal;

#[derive(Debug, Clone, Copy)]
//#[derive(PartialEq)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32 // 0 is a vector 1 is a point
}

/*
    This allows to use == and != when comparing two tuples
*/
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let a = fequal(self.x, other.x);
        let b = fequal(self.y, other.y);
        let c = fequal(self.z, other.z);
        let d = fequal(self.w, other.w);
        return a && b && c && d;
    }
}

/*
    This allows to use + when adding two tuples
*/
impl ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

/*
    This allows to use - when subtracting two tuples
*/
impl ops::Sub for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

/*
    this allows to use - for negation
*/
const ZERO_VECTOR : Tuple = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        ZERO_VECTOR - self
    }
}

/*
    this allows to use * for multiplication with a scalar
*/
impl ops::Mul<f32> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f32) -> Tuple {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

/*
    this allows to use / for division
*/
impl ops::Div<f32> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f32) -> Tuple {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

pub fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w
    }
}

pub fn tuple_i(x: i32, y: i32, z: i32, w: i32) -> Tuple {
    Tuple {
        x: x as f32,
        y: y as f32,
        z: z as f32,
        w: w as f32
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    tuple(x, y, z, 1.0)
}

fn point_i (x: i32, y: i32, z: i32) -> Tuple {
    point(x as f32, y as f32, z as f32)
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    tuple(x, y, z, 0.0)
}

fn vector_i(x: i32, y: i32, z: i32) -> Tuple {
    vector(x as f32, y as f32, z as f32)
}

fn magnitude(v: &Tuple) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w).sqrt()
}

fn normalize(v: &Tuple) -> Tuple {
    let mag = magnitude(&v);
    tuple(
        v.x / mag,
        v.y / mag,
        v.z / mag,
        v.w / mag
    )
}

fn dot(a: &Tuple, b: &Tuple) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    )
}

#[test]
fn point_test() {
    let a = point(1.0,2.0,3.0);
    let b = point(1.000001,2.0,3.0);
    let c = point(1.001,2.0,3.0);
    let d = vector(1.0,2.0,3.0);
    assert_eq!(a,b);
    assert!(b !=c);
    assert!(a != d);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a.w, 1.0);
}

#[test]
fn vector_test() {
    let a = vector(1.0,2.0,3.0);
    let b = vector(1.000001,2.0,3.0);
    let c = vector(1.001,2.0,3.0);
    assert_eq!(a,b);
    assert!(b != c);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a.w, 0.0);
}

#[test]
fn add_test() {
    let p = point(3.0, -2.0, 5.0);
    let v = vector(-2.0, 3.0, 1.0);
    let r = p + v;
    let t = point(1.0, 1.0, 6.0);
    assert_eq!(r, t);
}

#[test]
fn subtract_test() {
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    let t = p1 - p2;
    let r = vector(-2.0, -4.0, -6.0);
    assert_eq!(r, t);

    let p = point_i(3,2,1);
    let v = vector_i(5,6,7);
    let t = p - v;
    let r = point_i(-2,-4,-6);
    assert_eq!(r,t);

    let v1 = vector_i (3,2,1);
    let v2 = vector_i (5,6,7);
    let t = v1 - v2;
    let r = vector_i(-2,-4,-6);
    assert_eq!(r,t);
}

#[test]
fn negation_test() {
    let v = vector_i(1, -2, 3);
    let t = -v;
    let r = vector_i(-1, 2, -3);
    assert_eq!(r, t);
}

#[test]
fn multiply_test() {
    let a = tuple_i(1, -2, 3, -4);
    let t = a * 3.5;
    let r = tuple(3.5, -7.0, 10.5, -14.0);
    assert_eq!(r, t);
}

#[test]
fn divide_test() {
    let a = tuple_i(1, -2, 3, -4);
    let t = a / 2.0;
    let r = tuple(0.5, -1.0, 1.5, -2.0);
    assert_eq!(r, t);
}

#[test]
fn magnitude_test() {
    let v = vector_i(-1,-2,-3);
    let t = magnitude(&v);
    let r = (14.0 as f32).sqrt();
    assert_eq!(r, t);
}

#[test]
fn normalize_test() {
    let v = vector_i(4,0,0);
    let t = normalize(&v);
    let r = vector_i(1,0,0);
    assert_eq!(r, t);

    let v = vector_i(1, 2, 3);
    let t = normalize(&v);
    let r = vector(0.26726, 0.53452, 0.80178);
    assert_eq!(r, t);
}

#[test]
fn dot_test() {
    let v1 = vector_i(1,2,3);
    let v2 = vector_i(2,3,4);
    let t = dot(&v1, &v2);
    let r = 20.0;
    assert_eq!(r, t);
}

#[test]
fn cross_test() {
    let v1 = vector_i(1,2,3);
    let v2 = vector_i(2,3,4);
    let t = cross(&v1, &v2);
    let s = cross(&v2, &v1);
    let rt = vector_i(-1,2,-1);
    let rs = vector_i(1,-2,1);
    assert_eq!(rt, t);
    assert_eq!(rs, s);
}
