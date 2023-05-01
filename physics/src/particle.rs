use math::matrix::*;

pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
    pub force: Vec3,

    pub damping: f32,
    inv_mass: f32,
}

impl Particle {
    pub fn new(pos: Vec3, mass: f32) -> Self {
        assert_ne!(mass, 0.0);

        Self {
            pos,
            vel: Vec3::zeros(),
            acc: Vec3::zeros(),
            force: Vec3::zeros(),
            damping: 0.0,
            inv_mass: 1.0 / mass,
        }
    }

    pub fn step(&mut self, duration: f32) {
        assert!(duration > 0.0);

        self.acc = self.force * self.inv_mass;
        self.vel += self.acc * duration;
        self.vel += self.vel.normalize() * -self.damping.powf(duration);
        self.pos += self.vel * duration + self.acc * duration * duration * 0.5;

        self.force = Vec3::zeros();
    }

    pub fn set_mass(&mut self, mass: f32) {
        assert_ne!(mass, 0.0);
        self.inv_mass = 1.0 / mass;
    }

    pub fn add_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn inv_mass(&self) -> f32 {
        self.inv_mass
    }

    pub fn mass(&self) -> f32 {
        1.0 / self.inv_mass
    }

    pub fn is_inf_mass(&self) -> bool {
        self.inv_mass == 0.0
    }
}
