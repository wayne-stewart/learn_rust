use crate::color::Color;
use crate::tuple::Point;
use crate::rgb;
use crate::point;
use crate::matrix;
use crate::shape::Shape;
use crate::matrix::Matrix4x4;

pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub pattern: Option<Pattern>
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: rgb!(1,1,1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None
        }
    }
}

pub enum PatternType {
    Stripe,
    Gradient,
    Checkered
}

pub struct Pattern {
    pattern_type: PatternType,
    pub transform: Matrix4x4,
    pub color1: Color,
    pub color2: Color
}

impl Pattern {
    pub fn stripe(color1: Color, color2: Color, transform: Matrix4x4) -> Option<Pattern> {
        Some(Pattern {
            pattern_type: PatternType::Stripe,
            transform,
            color1,
            color2
        })
    }

    pub fn gradient(color1: Color, color2: Color, transform: Matrix4x4) -> Option<Pattern> {
        Some(Pattern {
            pattern_type: PatternType::Gradient,
            transform,
            color1,
            color2
        })
    }

    pub fn checkered(color1: Color, color2: Color, transform: Matrix4x4) -> Option<Pattern> {
        Some(Pattern {
            pattern_type: PatternType::Checkered,
            transform,
            color1,
            color2
        })
    }

    pub fn color_at(&self, shape: &Shape, world_point: &Point) -> Color {
        let shape_point = shape.transform.inverse().multiply_tuple(&world_point);
        let pattern_point = self.transform.inverse().multiply_tuple(&shape_point);
        match self.pattern_type {
            PatternType::Stripe => stripe_color_at(&self, &pattern_point),
            PatternType::Gradient => gradient_color_at(&self, &pattern_point),
            PatternType::Checkered => checkered_color_at(&self, &pattern_point)
        }
    }
}

fn stripe_color_at(pattern: &Pattern, point: &Point) -> Color {
    if (point.x.floor() as i32) % 2 == 0 {
        pattern.color1.clone()
    }
    else {
        pattern.color2.clone()
    }
}

fn gradient_color_at(pattern: &Pattern, point: &Point) -> Color {
    let color_distance = pattern.color2.subtract(&pattern.color1);
    let fraction = point.x - point.x.floor();
    pattern.color1.add(&color_distance.multiplyf(fraction))
}

fn checkered_color_at(pattern: &Pattern, point: &Point) -> Color {
    let floorx = point.x.floor() as i32;
    //let floory = point.y.floor() as i32;
    let floorz = point.z.floor() as i32;
    let mod2 = (floorx + floorz) % 2;
    if mod2 == 0 {
        pattern.color1.clone()
    }
    else {
        pattern.color2.clone()
    }
}

#[test]
fn stripe_color_at_test() {
    let pattern = Pattern::stripe(Color::WHITE, Color::BLACK, matrix::MATRIX_4X4_IDENTITY).unwrap();
    assert_eq!(Color::WHITE, stripe_color_at(&pattern, &point!(0,0,0)));
    assert_eq!(Color::WHITE, stripe_color_at(&pattern, &point!(0.9,0,0)));
    assert_eq!(Color::BLACK, stripe_color_at(&pattern, &point!(1,0,0)));
    assert_eq!(Color::BLACK, stripe_color_at(&pattern, &point!(-0.1,0,0)));
    assert_eq!(Color::BLACK, stripe_color_at(&pattern, &point!(-1,0,0)));
    assert_eq!(Color::WHITE, stripe_color_at(&pattern, &point!(-1.1,0,0)));
}

#[test]
fn gradient_color_at_test() {
    let pattern = Pattern::gradient(Color::WHITE, Color::BLACK, matrix::MATRIX_4X4_IDENTITY).unwrap();
    assert_eq!(Color::WHITE, gradient_color_at(&pattern, &point!(0,0,0)));
    assert_eq!(rgb!(0.75,0.75,0.75), gradient_color_at(&pattern, &point!(0.25,0,0)));
    assert_eq!(rgb!(0.5, 0.5,0.5), gradient_color_at(&pattern, &point!(0.5,0,0)));
    assert_eq!(rgb!(0.25,0.25,0.25), gradient_color_at(&pattern, &point!(0.75,0,0)));
}

#[test]
fn checkered_color_at_test() {
    let pattern = Pattern::checkered(Color::WHITE, Color::BLACK, matrix::MATRIX_4X4_IDENTITY).unwrap();
    assert_eq!(Color::WHITE, checkered_color_at(&pattern, &point!(0,0,0)));
    assert_eq!(Color::WHITE, checkered_color_at(&pattern, &point!(0.99,0,0)));
    assert_eq!(Color::BLACK, checkered_color_at(&pattern, &point!(1.01,0,0)));

    assert_eq!(Color::WHITE, checkered_color_at(&pattern, &point!(0,0,0)));
    assert_eq!(Color::WHITE, checkered_color_at(&pattern, &point!(0,0.99,0)));
    assert_eq!(Color::BLACK, checkered_color_at(&pattern, &point!(0,1.01,0)));

    assert_eq!(Color::WHITE, checkered_color_at(&pattern, &point!(0,0,0)));
    assert_eq!(Color::WHITE, checkered_color_at(&pattern, &point!(0,0,0.99)));
    assert_eq!(Color::BLACK, checkered_color_at(&pattern, &point!(0,0,1.01)));
}

#[test]
fn pattern_color_at_test() {
    // stripes with object transformation
    let mut sphere = Shape::sphere();
    sphere.transform = Matrix4x4::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::stripe(Color::WHITE, Color::BLACK, matrix::MATRIX_4X4_IDENTITY).unwrap();
    let c = pattern.color_at(&sphere, &point!(1.5, 0,0));
    assert_eq!(c, Color::WHITE);

    // stripes with pattern transformation
    let sphere = Shape::sphere();
    let pattern = Pattern::stripe(Color::WHITE, Color::BLACK, Matrix4x4::scaling(2.0, 2.0, 2.0)).unwrap();
    let c = pattern.color_at(&sphere, &point!(1.5, 0,0));
    assert_eq!(c, Color::WHITE);

    // stripes with both object and pattern transformation
    let mut sphere = Shape::sphere();
    sphere.transform = Matrix4x4::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::stripe(Color::WHITE, Color::BLACK, Matrix4x4::translation(0.5, 0.0, 0.0)).unwrap();
    let c = pattern.color_at(&sphere, &point!(2.5, 0,0));
    assert_eq!(c, Color::WHITE);
}