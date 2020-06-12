mod math;
mod macro_def;
mod tuple;
mod color;
mod canvas;
mod matrix;
mod ray;
mod shape;
mod light;
mod material;

/*
view ppm files
http://paulcuth.me.uk/netpbm-viewer/


*/

fn main() {
    let mut canvas = canvas::create_canvas(100, 100);

    let mut sphere = shape::Sphere::new(1);
    let black = color::Color::rgb(0.0,0.0,0.0);
    let red = color::Color::rgb(1.0,0.0,0.0);
    let ray_origin = point!(0,0,-5);
    let scale = matrix::Matrix4x4::scaling(4.0, 2.0, 4.0);
    sphere.transform = scale;

    for y in 0..100 {
        for x in 0..100 {
            let cx = ((x as f32) - 50.0) * 0.2;
            let cy = ((y as f32) - 50.0) * 0.2;
            let ray = ray::Ray::new(ray_origin, vector!(cx,cy,5));
            match ray::hit(sphere.intersects(&ray)) {
                None => canvas::set_pixel(&mut canvas, x, y, black),
                Some(_t) => canvas::set_pixel(&mut canvas, x, y, red)
            }
        }
    }

    let s = canvas::to_ppm(&canvas);
    std::fs::write("test.ppm", s).expect("unable to write file");
}
