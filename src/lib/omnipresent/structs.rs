

use std::ops::Range;

use bevy::prelude::*;

#[derive(Component)]
pub struct Solid;


#[derive(Component)]
pub struct Phy {
    pub vel: Vec3,
    pub grounded: bool, 
}

impl Phy {
    pub fn new(vel: Vec3, grounded: bool) -> Self {
        Self {
            vel,
            grounded,
        }
    }

    pub fn random_range(ranges: (Range<f32>, Range<f32>, Range<f32>)) -> Self {
        let vel = Vec3::new(rand::random_range(ranges.0), rand::random_range(ranges.1), rand::random_range(ranges.2));

        Self::new(vel, false)
    }
}

impl Default for Phy {
    fn default() -> Self {
        Self::new(Vec3::ZERO, false) 
    }
}

#[derive(Component)]
pub struct Gravitatable;

#[derive(Component)]
pub struct Friction;