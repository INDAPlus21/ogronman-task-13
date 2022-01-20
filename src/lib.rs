#![crate_type = "cdylib"]
extern crate jni;
use std::path::Path;
use std::path::PathBuf;
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use serde::{Serialize, Deserialize};
use std::ops::{Add, Sub, Mul, Neg};
use image::GenericImageView;
use image;
mod point;
mod vector;
use crate::point::Point;
use crate::vector::Vector3;
use image::io::Reader as ImageReader;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use std::io::{BufRead, BufReader};
use std::fs::File;

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

    pub fn create_reflection(normal: Vector3, collision: Vector3, col_point: Point, bias: f64) -> Ray{
        Ray {
            origin: col_point + (normal*bias),
            direction: collision - (2.0 * collision.dot(&normal) * normal),
        }
    }

}

pub struct DirectionalLight{
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct PointLight{
    pub pos: Point,
    pub color: Color,
    pub intensity: f32,
}

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


#[derive(Debug)]
pub struct Texture_thing{
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub enum Texture {
    color(Color),
    image(DynamicImage),
}

impl Texture {
    pub fn get_color(&self, coords: &Texture_thing) -> Color {
        match *self {
            Texture::color(ref c) => c.clone(),
            Texture::image(ref img) => {

                let img_x = wrap(coords.x, img.width());
                let img_y = wrap(coords.y, img.height());

                Color::from_rgba(img.get_pixel(img_x, img_y))

            }
        }
    }
}

fn wrap(val: f32, bound: u32) -> u32 {
    let s_bound = bound as i32;
    let f_coord = val * bound as f32;
    let wrap_coord = (f_coord as i32) % s_bound;
    if wrap_coord < 0 {
        (wrap_coord + s_bound) as u32
    } else {
        wrap_coord as u32
    }
}

#[derive(Clone, Debug)]
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

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {        
        
        Color{
            red: (rgba[0] as f32) / 255.0,
            green: (rgba[1] as f32) / 255.0,
            blue: (rgba[2] as f32) / 255.0,
        }

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

#[derive(Debug)]
pub enum surface_type{
    Diffuse,
    Reflective { reflectivity: f32},    // 0 equals no reflectivity, 1 equals 100% reflectivity
}

#[derive(Debug)]
pub struct Material{
    pub color: Texture,
    pub albedo: f32,

    pub surface: surface_type,

}


#[derive(Debug)]
pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}


#[derive(Debug)]
pub struct Plane{
    pub center: Point,
    pub normal: Vector3,
    pub material: Material,
}


#[derive(Debug)]
pub enum Element{
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn material(&self) -> &Material{
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
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

    fn texture_coords(&self, hit_point: &Point) -> Texture_thing;
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

    fn texture_coords(&self, hit_point: &Point) -> Texture_thing{
        match *self {
            Element::Sphere(ref s) => s.texture_coords(&hit_point),
            Element::Plane(ref p) => p.texture_coords(&hit_point),
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

    fn texture_coords(&self, hit_point: &Point) -> Texture_thing{

        let mut x_axis = self.normal.cross(&Vector3{
            x:0.0,
            y:0.0,
            z:1.0,
        });

        if x_axis.length() == 0.0{
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        
        let y_axis = self.normal.cross(&x_axis);
        
        let direction_hit = *hit_point - self.center;

        Texture_thing{
            x: direction_hit.dot(&x_axis) as f32,
            y: direction_hit.dot(&y_axis) as f32,
        }
        
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

    fn texture_coords(&self, hit_point: &Point) -> Texture_thing{
        let direction_hit = *hit_point - self.center;

        Texture_thing{
            x: (1.0 + (direction_hit.z.atan2(direction_hit.x) as f32) / std::f32::consts::PI) * 0.5,
            y: (direction_hit.y / self.radius).acos() as f32 / std::f32::consts::PI,
        }

    }
}

pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov: f64,

    pub elements: Vec<Element>,

    pub lights: Vec<Light>,
    pub bias: f64,

    pub max_rec: u32,
}
impl Scene {
    pub fn trace (&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

fn diffuse_shading(scene: &Scene, ray: &Ray, intersection: &Intersection, hit_point: Point, surface_normal: Vector3) -> Color{

    let texture_coords = intersection.element.texture_coords(&hit_point);
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

        let material = intersection.element.material();

        let light_power = (surface_normal.dot(&direction_light) as f32).max(0.0) * new_light_intensity;

        let reflected_light = material.albedo / std::f32::consts::PI;

        let color = light.color().clone() * light_power * reflected_light;

        combined_color = combined_color + (material.color.get_color(&texture_coords) * color);

    }
    
    combined_color.clamp()

}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color{
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let texture_coords = intersection.element.texture_coords(&hit_point);
    let zero: Vector3 = Vector3::zero();

    
    let mut combined_color = diffuse_shading(scene, ray, intersection, hit_point, surface_normal);

    if let surface_type::Reflective {reflectivity} = intersection.element.material().surface{
        let reflection = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.bias);

        combined_color = combined_color * (1.0-reflectivity);
        combined_color = combined_color + (raycast(scene, &reflection, depth + 1) * reflectivity);
    }

    combined_color

}

pub fn raycast(scene: &Scene, ray: &Ray, depth: u32) -> Color{

    let none = Color{
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    if depth >= scene.max_rec {
        return none
    }

    let intersection = scene.trace(&ray);

    intersection.map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(none)
}

pub fn render_scene(scene: &Scene) -> DynamicImage {

    
    let none = Rgba::from_channels(0,0,0,0);
    let mut output = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {

            let ray = Ray::create_prime(x,y,scene);
            output.put_pixel(x, y, raycast(scene, &ray, 0).to_rgba());
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
            material: Material{
                color: Color{
                    red: 0.2,
                    green: 0.8,
                    blue: 0.2,
    
                },
                albedo: 1.0,
            }

        }), Element::Sphere(Sphere{   //Small Yellow ball
            center: Point {
                x:2.0,
                y:1.0,
                z:-5.0,

            },
            radius: 0.5,
            material: Material{
                color: Color{
                    red: 0.8,
                    green: 0.8,
                    blue: 0.2,
                },
                albedo: 1.0,
            }

        }),Element::Sphere(Sphere{  //Red ball
            center: Point {
                x:-2.0,
                y:2.0,
                z:-5.0,

            },
            radius: 2.0,
            material: Material{
                color: Color{
                    red: 0.8,
                    green: 0.2,
                    blue: 0.2,
    
                },
                albedo: 1.0,
            }

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
            material: Material{
                color: Color{
                    red: 0.2,
                    green: 0.2,
                    blue: 0.2,
    
                },
                albedo: 1.0,
            }

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


    DynamicImage::save(&img, &Path::new("../image.png"));
    
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
    assert_eq!(scene.lights.len(), 3);


    DynamicImage::save(&img, &Path::new("../image.png"));
    
}

#[no_mangle]
pub extern "system" fn Java_main_hello(_env: JNIEnv, _class: JClass){
    println!("Hello world");
}

#[no_mangle]
pub extern fn Java_main_javaCall(){

    let reader = BufReader::new(File::open("inf.txt").expect("Cannot open inf.txt"));

    let mut element_vec: Vec<Element> = Vec::new();

    for line in reader.lines() {

        let words:Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();

        if(words[0].eq("sphere")){
            let new_element: Element = Element::Sphere(Sphere{
                center: Point {
                    x: words[1].parse::<f64>().unwrap(),
                    y: words[2].parse::<f64>().unwrap(),
                    z: words[3].parse::<f64>().unwrap(),
                },
                radius: words[4].parse::<f64>().unwrap(),
                material: Material {
                    color: Texture::color(Color{
                        red: words[5].parse::<f32>().unwrap() / 255.0,
                        green: words[6].parse::<f32>().unwrap() / 255.0,
                        blue: words[7].parse::<f32>().unwrap() / 255.0,
                    }),
                    albedo: 1.0,
                    surface: surface_type::Reflective{reflectivity: 0.1},
                },
            });
            element_vec.push(new_element);
        }
        
    }

    let scene2 = Scene{
        width: 1250,
        height: 1000,
        fov: 90.0,
        elements: element_vec,
        lights: vec![Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 0.5,
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
                y: 1.0,
                z: 0.1,
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
        }),Light::Point(PointLight{
            pos: Point{
                x: 0.0,
                y: 0.0,
                z:-1.0,
            },
            color: Color{
                red: 0.1,
                green: 0.8,
                blue: 0.1,
            },
            intensity: 5.0,
        })],
        bias:0.1,
        max_rec: 10, 
    };
    
    let scene = Scene{
        width: 1250,
        height: 1000,
        fov: 90.0, 
        elements: vec![Element::Sphere(Sphere{  //Green ball
            center: Point {
                x:0.0,
                y:0.0,
                z:-5.0,

            },
            radius: 1.0,
            material: Material{
                color: Texture::color(Color{
                    red: 0.2,
                    green: 0.8,
                    blue: 0.2,
    
                }),
                albedo: 1.0,
                surface: surface_type::Reflective{reflectivity: 0.1},
            }

        }), Element::Sphere(Sphere{   //Small Yellow ball
            center: Point {
                x:2.0,
                y:1.0,
                z:-5.0,

            },
            radius: 0.5,
            material: Material{
                color: Texture::color(Color{
                    red: 0.8,
                    green: 0.8,
                    blue: 0.2,
                }),
                albedo: 1.0,
                surface: surface_type::Diffuse,
            }

        }), Element::Sphere(Sphere{   //Small amog us ball
            center: Point {
                x: -1.0,
                y: -1.0,
                z: -2.0,

            },
            radius: 0.75,
            material: Material{
                color: Texture::image(image::open("texture/test.png").unwrap()),
                albedo: 0.75,
                surface: surface_type::Reflective{reflectivity: 0.3},
            }

        }), Element::Sphere(Sphere{   //Small amog us ball
            center: Point {
                x:-1.0,
                y:-1.0,
                z: 0.5,

            },
            radius: 0.75,
            material: Material{
                color: Texture::image(image::open("texture/test.png").unwrap()),
                albedo: 0.75,
                surface: surface_type::Reflective{reflectivity: 0.0},
            }

        }),Element::Sphere(Sphere{  //Red ball
            center: Point {
                x:-2.0,
                y:2.0,
                z:-5.0,

            },
            radius: 2.0,
            material: Material{
                color: Texture::color(Color{
                    red: 0.8,
                    green: 0.2,
                    blue: 0.2,
    
                }),
                albedo: 1.0,
                surface: surface_type::Reflective{reflectivity: 1.0},
            }

        }),Element::Plane(Plane{     //Plane
            center: Point {
                x: 0.0,
                y: -2.0,
                z: -5.0,

            },
            normal: Vector3{
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            material: Material{
                //color: Texture::color(Color{
                    //red:0.8,
                    //green:0.8,
                    //blue: 0.8,
                //}),
                color: Texture::image(image::open("texture/check.png").unwrap()),
                albedo: 1.0,
                surface: surface_type::Reflective{reflectivity: 0.99},
            }

        })],
        lights: vec![Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 0.5,
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
                y: 1.0,
                z: 0.1,
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
        }),Light::Point(PointLight{
            pos: Point{
                x: 0.0,
                y: 0.0,
                z:-1.0,
            },
            color: Color{
                red: 0.1,
                green: 0.8,
                blue: 0.1,
            },
            intensity: 5.0,
        })],
        bias:0.1,
        max_rec: 10, 
    };

    let img: DynamicImage = render_scene(&scene);


    DynamicImage::save(&img, &Path::new("../image.png"));

    let img2: DynamicImage = render_scene(&scene2);


    DynamicImage::save(&img2, &Path::new("../image2.png"));


}


fn main() {

    let scene = Scene{
        width: 1250,
        height: 1000,
        fov: 90.0, 
        elements: vec![Element::Sphere(Sphere{  //Green ball
            center: Point {
                x:0.0,
                y:0.0,
                z:-5.0,

            },
            radius: 1.0,
            material: Material{
                color: Texture::color(Color{
                    red: 0.2,
                    green: 0.8,
                    blue: 0.2,
    
                }),
                albedo: 1.0,
                surface: surface_type::Reflective{reflectivity: 0.1},
            }

        }), Element::Sphere(Sphere{   //Small Yellow ball
            center: Point {
                x:2.0,
                y:1.0,
                z:-5.0,

            },
            radius: 0.5,
            material: Material{
                color: Texture::color(Color{
                    red: 0.8,
                    green: 0.8,
                    blue: 0.2,
                }),
                albedo: 1.0,
                surface: surface_type::Diffuse,
            }

        }), Element::Sphere(Sphere{   //Small amog us ball
            center: Point {
                x: -1.0,
                y: -1.0,
                z: -2.0,

            },
            radius: 0.75,
            material: Material{
                color: Texture::image(image::open("test.png").unwrap()),
                albedo: 0.75,
                surface: surface_type::Reflective{reflectivity: 0.3},
            }

        }), Element::Sphere(Sphere{   //Small amog us ball
            center: Point {
                x:-1.0,
                y:-1.0,
                z: 0.5,

            },
            radius: 0.75,
            material: Material{
                color: Texture::image(image::open("test.png").unwrap()),
                albedo: 0.75,
                surface: surface_type::Reflective{reflectivity: 0.0},
            }

        }),Element::Sphere(Sphere{  //Red ball
            center: Point {
                x:-2.0,
                y:2.0,
                z:-5.0,

            },
            radius: 2.0,
            material: Material{
                color: Texture::color(Color{
                    red: 0.8,
                    green: 0.2,
                    blue: 0.2,
    
                }),
                albedo: 1.0,
                surface: surface_type::Reflective{reflectivity: 1.0},
            }

        }),Element::Plane(Plane{     //Plane
            center: Point {
                x: 0.0,
                y: -2.0,
                z: -5.0,

            },
            normal: Vector3{
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            material: Material{
                //color: Texture::color(Color{
                    //red:0.8,
                    //green:0.8,
                    //blue: 0.8,
                //}),
                color: Texture::image(image::open("check.png").unwrap()),
                albedo: 1.0,
                surface: surface_type::Reflective{reflectivity: 0.99},
            }

        })],
        lights: vec![Light::Directional(DirectionalLight{
            direction: Vector3{
                x: 0.5,
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
                y: 1.0,
                z: 0.1,
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
        }),Light::Point(PointLight{
            pos: Point{
                x: 0.0,
                y: 0.0,
                z:-1.0,
            },
            color: Color{
                red: 0.1,
                green: 0.8,
                blue: 0.1,
            },
            intensity: 5.0,
        })],
        bias:0.1,
        max_rec: 10, 
    };

    let img: DynamicImage = render_scene(&scene);


    DynamicImage::save(&img, &Path::new("../image.png"));


}