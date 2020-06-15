use crate::color::Color;
use crate::tuple::Point;
use crate::tuple::Vector;
use crate::material::Material;
use crate::material::Pattern;
use crate::shape::Shape;
use crate::point;
use crate::vector;
use crate::rgb;
use crate::matrix;

pub struct Light {
    pub position: Point,
    pub intensity: Color
}

impl Light {
    pub fn point_light(position: Point, intensity: Color) -> Light {
        Light {
            position,
            intensity
        }
    }
}

pub fn lighting(
    material: &Material,
    shape: &Shape,
    light: &Light,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
    in_shadow: bool) -> Color {
    
    let color = match &material.pattern {
        Some(pattern) => pattern.color_at(&shape, &point),
        None => material.color.clone()
    };

    let effective_color = color.hadamard(&light.intensity);

    // vector to the light from the position
    let light_vector = light.position.subtract(&point).normalize();

    // ambient is applied generally regardless of direct light
    let ambient = effective_color.multiplyf(material.ambient);
    
    // light_dot_normal represents the cosine of the angle
    // between the two vectors, negative means the light is
    // on the other side of the surface
    let light_dot_normal = light_vector.dot(&normalv);

    let diffuse : Color;
    let specular : Color;

    if in_shadow || light_dot_normal < 0.0 {
        diffuse = Color::BLACK;
        specular = Color::BLACK;
    }
    else {
        diffuse = effective_color.multiplyf(material.diffuse * light_dot_normal);

        // compute the cosine of the angle between the reflection vector
        // and the eye vector, negative means the light reflects away from the eye
        let reflect = light_vector.negate().reflect(&normalv);
        let reflect_dot_eye = reflect.dot(&eyev);
        if reflect_dot_eye <= 0.0 {
            specular = Color::BLACK;
        }
        else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity.multiplyf(material.specular * factor);
        }
    }
    return ambient.add(&diffuse).add(&specular);
}

#[test]
fn lighting_test_eye_between_light_and_surface() {
     let sphere = Shape::sphere();
     let material = Material::new();
    let position = point!(0,0,0);
    let eye = vector!(0,0,-1);
    let normal = vector!(0,0,-1);
    let light = Light::point_light(point!(0,0,-10), rgb!(1,1,1));
    let in_shadow = false;
    let result = lighting(&material, &sphere, &light, &position, &eye, &normal, in_shadow);
    assert_eq!(result, rgb!(1.9,1.9,1.9));
}

#[test]
fn lighting_test_eye_offset_between_light_and_surface() {
     let sphere = Shape::sphere();
     let material = Material::new();
    let position = point!(0,0,0);
    let eye = vector!(0, 2_f32.sqrt()/2.0, -2_f32.sqrt()/2.0);
    let normal = vector!(0,0,-1);
    let light = Light::point_light(point!(0,0,-10), rgb!(1,1,1));
    let in_shadow = false;
    let result = lighting(&material, &sphere, &light, &position, &eye, &normal, in_shadow);
    assert_eq!(result, rgb!(1,1,1));
}

 #[test]
 fn lighting_test_eye_opposite_surface_light_offset_45() {
     let sphere = Shape::sphere();
     let material = Material::new();
     let position = point!(0,0,0);
     let eye = vector!(0,0,-1);
     let normal = vector!(0,0,-1);
     let light = Light::point_light(point!(0,10,-10), rgb!(1,1,1));
    let in_shadow = false;
    let result = lighting(&material, &sphere, &light, &position, &eye, &normal, in_shadow);
     assert_eq!(result, rgb!(0.7364, 0.7364, 0.7364));
 }

 #[test]
 fn lighting_test_eye_in_path_of_reflection_of_light() {
     let sphere = Shape::sphere();
     let material = Material::new();
     let position = point!(0,0,0);
     let eye = vector!(0, -2_f32.sqrt()/2.0, -2_f32.sqrt()/2.0);
     let normal = vector!(0,0,-1);
     let light = Light::point_light(point!(0,10,-10), rgb!(1,1,1));
    let in_shadow = false;
    let result = lighting(&material, &sphere, &light, &position, &eye, &normal, in_shadow);
     assert_eq!(result, rgb!(1.6364, 1.6364, 1.6364));
 }

 #[test]
 fn lighting_test_light_behind_surface() {
     let sphere = Shape::sphere();
     let material = Material::new();
     let position = point!(0,0,0);
     let eye = vector!(0,0,-1);
     let normal = vector!(0,0,-1);
     let light = Light::point_light(point!(0,0,10), rgb!(1,1,1));
    let in_shadow = false;
    let result = lighting(&material, &sphere, &light, &position, &eye, &normal, in_shadow);
     assert_eq!(result, rgb!(0.1,0.1,0.1));
 }

 #[test]
 fn lighting_test_with_surface_in_shadow() {
     let sphere = Shape::sphere();
     let material = Material::new();
    let position = point!(0,0,0);
    let eye = vector!(0,0,-1);
    let normal = vector!(0,0,-1);
    let light = Light::point_light(point!(0,0,-10), rgb!(1,1,1));
    let in_shadow = true;
    let result = lighting(&material, &sphere, &light, &position, &eye, &normal, in_shadow);
    assert_eq!(result, rgb!(0.1,0.1,0.1));
 }

 #[test]
 fn lighting_test_with_stripe_pattern() {
     let sphere = Shape::sphere();
    let mut material = Material::new();
    material.pattern = Pattern::stripe(Color::WHITE, Color::BLACK, matrix::MATRIX_4X4_IDENTITY);
    material.ambient = 1.0;
    material.diffuse = 0.0;
    material.specular = 0.0;
    let eye = vector!(0,0,-1);
    let normal = vector!(0,0,-1);
    let light = Light::point_light(point!(0,0,-10), rgb!(1,1,1));
    let in_shadow = false;
    let c1 = lighting(&material, &sphere, &light, &point!(0.9,0,0), &eye, &normal, in_shadow);
    let c2 = lighting(&material, &sphere, &light, &point!(1.1,0,0), &eye, &normal, in_shadow);
    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::BLACK);
 }