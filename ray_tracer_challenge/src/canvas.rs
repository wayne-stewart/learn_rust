use crate::color::Color;

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let pixel_count: usize = (width * height) as usize;
        let mut pixels = Vec::<Color>::with_capacity(pixel_count);
        let black = Color::rgb(0.0,0.0,0.0);
        for _index in 0..pixel_count {
            pixels.push(black.clone());
        }
        Canvas {
            width,
            height,
            pixels
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        let index = (y * self.width + x) as usize;
        self.pixels[index] = color.clone();
    }
    
    pub fn get_pixel(&mut self, x: u32,  y: u32) -> Color {
        let index = (y * self.width + x) as usize;
        self.pixels[index].clone()
    }
}



fn clamp_255(v: f32) -> i32 {
    let mut a: i32 = (255.0 * v) as i32;
    if a < 0 {
        a = 0;
    }
    else if a > 255 {
        a = 255;
    }

    return a;
}

pub fn to_ppm(canvas : &Canvas) -> String {
    let mut v = Vec::<char>::new();
    v.push('P');
    v.push('3');
    v.push('\n');
    for c in canvas.width.to_string().chars() {
        v.push(c);
    }
    v.push(' ');
    for c in canvas.height.to_string().chars() {
        v.push(c);
    }
    v.push('\n');
    v.push('2');
    v.push('5');
    v.push('5');
    v.push('\n');

    let max_pixels_in_line = 5;
    for y in 0..canvas.height {
        let mut pixels_in_line = 0;
        for x in 0..canvas.width {
            if pixels_in_line > max_pixels_in_line {
                pixels_in_line = 0;
                v.push('\n');
            }
            if pixels_in_line > 0 {
                v.push(' ');
            }
            pixels_in_line += 1;
            let index = (y * canvas.width + x) as usize;
            let c = &canvas.pixels[index];
            let r = clamp_255(c.red);
            let g = clamp_255(c.green);
            let b = clamp_255(c.blue);
            for d in r.to_string().chars() {
                v.push(d);
            }
            v.push(' ');
            for d in g.to_string().chars() {
                v.push(d);
            }
            v.push(' ');
            for d in b.to_string().chars() {
                v.push(d);
            }
        }
        v.push('\n');
    }
    v.push('\n');

    let s: String = v.into_iter().collect();
    s
}

#[test]
fn create_canvas_test() {
    let a = create_canvas(10, 20);
    let pixel_count: usize = (a.width * a.height) as usize;
    let black = Color::rgb(0.0,0.0,0.0);
    assert_eq!(a.width, 10);
    assert_eq!(a.height, 20);
    for index in 0..pixel_count {
        assert_eq!(a.pixels[index], black);
    }
}

#[test]
fn set_get_pixel_test() {
    let mut canvas = create_canvas(10, 20);
    let red = Color::rgb(1.0,0.0,0.0);
    let green = Color::rgb(0.0,1.0,0.0);
    let blue = Color::rgb(0.0,0.0,1.0);
    set_pixel(&mut canvas, 0,0, &red);
    set_pixel(&mut canvas, 9, 19, &green);
    set_pixel(&mut canvas, 0, 19, &blue);
    let r = get_pixel(&canvas, 0,0);
    let g = get_pixel(&canvas, 9, 19);
    let b = get_pixel(&canvas, 0, 19);
    assert_eq!(red, r);
    assert_eq!(green, g);
    assert_eq!(blue, b);
}

#[test]
fn to_ppm_test() {
    let canvas = create_canvas(5, 3);
    let s = to_ppm(&canvas);
    println!("{}",s);
}