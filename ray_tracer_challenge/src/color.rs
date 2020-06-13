use crate::math::fequal;
use crate::rgb;

#[derive(Debug, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

impl Color {
    pub fn rgb (r:f32, g:f32, b:f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }

    pub const BLACK : Color = rgb!(0,0,0);

    pub fn add(&self, rhs: &Color) -> Color {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }

    pub fn subtract(&self, rhs: &Color) -> Color {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue
        }
    }

    pub fn multiplyf(&self, rhs: f32) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        }
    }

    pub fn multiply(&self, rhs: &Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        }
    }
}

/*
    This allows to use == and != when comparing two tuples
*/
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let a = fequal(self.red, other.red);
        let b = fequal(self.green, other.green);
        let c = fequal(self.blue, other.blue);
        return a && b && c;
    }
}

#[test]
fn add_test() {
    let a = Color::rgb(0.9,0.6,0.75);
    let b = Color::rgb(0.7,0.1,0.25);
    let c = a.add(&b);
    let d = Color::rgb(1.6,0.7,1.0);
    assert_eq!(c,d);
}

#[test]
fn subtract_test() {
    let a = Color::rgb(0.9,0.6,0.75);
    let b = Color::rgb(0.7,0.1,0.25);
    let c = a.subtract(&b);
    let d = Color::rgb(0.2,0.5,0.5);
    assert_eq!(c,d);
}

#[test]
fn multiply_scalar_test() {
    let a = Color::rgb(0.2,0.3,0.4);
    let b = a.multiplyf(2.0);
    let c = Color::rgb(0.4,0.6,0.8);
    assert_eq!(b,c);
}

#[test]
fn mutliply_hadamard_test() {
    let a = Color::rgb(1.0, 0.2, 0.4);
    let b = Color::rgb(0.9, 1.0, 0.1);
    let c = a.multiply(&b);
    let d = Color::rgb(0.9,0.2,0.04);
    assert_eq!(c,d);
}