use math::{matrix::*, precision::Real};

pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
    pub force: Vec3,

    pub damping: Real,
    inv_mass: Real,
}

impl Particle {
    pub fn new(pos: Vec3, mass: Real) -> Self {
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

    pub fn step(&mut self, duration: Real) {
        assert!(duration > 0.0);

        self.acc = self.force * self.inv_mass;
        self.vel += self.acc * duration;
        self.vel += self.vel.normalize() * -self.damping.powf(duration);
        self.pos += self.vel * duration + self.acc * duration * duration * 0.5;

        self.force = Vec3::zeros();
    }

    pub fn set_mass(&mut self, mass: Real) {
        assert_ne!(mass, 0.0);
        self.inv_mass = 1.0 / mass;
    }

    pub fn add_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn inv_mass(&self) -> Real {
        self.inv_mass
    }

    pub fn mass(&self) -> Real {
        1.0 / self.inv_mass
    }

    pub fn is_inf_mass(&self) -> bool {
        self.inv_mass == 0.0
    }
}
