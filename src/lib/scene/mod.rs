
pub mod structs;
pub mod systems;



use bevy::prelude::*;

use systems::*;



pub struct ScenePlug;
impl Plugin for ScenePlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (
            spawn_ground,
            spawn_light,
        ))
        ;
    }
}


