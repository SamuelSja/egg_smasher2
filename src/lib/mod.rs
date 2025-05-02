
pub mod eggs;
pub mod player;
pub mod scene;
pub mod other;
pub mod helper;

use bevy::prelude::*;
use eggs::EggPlug;
use player::PlayerPlug;
use scene::ScenePlug;


pub fn start() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        EggPlug,
        PlayerPlug,
        ScenePlug,
    ))
    .run();

}


