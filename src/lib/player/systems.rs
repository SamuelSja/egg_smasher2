

use bevy::prelude::*;

use bevy::color::palettes::basic::BLUE;

use super::structs::{MainCamera, Player};

use super::PLAYER_SPEED;

pub fn spawn_player (
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    coms.spawn((
        Player {},
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    coms.spawn((
        MainCamera {},
        Camera3d::default(),
        Transform::from_xyz(5.0, 4.0, 6.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}


pub fn move_player (
    mut player_q: Query<&mut Transform, With<Player>>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {

    if let (Ok(camera), Ok(mut player)) = (camera_q.get_single(), player_q.get_single_mut()) {

        let mut dir = Vec3::ZERO;

        if button_input.pressed(KeyCode::KeyW) {
            dir += camera.forward().as_vec3();
        }
        if button_input.pressed(KeyCode::KeyS) {
            dir += camera.back().as_vec3();
        } 
        if button_input.pressed(KeyCode::KeyD) {
            dir += camera.right().as_vec3();
        }
        if button_input.pressed(KeyCode::KeyA) {
            dir += camera.left().as_vec3();
        }

        dir.y = 0.0;

        dir = dir.normalize_or_zero();


        player.translation += dir * PLAYER_SPEED * time.delta_secs();


    }





}