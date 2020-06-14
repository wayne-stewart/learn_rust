
pub const EPSILON : f32 = 0.0001;

pub fn fequal(a: f32, b: f32) -> bool {
    let mut x = a - b;
    if x < 0.0 {
        x *= -1.0;
    }
    if x > EPSILON {
        return false;
    }
    else {
        return true;
    }
}

