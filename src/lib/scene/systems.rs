


use bevy::{color::palettes::css::GREEN, prelude::*};



pub fn spawn_ground(
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    coms.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 2.0, 10.0))),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));


}
