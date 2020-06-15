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
mod camera;

use camera::Camera;
use world::World;
use canvas::Canvas;
use light::Light;
use shape::Shape;
use matrix::Matrix4x4;
use material::Pattern;

/*
view ppm files
http://paulcuth.me.uk/netpbm-viewer/


*/

fn main() {
    let dimx = 500;
    let dimy = 250;
    let mut canvas = Canvas::new(dimx, dimy);
    let mut camera = Camera::new(dimx, dimy, std::f32::consts::PI / 3.0);
    camera.transform = Matrix4x4::view_transform(&point!(0,1.5,-5), &point!(0,1,0), &point!(0,1,0));

    let world = create_world();

    render(&camera, &world, &mut canvas);

    let s = canvas::to_ppm(&canvas);
    std::fs::write("test.ppm", s).expect("unable to write file");
}

fn create_world() -> World {
    let mut world = World::new();

    let light = Light::point_light(point!(-10,10,-10), rgb!(1,1,1));
    world.lights.push(light);

    let mut plane = Shape::plane();
    plane.material.pattern = Pattern::stripe(rgb!(1,0.5,0.5), rgb!(0.5,1,0.5), matrix::MATRIX_4X4_IDENTITY);
    world.objects.push(plane);

    let mut plane = Shape::plane();
    plane.material.pattern = Pattern::checkered(rgb!(1,0.5,0.5), rgb!(0.5,1,0.5), matrix::MATRIX_4X4_IDENTITY);
    plane.material.specular = 0.0;
    plane.transform = Matrix4x4::rotation_x(std::f32::consts::PI / 2.0)
        .multiply(&Matrix4x4::translation(0.0,10.0,0.0));
    world.objects.push(plane);

    let mut sphere = Shape::sphere();
    sphere.transform = Matrix4x4::translation(-0.5, 1.0, 0.5);
    let pattern_transform = Matrix4x4::scaling(0.1,0.1,0.1)
        .multiply(&Matrix4x4::rotation_z(std::f32::consts::PI / 4.0))
        .multiply(&Matrix4x4::rotation_y(std::f32::consts::PI / 4.0));
    sphere.material.pattern = Pattern::stripe(rgb!(0.1,0.1,1), rgb!(1,1,1), pattern_transform);
    sphere.material.color = rgb!(0.1,0.1,1);
    sphere.material.diffuse = 0.7;
    sphere.material.specular = 0.3;
    world.objects.push(sphere);

    let mut sphere = Shape::sphere();
    sphere.transform = Matrix4x4::translation(1.5, 0.5, -0.5).multiply(&Matrix4x4::scaling(0.5,0.5,0.5));
    sphere.material.color = rgb!(0.5,1,0.1);
    sphere.material.diffuse = 0.7;
    sphere.material.specular = 0.3;
    world.objects.push(sphere);

    return world;
}

fn render(camera: &Camera, world: &World, canvas: &mut Canvas) {
    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray);
            canvas.set_pixel(x, y, &color);
        }
    }
}