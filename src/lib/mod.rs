
pub mod eggs;
pub mod player;
pub mod scene;
pub mod other;
pub mod helper;
pub mod gui;
pub mod upgrades;
pub mod testing;

use bevy::prelude::*;
use eggs::EggPlug;
use gui::GUIPlug;
use player::PlayerPlug;
use scene::ScenePlug;

#[cfg(debug_assertions)]
use crate::lib::testing::TestingPlug;
use crate::lib::upgrades::base::UpgradeInfo;


pub fn start() {
    let mut app = App::new();
    app
    .add_plugins((
        DefaultPlugins,
        EggPlug,
        PlayerPlug,
        ScenePlug,
        GUIPlug,
    ))
    .init_resource::<UpgradeInfo>();

    #[cfg(debug_assertions)]
    app.add_plugins(TestingPlug);

    app.run();

}


