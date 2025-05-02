
pub mod structs;
pub mod systems;



use bevy::prelude::*;
use systems::*;


pub const PLAYER_SPEED: f32 = 5.0;


pub struct PlayerPlug;
impl Plugin for PlayerPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            move_player,
        ))
        ;
    }
}


