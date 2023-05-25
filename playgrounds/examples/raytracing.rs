use geometric::{geom3d::*, intersect3d::ray_sphere_intersect_param};
use graphics::ppm::PPM;
use math::{matrix::*, precision::Real};
use rand::Rng;

const CANVA_WIDTH: usize = 200;
const CANVA_HEIGHT: usize = 100;

struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub near: Real,
    pub width: Real,
    pub height: Real,

    front: Vec3,
    right: Vec3,
    up: Vec3,
    low_left_corner: Vec3,
}

impl Camera {
    pub fn new(near: Real, width: Real, height: Real) -> Self {
        Self {
            position: Vec3::zeros(),
            target: Vec3::from_xyz(0.0, 0.0, -1.0),
            near,
            width,
            height,
            front: Vec3::from_xyz(0.0, 0.0, -near),
            up: Vec3::from_xyz(0.0, height, 0.0),
            right: Vec3::from_xyz(width, 0.0, 0.0),
            low_left_corner: Vec3::from_xyz(-width * 0.5, -height * 0.5, -near),
        }
    }

    pub fn lookat(&mut self, target: Vec3) {
        self.target = target;
        self.calc_lookat();
    }

    pub fn move_to(&mut self, pos: Vec3) {
        self.position = pos;
        self.calc_lookat();
    }

    pub fn front(&self) -> &Vec3 {
        &self.front
    }

    pub fn right(&self) -> &Vec3 {
        &self.right
    }

    pub fn up(&self) -> &Vec3 {
        &self.up
    }

    pub fn lower_left_corner(&self) -> &Vec3 {
        &self.low_left_corner
    }

    fn calc_lookat(&mut self) {
        let up = Vec3::from_xyz(0.0, 1.0, 0.0);
        let front = (self.target - self.position).normalize();
        let right = front.cross(&up);
        let up = right.cross(&front);

        self.up = up * self.height;
        self.right = right * self.width;
        self.front = front * self.near;

        self.low_left_corner = -self.right * 0.5 - self.up * 0.5 + self.front;
    }

    pub fn get_ray(&self, u: Real, v: Real) -> Ray3D {
        let dir = self.low_left_corner + *self.right() * u + *self.up() * v;
        Ray3D::new(self.position, dir)
    }
}

type Scatter = Box<dyn Fn() -> Option<Ray3D>>;

trait Scattering {
    fn scatter(&self, ray_in: &Ray3D, hit: &HitResult) -> (Vec3, Ray3D);
}

#[derive(Copy, Clone)]
struct HitResult<'a> {
    pub pt: Vec3,
    pub normal: Vec3,
    pub t: Real,
    pub material: &'a dyn Scattering,
}

struct Shape {
    sphere: Sphere,
    scatter: Box<dyn Scattering>,
}

impl Shape {
    pub fn new(sphere: Sphere, scatter: Box<dyn Scattering>) -> Self {
        Self { sphere, scatter }
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
                    }
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
            Some(HitResult {
                pt,
                normal: (pt - self.sphere.center) / self.sphere.radius,
                t,
                material: self.scatter.as_ref(),
            })
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let a = rng.gen_range(0.0..2.0 * math::consts::PI);
        let z = rng.gen_range(-1.0..1.0);
        let r = (1.0f32 - z * z).sqrt();
        return Vec3::from_xyz(r * a.cos(), r * a.sin(), z);
    }
}

const MAX_RECURSE: u32 = 50;
const SAMPLE_COUNT: u32 = 100;

fn ray_color(ray: &Ray3D, world: &World, count: u32) -> Vec3 {
    let unit_direction = random_in_unit_sphere();
    if let Some(hit) = world.hit(ray) {
        if count == 0 {
            return Vec3::zeros();
        }
        let (ambient, ray) = hit.material.scatter(&ray, &hit);
        ray_color(&ray, world, count - 1).mul_each(ambient)
    } else {
        Vec3::ones()
    }
}

struct Lambertian {
    color: Vec3,
}

impl Scattering for Lambertian {
    fn scatter(&self, ray_in: &Ray3D, hit: &HitResult) -> (Vec3, Ray3D) {
        let unit_direction = random_in_unit_sphere();
        let target = hit.pt + hit.normal + unit_direction;
        let ray = Ray3D::new(hit.pt, target - hit.pt);
        (self.color, ray)
    }
}

struct Metal {
    color: Vec3,
}

impl Scattering for Metal {
    fn scatter(&self, ray_in: &Ray3D, hit: &HitResult) -> (Vec3, Ray3D) {
        let ray = Ray3D::new(hit.pt, math::cg::reflect(-ray_in.dir, hit.normal));
        (self.color, ray)
    }
}

fn main() {
    let mut ppm = PPM::new(CANVA_WIDTH, CANVA_HEIGHT);
    let mut camera = Camera::new(1.0, 4.0, 2.0);
    camera.move_to(Vec3::from_xyz(0.0, 1.0, 1.0));

    let mut world = World::new();
    world.add(Box::new(Shape::new(
        Sphere::new(Vec3::from_xyz(0.0, 0.0, -1.0), 0.5),
        Box::new(Lambertian {
            color: Vec3::from_xyz(0.1, 0.4, 0.2),
        }),
    )));
    world.add(Box::new(Shape::new(
        Sphere::new(Vec3::from_xyz(0.0, -100.5, -1.0), 100.0),
        Box::new(Lambertian {
            color: Vec3::from_xyz(0.5, 0.5, 0.9),
        }),
    )));
    world.add(Box::new(Shape::new(
        Sphere::new(Vec3::from_xyz(-1.0, 0.0, -1.0), 0.5),
        Box::new(Metal {
            color: Vec3::from_xyz(0.8, 0.8, 0.8),
        }),
    )));

    let mut rng = rand::thread_rng();

    for y in (0..CANVA_HEIGHT).rev() {
        for x in 0..CANVA_WIDTH {
            println!(
                "processing : {}/{}",
                (CANVA_HEIGHT - y - 1) * CANVA_WIDTH + x,
                CANVA_WIDTH * CANVA_HEIGHT
            );
            let mut color = Vec3::zeros();
            for _ in 0..SAMPLE_COUNT {
                let u = (x as Real + rng.gen_range(0.0..1.0)) / CANVA_WIDTH as Real;
                let v = ((CANVA_HEIGHT - y - 1) as Real - rng.gen_range(0.0..1.0))
                    / CANVA_HEIGHT as Real;

                color += ray_color(&camera.get_ray(u, v), &world, MAX_RECURSE);
            }

            color /= SAMPLE_COUNT as Real;
            color[0] = color.x().sqrt();
            color[1] = color.y().sqrt();
            color[2] = color.z().sqrt();

            ppm.set_pixel(x, y, color);
        }
    }
    ppm.write_to_file("result.ppm").unwrap();
}
