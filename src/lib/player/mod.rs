
pub mod structs;
pub mod systems;



use std::f32::consts::PI;

use bevy::prelude::*;
use systems::*;


pub const PLAYER_SPEED: f32 = 5.0;
pub const GRAVITY: f32 = 1.0;
pub const CAMERA_DIST: f32 = 8.0;
pub const CAMERA_MOVE_SPEED: f32 = 0.01; // rad / px
pub const CAMERA_LIMIT: f32 = (7.0 / 16.0) * PI; // rad


pub struct PlayerPlug;
impl Plugin for PlayerPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            move_player.before(camera_follow).before(restrict_movement),
            apply_yvel.before(camera_follow).before(restrict_movement),
            restrict_movement.before(camera_follow),
            gravity,
            camera_movement.before(camera_follow),
            camera_follow,

        ))
        ;
    }
}


