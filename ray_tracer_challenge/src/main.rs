mod math;
mod tuple;
mod color;
mod canvas;
mod matrix;
mod ray;

use crate::tuple::Tuple;
// use crate::point;
// use crate::vector;

/*
view ppm files
http://paulcuth.me.uk/netpbm-viewer/


*/

fn main() {
    let mut canvas = canvas::create_canvas(100, 100);

    let mut sphere = ray::Sphere::new(1);
    let black = color::Color::rgb(0.0,0.0,0.0);
    let red = color::Color::rgb(1.0,0.0,0.0);
    let ray_origin = point!(0,0,-5);
    let translate = matrix::Matrix4x4::translation(50.0, 50.0, 0.0);
    let scale = matrix::Matrix4x4::scaling(4.0, 2.0, 4.0);
    sphere.transform = scale;

    for y in 0..100 {
        for x in 0..100 {
            let cx = ((x as f32) - 50.0) * 0.2;
            let cy = ((y as f32) - 50.0) * 0.2;
            let ray = ray::Ray::new(ray_origin, vector!(cx,cy,5));
            match ray::hit(sphere.intersects(&ray)) {
                None => canvas::set_pixel(&mut canvas, x, y, black),
                Some(t) => canvas::set_pixel(&mut canvas, x, y, red)
            }
        }
    }
    
    // let p = point!(0,0,0);
    // let v = vector!(35,0,0);
    // let transform_to_center = matrix::Matrix4x4::translation(50.0, 50.0, 0.0);
    // let transform_to_circle = matrix::Matrix4x4::translation(35.0, 0.0, 0.0);
    // for i in 0..12 {
    //     let rotate = matrix::Matrix4x4::rotation_z(std::f32::consts::PI / 6.0 * (i as f32));
    //     let pixel = transform_to_center * rotate * transform_to_circle * p;
    //     canvas::set_pixel(&mut canvas, pixel.x as u32, pixel.y as u32, color);
    // }

    let s = canvas::to_ppm(&canvas);
    std::fs::write("test.ppm", s).expect("unable to write file");
}
