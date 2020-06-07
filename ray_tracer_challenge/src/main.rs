use std::ops;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
//#[derive(PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32 // 0 is a vector 1 is a point
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
    this allows to use * for multiplication
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

fn fequal(a: f32, b: f32) -> bool {
    let mut x = a - b;
    if x < 0.0 {
        x *= -1.0;
    }
    if x > 0.0001 {
        return false;
    }
    else {
        return true;
    }
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 1.0
    }
}

fn point_i (x: i32, y: i32, z: i32) -> Tuple {
    point(x as f32, y as f32, z as f32)
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 0.0
    }
}

fn vector_i(x: i32, y: i32, z: i32) -> Tuple {
    vector(x as f32, y as f32, z as f32)
}

// fn add(a: Tuple, b: Tuple) -> Tuple {
//     Tuple {
//         x: a.x + b.x,
//         y: a.y + b.y,
//         z: a.z + b.z,
//         w: a.w + b.w
//     }
// }

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
    let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
    let t = a * 3.5;
    let r = Tuple{x: 3.5, y: -7.0, z: 10.5, w: -14.0 };
    assert_eq!(r, t);
}

#[test]
fn divide_test() {
    let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
    let t = a / 2.0;
    let r = Tuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0 };
    assert_eq!(r, t);
}