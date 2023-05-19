use math::{matrix::*, precision::Real};
use graphics::ppm::PPM;
use geometric::{geom3d::*, intersect3d::ray_sphere_intersect_param};
use rand::Rng;

const CANVA_WIDTH: usize = 200;
const CANVA_HEIGHT: usize = 100;

struct Camera {
    position: Vec3,
    mat: Mat44,
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self { position, mat: Mat44::identity() }
    }

    pub fn lookat(&mut self, target: Vec3) {
        self.mat = math::cg::lookat(self.position, target, Vec3::y_axis());
    }
}

#[derive(Copy, Clone, Debug)]
struct HitResult<'a> {
    pub pt: Vec3,
    pub normal: Vec3,
    pub t: Real,
    pub material: &'a Material,
}

#[derive(Copy, Clone, Debug)]
struct Material {
    color: Vec3,
}

struct Shape {
    sphere: Sphere,
    material: Material,
}

impl Shape {
    pub fn new(sphere: Sphere, material: Material) -> Self {
        Self { sphere, material }
    }
}

trait Hitable {
    fn hit(&self, ray: &Ray3D) -> Option<HitResult>;
}

struct World {
    shapes: Vec<Box<dyn Hitable>>,
}

impl World {
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }

    pub fn add(&mut self, shape: Box<dyn Hitable>) {
        self.shapes.push(shape);
    }

    pub fn hit(&self, ray: &Ray3D) -> Option<HitResult> {
        let mut param: Option<HitResult> = None;

        for shape in &self.shapes {
            if let Some(hit) = shape.hit(ray) {
                match &mut param {
                    Some(o) => {
                        if hit.t <= o.t {
                            *o = hit;
                        }
                    },
                    None => param = Some(hit),
                }
            }
        }

        param
    }
}

impl Hitable for Shape {
    fn hit(&self, ray: &Ray3D) -> Option<HitResult> {
        let params = ray_sphere_intersect_param(&ray, &self.sphere);
        if let Some((t, _)) = params {
            let pt = ray.start + ray.dir * t;
            Some(HitResult{ pt , normal: (pt - self.sphere.center) / self.sphere.radius, t, material: &self.material })
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3::from_xyz(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if v.length_sqrd() <= 1.0 {
            return v;
        }
    }
}

fn fill_background(ppm: &mut PPM, x: usize, y: usize, v: Real) {
    let color = lerp(Vec3::from_xyz(0.5, 0.7, 1.0), Vec3::from_xyz(1.0, 1.0, 1.0), v);
    ppm.set_pixel(x, y, color);
}

const MAX_RECURSE: u32 = 1000;

fn ray_color(ray: &Ray3D, world: &World, count: u32) -> Vec3 {
    if let Some(hit) = world.hit(ray) {
        if count == 0 {
            return Vec3::ones();
        } else {
            let pt_in_sphere = random_in_unit_sphere();
            let target = hit.pt + hit.normal + pt_in_sphere;
            let ray = Ray3D::new(hit.pt, target - hit.pt);
            return ray_color(&ray, world, count - 1).mul_each(hit.material.color);
        }
    } else {
        return Vec3::ones();
    }
}

fn main() {
    let mut ppm = PPM::new(CANVA_WIDTH, CANVA_HEIGHT);
    let sphere = Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5);
    let horizontal = Vec3::from_xyz(4.0, 0.0, 0.0);
    let vertical = Vec3::from_xyz(0.0, 2.0, 0.0);
    let origin = Vec3::from_xyz(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::from_xyz(-2.0, -1.0, -1.0);

    let mut world = World::new();
    world.add(Box::new(Shape::new(sphere, Material { color: Vec3::from_xyz(0.1, 0.4, 0.2) })));

    for y in (0..CANVA_HEIGHT).rev() {
        for x in 0..CANVA_WIDTH {
            let u = x as Real / CANVA_WIDTH as Real;
            let v = (CANVA_HEIGHT - y - 1) as Real / CANVA_HEIGHT as Real;

            let ray = Ray3D::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let color = ray_color(&ray, &world, MAX_RECURSE);
            ppm.set_pixel(x, y, color);
            // fill_background(&mut ppm, x, y, v);
        }
    }
    ppm.write_to_file("result.ppm").unwrap();
}