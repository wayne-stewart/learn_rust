use crate::color::Color;
use crate::tuple::Point;
use crate::tuple::Vector;
use crate::material::Material;
use crate::point;
use crate::vector;
use crate::rgb;

pub struct Light {
    position: Point,
    intensity: Color
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
    light: &Light,
    position: &Point,
    eye: &Vector,
    normal: &Vector) -> Color {
    
    let effective_color = material.color * light.intensity;

    // vector to the light from the position
    let light_vector = (light.position - (*position)).normalize();

    // ambient is applied generally regardless of direct light
    let ambient = effective_color * material.ambient;
    
    // light_dot_normal represents the cosine of the angle
    // between the two vectors, negative means the light is
    // on the other side of the surface
    let light_dot_normal = light_vector.dot(&normal);

    let mut diffuse : Color;
    let mut specular : Color;

    if light_dot_normal < 0.0 {
        diffuse = Color::BLACK;
        specular = Color::BLACK;
    }
    else {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // compute the cosine of the angle between the reflection vector
        // and the eye vector, negative means the light reflects away from the eye
        let reflect = (-light_vector).reflect(&normal);
        let reflect_dot_eye = reflect.dot(&eye);
        if reflect_dot_eye <= 0.0 {
            specular = Color::BLACK;
        }
        else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    return ambient + diffuse + specular;
}

#[test]
fn lighting_test_eye_between_light_and_surface() {
    let material = Material::new();
    let position = point!(0,0,0);
    let eye = vector!(0,0,-1);
    let normal = vector!(0,0,-1);
    let light = Light::point_light(point!(0,0,-10), rgb!(1,1,1));
    let result = lighting(&material, &light, &position, &eye, &normal);
    assert_eq!(result, rgb!(1.9,1.9,1.9));
}

#[test]
fn lighting_test_eye_offset_between_light_and_surface() {
    let material = Material::new();
    let position = point!(0,0,0);
    let eye = vector!(0, 2_f32.sqrt()/2.0, -2_f32.sqrt()/2.0);
    let normal = vector!(0,0,-1);
    let light = Light::point_light(point!(0,0,-10), rgb!(1,1,1));
    let result = lighting(&material, &light, &position, &eye, &normal);
    assert_eq!(result, rgb!(1,1,1));
}

 #[test]
 fn lighting_test_eye_opposite_surface_light_offset_45() {
     let material = Material::new();
     let position = point!(0,0,0);
     let eye = vector!(0,0,-1);
     let normal = vector!(0,0,-1);
     let light = Light::point_light(point!(0,10,-10), rgb!(1,1,1));
     let result = lighting(&material, &light, &position, &eye, &normal);
     assert_eq!(result, rgb!(0.7364, 0.7364, 0.7364));
 }

 #[test]
 fn lighting_test_eye_in_path_of_reflection_of_light() {
     let material = Material::new();
     let position = point!(0,0,0);
     let eye = vector!(0, -2_f32.sqrt()/2.0, -2_f32.sqrt()/2.0);
     let normal = vector!(0,0,-1);
     let light = Light::point_light(point!(0,10,-10), rgb!(1,1,1));
     let result = lighting(&material, &light, &position, &eye, &normal);
     assert_eq!(result, rgb!(1.6364, 1.6364, 1.6364));
 }

 #[test]
 fn lighting_test_light_behind_surface() {
     let material = Material::new();
     let position = point!(0,0,0);
     let eye = vector!(0,0,-1);
     let normal = vector!(0,0,-1);
     let light = Light::point_light(point!(0,0,10), rgb!(1,1,1));
     let result = lighting(&material, &light, &position, &eye, &normal);
     assert_eq!(result, rgb!(0.1,0.1,0.1));
 }