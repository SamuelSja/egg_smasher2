use bevy::prelude::*;

use crate::lib::upgrades::Upgrade;



#[derive(Component)]
pub struct ShellsLabel;

#[derive(Component)]
pub struct UpgradeLabel {
    pub upgrade: Upgrade,
}

impl UpgradeLabel {
    pub fn new(upgrade: Upgrade) -> Self {
        Self {
            upgrade,
        }
    }
}

#[derive(Component)]
pub struct UpgradeButton {
    pub upgrade: Upgrade,
    pub background_color: Color,
}

impl UpgradeButton {
    pub fn new(upgrade: Upgrade) -> Self {
        Self {
            upgrade,
            background_color: Color::linear_rgb(0.5, 0.5, 0.5),
        }
    }
}

