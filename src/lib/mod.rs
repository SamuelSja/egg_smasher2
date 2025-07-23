
pub mod eggs;
pub mod player;
pub mod scene;
pub mod other;
pub mod helper;
pub mod gui;
pub mod upgrades;
pub mod testing;
pub mod omnipresent;

use bevy::prelude::*;
use eggs::EggPlug;
use gui::GUIPlug;
use player::PlayerPlug;
use scene::ScenePlug;
use omnipresent::OmniPlug;

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
        OmniPlug
    ))
    .init_resource::<UpgradeInfo>();

    #[cfg(debug_assertions)]
    app.add_plugins(TestingPlug);

    app.run();

}


