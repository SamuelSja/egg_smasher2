
pub mod structs;
pub mod systems;



use bevy::prelude::*;

use systems::*;

use crate::lib::player::systems::camera_follow;


pub struct ScenePlug;
impl Plugin for ScenePlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (
            spawn_ground,
            spawn_light,
        ))
        .add_systems(Update, (
            apply_yvel.before(camera_follow).before(restrict_yvel),
            restrict_yvel.before(camera_follow),
            gravity.before(apply_yvel),
        ))
        ;
    }
}


