// TODO: This file should be replaced by reading from a data file

use physics::particle::*;

pub struct ParticleTypeDefinition {
    pub base: BasicParticleType,
    pub color: (u8, u8, u8, u8)
}

pub fn particle_types() -> Vec<ParticleTypeDefinition> {
    vec!{
        ParticleTypeDefinition {
            base: BasicParticleType {
                inertia_mass: 100.0,
                gravity_mass: 100.0,
                d0: 2.0,
                hardness: 1.0
            },
            color: (173, 200, 206, 128),
        },
        ParticleTypeDefinition {
            base: BasicParticleType {
                inertia_mass: 50.0,
                gravity_mass: 50.0,
                d0: 4.0,
                hardness: 1.0
            },
            color: (206, 0, 0, 128),
        },
        ParticleTypeDefinition {
            base: BasicParticleType {
                inertia_mass: 50.0,
                gravity_mass: 50.0,
                d0: 4.0,
                hardness: 1.0,
            },
            color: (0, 206, 0, 128),
        },
    }
}
