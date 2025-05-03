
pub mod structs;
pub mod systems;

use bevy::prelude::*;
use systems::*;




pub struct EggPlug;
impl Plugin for EggPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_test_egg)
        .add_systems(Update, (
            smash_egg,
        ))
        ;
    }
}

