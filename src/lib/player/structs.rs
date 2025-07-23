

use std::ops::Range;

use bevy::prelude::*;
use rand::random_range;

#[derive(Component)]
pub struct Player {
        
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


