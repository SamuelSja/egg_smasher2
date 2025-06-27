
use std::time::Duration;

use bevy::prelude::*;

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





