


use bevy::{color::palettes::css::{GREEN, RED, WHITE}, prelude::*};

use super::structs::Solid;



pub fn spawn_ground(
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    coms.spawn((
        Solid {},
        Mesh3d(meshes.add(Cuboid::new(10.0, 2.0, 10.0))),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));


}





pub fn spawn_light (
    mut coms: Commands,

) {
    coms.spawn((
        SpotLight {
            intensity: 100_000_000.0,
            color: WHITE.into(),
            shadows_enabled: true,
            range: 1000.0,
            ..default()
        },
        Transform::from_xyz(30.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}