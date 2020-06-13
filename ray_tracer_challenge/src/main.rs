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
mod world;

/*
view ppm files
http://paulcuth.me.uk/netpbm-viewer/


*/

fn main() {
    let dim = 300;
    let dimf = dim as f32;
    let mut canvas = canvas::create_canvas(dim, dim);

    let mut sphere = shape::Sphere::new();
    let ray_origin = point!(0,0,-5);
    let scale = matrix::Matrix4x4::scaling(1.0, 1.0, 1.0);
    sphere.transform = scale;
    sphere.material.color = rgb!(1, 0.2, 0.5);

    let light = light::Light::point_light(point!(-10, -10, -10), rgb!(1,1,1));

    for y in 0..dim {
        for x in 0..dim {
            let cx = ((x as f32) - dimf / 2.0) * 0.007;
            let cy = ((y as f32) - dimf / 2.0) * 0.007;
            let ray = ray::Ray::new(ray_origin.clone(), vector!(cx,cy,5).normalize());
            match ray::hit(sphere.intersects(&ray)) {
                None => canvas::set_pixel(&mut canvas, x, y, &color::Color::BLACK),
                Some(t) => {
                    let hit_point = ray.position(t);
                    let normal = sphere.normal_at(&hit_point);
                    let eye = ray.direction.negate();
                    let color = light::lighting(&sphere.material, &light, &hit_point, &eye, &normal);
                    canvas::set_pixel(&mut canvas, x, y, &color);
                }
            }
        }
    }

    let s = canvas::to_ppm(&canvas);
    std::fs::write("test.ppm", s).expect("unable to write file");
}

