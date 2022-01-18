use std::path::Path;
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use serde::{Serialize, Deserialize};
use std::ops::{Add, Sub, Mul, Neg};
use image::GenericImageView;
use image;
mod point;
mod vector;
use crate::point::Point;
use crate::vector::Vector3;


const GAMMA: f32 = 1.5;


fn encode_gamma(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn decode_gamma(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,

}

impl Ray{
    pub fn create_prime_old(x:u32, y:u32, scene: &Scene) -> Ray{
        Ray{
            origin: Point::zero(),
            direction: Vector3::zero(),
        }
    }
    pub fn create_prime(x: u32, y:u32, scene: &Scene) -> Ray{
        assert!(scene.width > scene.height);
        let fov_adj = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio;
        let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;

        Ray{
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }

    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DirectionalLight{
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PointLight{
    pub pos: Point,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Light{
    Directional(DirectionalLight),
    Point(PointLight),
}

impl Light{
    pub fn color(&self) -> &Color {
        match *self {
            Light::Directional(ref d) => &d.color,
            Light::Point(ref p) => &p.color,
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32{
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Point(ref p) => {
                let r2 = (p.pos - *hit_point).norm() as f32;
                p.intensity / (4.0 * ::std::f32::consts::PI *r2)
            }
            
        }
    }

    pub fn direction(&self, hit_point: &Point) -> Vector3{
        let zero: Vector3 = Vector3::zero();
        match *self {
            Light::Directional(ref d) => (zero-d.direction).normalize(),
            Light::Point(ref p) => (p.pos - *hit_point).normalize(),
        }
    }

    pub fn distance(&self, hit_point: &Point) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Point(ref p) => (p.pos - *hit_point).length(),
        }
    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Color{
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl Color{
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (encode_gamma(self.red) * 255.0) as u8,
            (encode_gamma(self.green) * 255.0) as u8,
            (encode_gamma(self.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}
impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Plane{
    pub center: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Element{
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color{
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p) => p.albedo,
        }
    }
}


pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,

    _secret: (),
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            element: element,
            _secret: (),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, hit_point: &Point) -> Vector3;
}

impl Intersectable  for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3{
        match *self {
            Element::Sphere(ref s) => s.surface_normal(&hit_point),
            Element::Plane(ref p) => p.surface_normal(&hit_point),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);

        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        let zero: Vector3 = Vector3::zero();
        zero-self.normal
    }
}

impl Intersectable for Sphere {

    fn intersect(&self, ray: &Ray) -> Option<f64>{
        
        let origin_center: Vector3 = self.center - ray.origin;

        let ray_direction = origin_center.dot(&ray.direction);

        let center_ray = origin_center.dot(&origin_center) - (ray_direction * ray_direction);

        let radius = self.radius * self.radius;

        if center_ray > radius {
            return None;
        }

        let thicc = (radius - center_ray).sqrt();
        let thicc_ray = ray_direction - thicc;
        let ray_thicc = ray_direction + thicc;

        if thicc_ray < 0.0 && ray_thicc < 0.0 {
            return None;
        }

        let distance = if thicc_ray < ray_thicc {thicc_ray} else {ray_thicc};
        Some(distance)

    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3{
        (*hit_point - self.center).normalize()
    }
}

pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov: f64,

    pub elements: Vec<Element>,

    pub lights: Vec<Light>,
    pub bias: f64,
}
impl Scene {
    pub fn trace (&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color{
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let zero: Vector3 = Vector3::zero();

    
    let mut combined_color = Color{
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };



    for light in &scene.lights{
        let direction_light = light.direction(&hit_point);


        let shadow_ray = Ray{
            origin: hit_point + (surface_normal * scene.bias),
            direction: direction_light,
        };


        let shadow_intersection = scene.trace(&shadow_ray);

        let mut distance:f64 = 0.0;

        if !shadow_intersection.is_none(){
            distance = shadow_intersection.as_ref().unwrap().distance;
        }
        
        let is_not_shadow = shadow_intersection.is_none() || distance > light.distance(&hit_point);

        let new_light_intensity = if is_not_shadow {light.intensity(&hit_point)} else{0.0};


        let light_power = (surface_normal.dot(&direction_light) as f32).max(0.0) * new_light_intensity;

        let reflected_light = intersection.element.albedo() / std::f32::consts::PI;

        let color = light.color().clone() * light_power * reflected_light;

        combined_color = combined_color + (intersection.element.color().clone() * color);

    }
    
    combined_color.clamp()

}

pub fn render_scene(scene: &Scene) -> DynamicImage {

    
    let none = Rgba::from_channels(0,0,0,0);
    let mut output = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {

            let ray = Ray::create_prime(x,y,scene);
            let intersection = scene.trace(&ray);
            let color = intersection.map(|i| Color::to_rgba(&get_color(scene, &ray, &i)))
                .unwrap_or(none);
            output.put_pixel(x, y, color);
        }
    }
    output
}

#[test]
fn test_can_renderScene_scene(){
    let scene = Scene{
        width: 800,
        height: 600,
        fov: 90.0, 
        elements: vec![Element::Sphere(Sphere{  //Green ball
            center: Point {
                x:0.0,
                y:0.0,
                z:-5.0,

            },
            radius: 1.0,
            color: Color{
                red: 0.2,
                green: 0.8,
                blue: 0.2,

            },
            albedo: 1.0,

        }), Element::Sphere(Sphere{   //Small Yellow ball
            center: Point {
                x:2.0,
                y:1.0,
                z:-5.0,

            },
            radius: 0.5,
            color: Color{
                red: 0.8,
                green: 0.8,
                blue: 0.2,
            },
            albedo: 1.0,

        }),Element::Sphere(Sphere{  //Red ball
            center: Point {
                x:-2.0,
                y:2.0,
                z:-5.0,

            },
            radius: 2.0,
            color: Color{
                red: 0.8,
                green: 0.2,
                blue: 0.2,

            },
            albedo: 1.0,

        }),Element::Plane(Plane{     //Plane
            center: Point {
                x: 0.0,
                y: 5.0,
                z: -6.0,

            },
            normal: Vector3{
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            color: Color{
                red: 0.2,
                green: 0.2,
                blue: 0.2,

            },
            albedo: 1.0,

        })],
        lights: vec![Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 1.0,
                y: -0.25,
                z: -0.5,
            },
            color: Color{
                red: 0.8,
                green: 0.8,
                blue: 0.8,
            },
            intensity: 1.0,
        }),Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 0.0,
                y: 0.1,
                z: 0.0,
            },
            color: Color{
                red: 0.1,
                green: 0.8,
                blue: 0.1,
            },
            intensity: 0.1,
        }),Light::Point(PointLight{
            pos: Point{
                x: -2.0,
                y: 2.0,
                z:-5.0,
            },
            color: Color{
                red: 0.9,
                green: 0.2,
                blue: 0.1,
            },
            intensity: 10.0,
        })],
        bias:0.1,
    };

    let img: DynamicImage = render_scene(&scene);



    println!("hej");
    
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
    assert_eq!(scene.lights.len(), 3);


    DynamicImage::save(&img, &Path::new("image.png"));
    
}



fn main() {

    let scene = Scene{
        width: 800,
        height: 600,
        fov: 90.0, 
        elements: vec![Element::Sphere(Sphere{  //Green ball
            center: Point {
                x:0.0,
                y:0.0,
                z:-5.0,

            },
            radius: 1.0,
            color: Color{
                red: 0.2,
                green: 0.8,
                blue: 0.2,

            },
            albedo: 1.0,

        }), Element::Sphere(Sphere{   //Small Yellow ball
            center: Point {
                x:2.0,
                y:1.0,
                z:-5.0,

            },
            radius: 0.5,
            color: Color{
                red: 0.8,
                green: 0.8,
                blue: 0.2,
            },
            albedo: 1.0,

        }),Element::Sphere(Sphere{  //Red ball
            center: Point {
                x:-2.0,
                y:2.0,
                z:-5.0,

            },
            radius: 2.0,
            color: Color{
                red: 0.8,
                green: 0.2,
                blue: 0.2,

            },
            albedo: 1.0,

        }),Element::Plane(Plane{     //Plane
            center: Point {
                x: 0.0,
                y: 5.0,
                z: -6.0,

            },
            normal: Vector3{
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            color: Color{
                red: 0.2,
                green: 0.2,
                blue: 0.2,

            },
            albedo: 1.0,

        })],
        lights: vec![Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 1.0,
                y: -0.25,
                z: -0.5,
            },
            color: Color{
                red: 0.8,
                green: 0.8,
                blue: 0.8,
            },
            intensity: 1.0,
        }),Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 0.0,
                y: 0.1,
                z: 0.0,
            },
            color: Color{
                red: 0.1,
                green: 0.8,
                blue: 0.1,
            },
            intensity: 0.1,
        }),Light::Point(PointLight{
            pos: Point{
                x: 1.0,
                y: 2.0,
                z:-5.0,
            },
            color: Color{
                red: 0.9,
                green: 0.2,
                blue: 0.1,
            },
            intensity: 10.0,
        })],
        bias:0.1,
    };

    let img: DynamicImage = render_scene(&scene);


    DynamicImage::save(&img, &Path::new("image.png"));


}


