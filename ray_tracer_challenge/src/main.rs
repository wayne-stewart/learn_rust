mod math;
mod tuple;
mod color;
mod canvas;
use std::io::prelude::*;

fn main() {
    let mut canvas = canvas::create_canvas(5, 3);
    let c1 = color::rgb(1.5, 0.0, 0.0);
    let c2 = color::rgb(0.0, 0.5, 0.0);
    let c3 = color::rgb(-0.5, 0.0, 1.0);
    canvas::set_pixel(&mut canvas, 0, 0, c1);
    canvas::set_pixel(&mut canvas, 4, 2, c3);
    canvas::set_pixel(&mut canvas, 2, 1, c2);
    let s = canvas::to_ppm(&canvas);
    //println!("{}", s);
    std::fs::write("test.ppm", s).expect("unable to write file");
}
