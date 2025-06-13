
pub mod structs;
pub mod systems;



use std::f32::consts::PI;

use bevy::prelude::*;
use structs::Shells;
use systems::*;


pub const PLAYER_SPEED: f32 = 5.0;
pub const GRAVITY: f32 = 20.0;
pub const CAMERA_DIST: f32 = 8.0;
pub const CAMERA_MOVE_SPEED: f32 = 0.01; // rad / px
pub const CAMERA_LIMIT: f32 = (7.0 / 16.0) * PI; // rad
pub const JUMP_POWER: f32 = 10.0;


pub struct PlayerPlug;
impl Plugin for PlayerPlug {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Shells>()
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            jump.before(apply_yvel),
            move_player.before(camera_follow).before(restrict_movement),
            apply_yvel.before(camera_follow).before(restrict_movement),
            restrict_movement.before(camera_follow),
            gravity.before(apply_yvel),
            camera_movement.before(camera_follow),
            camera_follow,
            camera_zoom,


        ))
        ;        
    }
}


