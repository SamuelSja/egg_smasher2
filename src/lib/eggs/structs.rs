
use std::{ops::Range, time::Duration};

use bevy::prelude::*;
use rand::random_range;

use super::EGG_PERIOD;


#[derive(Component)]
pub struct Egg {
    pub vel_thresh: f32,
    pub health: f32,
    pub max_health: f32,
    pub color: Color,
    pub shells: i32,
}

impl Egg {
    pub fn health_par(&self) -> f32 {
        self.health / self.max_health
    }

    pub fn health_color(&self) -> Color {
        let par = self.health_par();

        let linear = self.color.to_linear();
        
        Color::linear_rgb(linear.red * par, linear.green * par, linear.blue * par)
    }

    pub fn new(health: f32, vel_thresh: f32, color: Color, shells: i32) -> Self {
        Self {
            vel_thresh,
            health,
            max_health: health,
            color,
            shells,
        }
    }
}

#[derive(Component)]
pub struct EggParticle {
    pub secs_left: f32,
}

impl Default for EggParticle {

    /// Creates an EggParticle with a despawn time of 10 seconds
    fn default() -> Self {
        // Self {
        //     secs_left: 10.0,
        // }

        Self::new(10.0)

    }
}

impl EggParticle {

    /// Creates a new EggParticle with the given despawn time
    pub fn new(time: f32) -> Self {
        Self {
            secs_left: time, 
        }
    }

    /// Creates a new EggParticle with a random despawn time in the given range
    pub fn rand(range: Range<f32>) -> Self {
        Self::new(random_range(range))
    }
}



#[derive(Resource)]
pub struct EggGenerationInfo {
    pub eggs: u32,
    pub timer: Timer,
}

impl Default for EggGenerationInfo {
    fn default() -> Self {
        Self {
            eggs: 0,
            timer: Timer::new(EGG_PERIOD, TimerMode::Repeating),
        }
    }
}





