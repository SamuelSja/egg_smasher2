
use bevy::prelude::*;

use crate::lib::{gui::structs::UpgradeButton, player::structs::Shells, upgrades::base::UpgradeInfo};

use super::structs::ShellsLabel;


pub fn update_shells (
    mut label_q: Query<&mut Text, With<ShellsLabel>>,
    shells: Res<Shells>,

) {
    for mut label in label_q.iter_mut() {
        label.0 = format!("Shells: {}", shells.val);
    }
}


pub fn upgrade_button_pressed (
    mut shells: ResMut<Shells>,
    mut upgrade_info: ResMut<UpgradeInfo>,
    mut buttons: Query<(&UpgradeButton, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (upgrade_button, interaction, mut background_color) in buttons.iter_mut() {
        match &interaction {
            Interaction::None => {
                background_color.0 = upgrade_button.background_color;
            },
            Interaction::Hovered => {
                let mut hovered_color = upgrade_button.background_color.to_linear();
                hovered_color.red -= 0.1;
                hovered_color.green -= 0.1;
                hovered_color.blue -= 0.1;

                background_color.0 = Color::from(hovered_color);
            },
            Interaction::Pressed => {
                let mut hovered_color = upgrade_button.background_color.to_linear();
                hovered_color.red += 0.1;
                hovered_color.green += 0.1;
                hovered_color.blue += 0.1;


                let upgrade = upgrade_button.upgrade;
                let level = upgrade_info.upgrades[upgrade as usize];
                let cost = upgrade.next_cost(level);
                if shells.val >= cost {
                    println!("buy");
                    shells.val -= cost;
                    upgrade_info.upgrades[upgrade as usize] += 1;
                } else {
                    println!("not enough shells");
                }



            }
        }
    }
}

