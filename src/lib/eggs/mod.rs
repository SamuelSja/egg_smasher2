
pub mod structs;
pub mod systems;

use std::time::Duration;

use bevy::prelude::*;
use structs::EggGenerationInfo;
use systems::*;



pub const EGG_PERIOD: Duration = Duration::new(1, 0);
pub const EGG_SPAWN_POS: Vec2 = Vec2::new(0.0, 0.0);
pub const EGG_SPAWN_SIZE: Vec2 = Vec2::new(10.0, 10.0);


pub struct EggPlug;
impl Plugin for EggPlug {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EggGenerationInfo>()
        .add_systems(Update, (
            smash_egg,
            random_egg_generation,
            egg_particle_despawn,
        ))
        ;
    }
}

