pub mod structs;
pub mod layout;
pub mod styles;
pub mod updates;

use bevy::prelude::*;

use layout::*;
use updates::*;

pub struct GUIPlug;
impl Plugin for GUIPlug {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, build_gui)
        .add_systems(Update, (
            update_shells,
            upgrade_button_pressed,
        ))

        ;
    }
}
