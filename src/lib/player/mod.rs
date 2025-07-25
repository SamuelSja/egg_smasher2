
pub mod structs;
pub mod systems;



use std::f32::consts::PI;

use bevy::prelude::*;
use structs::Shells;
use systems::*;

use crate::lib::omnipresent::systems::{apply_phy, restrict_phy};

// use crate::lib::scene::systems::{apply_yvel, restrict_yvel};


pub const PLAYER_SPEED: f32 = 5.0;
pub const CAMERA_DIST: f32 = 8.0;
pub const CAMERA_MOVE_SPEED: f32 = 0.01; // rad / px
pub const CAMERA_LIMIT: f32 = (7.0 / 16.0) * PI; // rad
pub const JUMP_POWER: f32 = 10.0;
pub const CAMERA_DIST_LIMIT: (f32, f32) = (2.0, 16.0);


pub struct PlayerPlug;
impl Plugin for PlayerPlug {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Shells>()
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            jump.before(apply_phy),
            move_player.before(camera_follow).before(restrict_phy),
            camera_movement.before(camera_follow),
            camera_follow,
            camera_zoom,
            restrict_camera_zoom.after(camera_zoom),
        ))
        ;        
    }
}


