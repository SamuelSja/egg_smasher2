

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
        
}

#[derive(Component)]
pub struct YVel {
    pub vel: f32,
    pub grounded: bool,
}

impl Default for YVel {
    fn default() -> Self {
        Self {
            grounded: false,
            vel: 0.0,
        } 
    }
}



#[derive(Component)]
pub struct MainCamera {
    pub vert: f32,
    pub horez: f32,
    pub dist: f32,
}

#[derive(Resource)]
pub struct Shells {
    pub val: u32,
}

impl Default for Shells {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


