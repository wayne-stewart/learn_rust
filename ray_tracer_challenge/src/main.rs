

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
//#[derive(PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: i32 // 0 is a vector 1 is a point
}

/*
    This allows for us to use == and != when comparing two tuples
*/
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let a = fequal(self.x, other.x);
        let b = fequal(self.y, other.y);
        let c = fequal(self.z, other.z);
        let d = self.w == other.w;
        return a && b && c && d;
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
        w: 1
    }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 0
    }
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
    assert_eq!(a.w, 1);
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
    assert_eq!(a.w, 0);
}