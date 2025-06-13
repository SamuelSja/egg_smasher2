
use bevy::prelude::*;

use crate::lib::upgrades::{base::UpgradeInfo, Upgrade};

use super::{structs::{ShellsLabel, UpgradeButton, UpgradeLabel}, styles::*};



pub fn build_gui (
    mut coms: Commands,
    mut assets: Res<AssetServer>,
    upgrade_info: Res<UpgradeInfo>
) {
    coms.spawn((
        main_style(),
        
    )).with_children(|p| {
        p.spawn((
            side_panel_style(),
        )).with_children(|p| {
            p.spawn((
                Text::new("shells: #"),
                ShellsLabel,
            ));

            spawn_upgrades(p, &assets, &upgrade_info);

        });
    })
    ;
}


/// Spawns all upgrade labels/buttons on a &mut ChildBuilder
/// 
/// Panics when upgrade_info's upgrades len is greater than the Upgrade enum varient count
pub fn spawn_upgrades(p: &mut ChildBuilder<'_>, assets: &Res<AssetServer>, upgrade_info: &Res<UpgradeInfo>) {
    // p.spawn(Text::new("<todo(upgrade here)>"));


    for upgrade_index in 0..upgrade_info.upgrades.len() {
        let upgrade: Upgrade = Upgrade::try_from(upgrade_index as u32)
        .expect("Tried to get access to a non-excistant upgrade. Resulted in error. Check upgrade_info.upgrades.len() and the Upgrades enum.");

        // let level = upgrade_info.upgrades[upgrade_index];


        spawn_upgrade(p, assets, upgrade_info, upgrade);


    }




}


/// Spawns a single upgrade label/button on a &mut ChildBuilder
pub fn spawn_upgrade(p: &mut ChildBuilder<'_>, assets: &Res<AssetServer>, upgrade_info: &Res<UpgradeInfo>, upgrade: Upgrade) {
    p.spawn(upgrade_item_style()).with_children(|p| {
        p.spawn((
            Text::new(format!("upgrade info for {}", upgrade.name())),
            UpgradeLabel::new(upgrade),
        ));
        p.spawn((
            upgrade_button_style(),
            Button {},
            BackgroundColor::from(Color::linear_rgb(0.5, 0.0, 0.1)),
            UpgradeButton::new(upgrade),
        ));
    });
}
