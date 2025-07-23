pub mod structs;
pub mod systems;

use bevy::prelude::*;

use systems::*;

use crate::lib::player::systems::camera_follow;


pub const GRAVITY: f32 = 20.0;

pub struct OmniPlug;
impl Plugin for OmniPlug {
    fn build(&self, app: &mut App) {
        app

        .add_systems(Update, (
            restrict_phy.before(camera_follow),
            apply_phy.before(camera_follow).before(restrict_phy),
            gravity.before(apply_phy),
            static_friction,
        ))

        ; 
    }
}
