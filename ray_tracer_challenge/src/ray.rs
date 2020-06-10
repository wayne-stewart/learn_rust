
use crate::tuple;
use crate::tuple::Tuple;
use crate::point;
use crate::vector;


pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
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