

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
        
}

#[derive(Component)]
pub struct YVel {
    pub vel: f32,
}

impl Default for YVel {
    fn default() -> Self {
        Self {
            vel: 0.0,
        } 
    }
}



#[derive(Component)]
pub struct MainCamera {
    pub dir: Vec3,
}
