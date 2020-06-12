
#[macro_export]
macro_rules! rgb {
    ($red:expr, $green:expr, $blue:expr) => {
        crate::color::Color {
            red: $red as f32,
            green: $green as f32,
            blue: $blue as f32
        }
    };
}

#[macro_export]
macro_rules! tuple {
    ($x:expr, $y:expr, $z:expr, $w:expr ) => {
        crate::tuple::Tuple {
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
        crate::tuple::Point {
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
        crate::tuple::Vector {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
            w: 0.0
        }
    };
}
