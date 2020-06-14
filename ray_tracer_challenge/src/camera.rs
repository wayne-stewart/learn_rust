use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::point;
use crate::vector;

pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    //field_of_view: f32,
    pub transform: Matrix4x4,
    pixel_size: f32,
    half_width: f32,
    half_height: f32
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f32) -> Camera {
        let half_view = (field_of_view/2.0).tan();
        let aspect = (hsize as f32) / (vsize as f32);
        let half_width: f32;
        let half_height: f32;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }
        else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        Camera {
            hsize,
            vsize,
            //field_of_view,
            transform: crate::matrix::MATRIX_4X4_IDENTITY,
            half_width,
            half_height,
            pixel_size: half_width * 2.0 / (hsize as f32)
        }
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        // offset from the edge of the canvas to the pixel center
        let xoffset = (x as f32 + 0.5) * self.pixel_size;
        let yoffset = (y as f32 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space
        // the camera looks toward -z so +x is to the left;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // transform the canvas point and the origin
        // then compute the ray's direction vector
        // the canvas is set at z -1 from the camera
        let inverse = self.transform.inverse();
        let pixel = inverse.multiply_tuple(&point!(world_x, world_y, -1));
        let origin = inverse.multiply_tuple(&point!(0,0,0));
        let direction = pixel.subtract(&origin).normalize();
        Ray::new(origin, direction)
    }
}

#[test]
fn camera_test() {
    let half_pi = std::f32::consts::PI / 2.0;
    let quarter_pi = std::f32::consts::PI / 4.0;

    let camera = Camera::new(200, 125, half_pi);
    assert_eq!(0.01, camera.pixel_size);

    let camera = Camera::new(125, 200, half_pi);
    assert_eq!(0.01, camera.pixel_size);

    let camera = Camera::new(201, 101, half_pi);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, point!(0,0,0));
    assert_eq!(ray.direction, vector!(0,0,-1));

    let ray = camera.ray_for_pixel(0,0);
    assert_eq!(ray.origin, point!(0,0,0));
    assert_eq!(ray.direction, vector!(0.66519, 0.33259, -0.66851));

    let mut camera = Camera::new(201, 101, half_pi);
    camera.transform = Matrix4x4::rotation_y(quarter_pi).multiply(&Matrix4x4::translation(0.0, -2.0, 5.0));
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, point!(0, 2, -5));
    assert_eq!(ray.direction, vector!(2_f32.sqrt()/2.0, 0, -2_f32.sqrt()/2.0));
}