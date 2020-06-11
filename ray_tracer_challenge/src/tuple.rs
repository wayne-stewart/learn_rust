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

pub type Point = Tuple;
pub type Vector = Tuple;

#[macro_export]
macro_rules! tuple {
    ($x:expr, $y:expr, $z:expr, $w:expr ) => {
        Tuple {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
            w: $w as f32
        }
    };
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr ) => {
        Point {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
            w: 1.0
        }
    };
}

#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr ) => {
        Vector {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
            w: 0.0
        }
    };
}

impl Tuple {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    
    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        tuple!(
            self.x / mag,
            self.y / mag,
            self.z / mag,
            self.w / mag
        )
    }
    
    pub fn dot(&self, b: &Tuple) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
    
    pub fn cross(&self, b: &Tuple) -> Tuple {
        vector!(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x
        )
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        *self - *normal * 2.0 * self.dot(&normal)
    }
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
const ZERO_VECTOR : Tuple = vector!(0,0,0);

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

#[test]
fn point_test() {
    let a = point!(1,2,3);
    let b = point!(1.000001,2.0,3.0);
    let c = point!(1.001,2.0,3.0);
    let d = vector!(1,2,3);
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
    let a = vector!(1,2,3);
    let b = vector!(1.000001,2.0,3.0);
    let c = vector!(1.001,2.0,3.0);
    assert_eq!(a,b);
    assert!(b != c);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a.w, 0.0);
}

#[test]
fn add_test() {
    let p = point!(3, -2, 5);
    let v = vector!(-2, 3, 1);
    let r = p + v;
    let t = point!(1, 1, 6);
    assert_eq!(r, t);
}

#[test]
fn subtract_test() {
    let p1 = point!(3, 2, 1);
    let p2 = point!(5, 6, 7);
    let t = p1 - p2;
    let r = vector!(-2, -4, -6);
    assert_eq!(r, t);

    let p = point!(3,2,1);
    let v = vector!(5,6,7);
    let t = p - v;
    let r = point!(-2,-4,-6);
    assert_eq!(r,t);

    let v1 = vector!(3,2,1);
    let v2 = vector!(5,6,7);
    let t = v1 - v2;
    let r = vector!(-2,-4,-6);
    assert_eq!(r,t);
}

#[test]
fn negation_test() {
    let v = vector!(1, -2, 3);
    let t = -v;
    let r = vector!(-1, 2, -3);
    assert_eq!(r, t);
}

#[test]
fn multiply_test() {
    let a = tuple!(1, -2, 3, -4);
    let t = a * 3.5;
    let r = tuple!(3.5, -7.0, 10.5, -14.0);
    assert_eq!(r, t);
}

#[test]
fn divide_test() {
    let a = tuple!(1, -2, 3, -4);
    let t = a / 2.0;
    let r = tuple!(0.5, -1.0, 1.5, -2.0);
    assert_eq!(r, t);
}

#[test]
fn magnitude_test() {
    let v = vector!(-1,-2,-3);
    let t = v.magnitude();
    let r = (14.0 as f32).sqrt();
    assert_eq!(r, t);
}

#[test]
fn normalize_test() {
    let v = vector!(4,0,0);
    let t = v.normalize();
    let r = vector!(1,0,0);
    assert_eq!(r, t);

    let v = vector!(1, 2, 3);
    let t = v.normalize();
    let r = vector!(0.26726, 0.53452, 0.80178);
    assert_eq!(r, t);
}

#[test]
fn dot_test() {
    let v1 = vector!(1,2,3);
    let v2 = vector!(2,3,4);
    let t = v1.dot(&v2);
    let r = 20.0;
    assert_eq!(r, t);
}

#[test]
fn cross_test() {
    let v1 = vector!(1,2,3);
    let v2 = vector!(2,3,4);
    let t = v1.cross(&v2);
    let s = v2.cross(&v1);
    let rt = vector!(-1,2,-1);
    let rs = vector!(1,-2,1);
    assert_eq!(rt, t);
    assert_eq!(rs, s);
}

#[test]
fn reflect_test() {
    // incoming vector at 45 degree angle
    let v = vector!(1,-1,0);
    // vertical normal
    let n = vector!(0,1,0);
    let r = v.reflect(&n);
    // reflected vector should mirror v with y reversed
    assert_eq!(r, vector!(1,1,0));

    // incoming vector coming in straight down
    let v = vector!(0,-1,0);
    // 45 degree normal
    let n = vector!(2_f32.sqrt()/2.0, 2_f32.sqrt()/2.0, 0);
    let r = v.reflect(&n);
    // reflected vector should be horizontal
    assert_eq!(r, vector!(1,0,0));
}

