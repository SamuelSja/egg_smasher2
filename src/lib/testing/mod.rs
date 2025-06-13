use bevy::prelude::*;

use crate::lib::player::structs::Shells;





pub struct TestingPlug;
impl Plugin for TestingPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, give_shells)
        ;
    }
}


pub fn give_shells (
    mut shells: ResMut<Shells>,
) {
    shells.val = 1_000_000;
}

