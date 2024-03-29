use crate::particle::Particle;
use math::{matrix::*, precision::Real};

pub trait ForceGeneratable {
    fn update_force(&self, p: &mut Particle, duration: Real);
}

pub struct GravityGenerator {
    gravity: Vec3,
}

impl GravityGenerator {
    pub fn new(gravity: Vec3) -> Self {
        Self { gravity }
    }
}

impl ForceGeneratable for GravityGenerator {
    fn update_force(&self, p: &mut Particle, duration: Real) {
        if p.is_inf_mass() {
            return;
        }

        p.add_force(self.gravity * p.mass());
    }
}

pub struct SpringForceGenerator<'a> {
    other: Option<&'a Particle>,
    pub k: Real,
    pub static_length: Real,
}

impl<'a> SpringForceGenerator<'a> {
    pub fn new(other: &'a Particle, k: Real, static_length: Real) -> Self {
        Self {
            other: Some(other),
            k,
            static_length,
        }
    }
}

impl<'a> ForceGeneratable for SpringForceGenerator<'a> {
    fn update_force(&self, p: &mut Particle, duration: Real) {
        if let Some(other) = self.other {
            let dir = other.pos - p.pos;
            let magnitude = dir.length() - self.static_length;
            p.add_force(dir.normalize() * magnitude);
        }
    }
}

pub struct AnchoredSpringForceGenerator {
    anchor: Vec3,
    pub k: Real,
    pub static_length: Real,
}

impl AnchoredSpringForceGenerator {
    pub fn new(anchor: Vec3, k: Real, static_length: Real) -> Self {
        Self {
            anchor,
            k,
            static_length,
        }
    }
}

impl ForceGeneratable for AnchoredSpringForceGenerator {
    fn update_force(&self, p: &mut Particle, duration: Real) {
        let dir = self.anchor - p.pos;
        let magnitude = dir.length() - self.static_length;
        p.add_force(dir.normalize() * magnitude);
    }
}
