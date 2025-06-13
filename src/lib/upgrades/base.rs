

use bevy::prelude::*;
use std::mem;

use super::Upgrade;

#[derive(Resource)]
pub struct UpgradeInfo {
    // upgrades: [Upgrade; mem::variant_count::<Upgrade>()],
    pub upgrades: Vec<u32>,
} 

impl Default for UpgradeInfo {
    fn default() -> Self {
        Self {
            upgrades: vec![1; mem::variant_count::<Upgrade>()],
        } 
    }
}












