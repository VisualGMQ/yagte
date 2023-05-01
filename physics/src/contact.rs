use math::matrix::Vec3;

use crate::particle::Particle;

pub struct ParticleContact<'a> {
    pub particle: (&'a Particle, Option<&'a Particle>),
    pub restitution: f32,
    pub normal: Vec3,
    pub depth: f32,
}

impl<'a> ParticleContact<'a> {
    pub fn new(
        p1: &'a Particle,
        p2: Option<&'a Particle>,
        restitution: f32,
        normal: Vec3,
        depth: f32,
    ) -> Self {
        Self {
            particle: (p1, p2),
            restitution,
            normal,
            depth,
        }
    }

    pub fn resolve(&mut self, duration: f32) {}
}
