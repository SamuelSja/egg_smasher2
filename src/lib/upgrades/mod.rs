
pub mod base;



use std::error::Error;

use bevy::prelude::*;
use num_enum::TryFromPrimitive;





#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Upgrade {
    Speed,
    Damage,
}

impl Upgrade {
    /// Gives the cost of the upgrade based off the current level
    pub fn next_cost(&self, count: u32) -> u32 {
        match self {
            Self::Speed => {
                10 * 10_u32.pow(count)
            },
            Self::Damage => {
                10 * 2_u32.pow(count)
            },
        }
    }

    /// Gives the effect of the upgrade at the given level and time.
    /// 
    /// Time is in seconds. Returns an error if time is not given and time is necessary
    pub fn effect(&self, count: u32, time: Option<f32>) -> Result<f32, Box<dyn Error>> {
        match self {
            Self::Speed => {
                Ok((1.0 / 5.0) * count as f32 + 1.0)
            },
            Self::Damage => {
                Ok(1.0 * (1.5_f32.powf(count as f32 - 1.0)))
            },
        }
    }

    /// Give the description of the upgrade
    pub fn description(&self, count: u32) -> String {
        match self {
            Self::Speed => {
                format!("increaces your speed")
            },
            Self::Damage => {
                format!("increaces your damage")
            },
        }
    }

    /// Gives the name of the upgrade
    pub fn name(&self) -> String {
        String::from(match self {
            Self::Speed => {
                "Speed"       
            },
            Self::Damage => {
                "Damage"
            },
        })
    }
}

