use math::matrix::*;

use crate::{force_generator::*, particle::Particle};

pub type ParticleUpdate<T> = fn(u32, &mut Particle, f32, &mut T);

struct ParticleWithID(u32, Particle);

pub struct World {
    force_generators: Vec<Box<dyn ForceGeneratable>>,
    particles: Vec<ParticleWithID>,
}

impl World {
    pub fn new() -> Self {
        Self {
            force_generators: vec![Box::new(GravityGenerator::new(Vec3::from_xyz(
                0.0, 9.8, 0.0,
            )))],
            particles: Vec::new(),
        }
    }

    pub fn add_force_generator(&mut self, g: Box<dyn ForceGeneratable>) {
        self.force_generators.push(g);
    }

    pub fn create_particle<'a>(&'a mut self, id: u32, pos: Vec3, mass: f32) {
        self.particles
            .push(ParticleWithID(id, Particle::new(pos, mass)));
    }

    pub fn update<T>(&mut self, duration: f32, update: ParticleUpdate<T>, param: &mut T) {
        for particle in &mut self.particles {
            update(particle.0, &mut particle.1, duration, param);
        }

        for generator in &self.force_generators {
            for particle in &mut self.particles {
                generator.update_force(&mut particle.1, duration);
            }
        }

        for particle in &mut self.particles {
            particle.1.step(duration);
        }
    }
}
