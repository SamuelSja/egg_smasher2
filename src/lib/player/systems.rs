

use std::f32::consts::PI;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use bevy::color::palettes::basic::BLUE;

use crate::lib::omnipresent::structs::{Gravitatable, Phy};
use crate::lib::player::CAMERA_DIST_LIMIT;
use crate::lib::upgrades::base::UpgradeInfo;
use crate::lib::upgrades::Upgrade;

use super::structs::{MainCamera, Player};

use super::{CAMERA_LIMIT, CAMERA_MOVE_SPEED, JUMP_POWER, PLAYER_SPEED};

pub fn spawn_player (
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    coms.spawn((
        Player {},
        Phy::default(),
        Gravitatable {},
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 5.5, 0.0),
    ));

    coms.spawn((
        MainCamera { 
            vert: PI / 4.0,
            horez: PI / 8.0,
            dist: 8.0,
        },
        Camera3d::default(),
        Transform::from_xyz(5.0, 4.0, 6.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}


pub fn move_player (
    mut player_q: Query<&mut Transform, With<Player>>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    upgrade_info: Res<UpgradeInfo>,
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

        // arg. time is 0 because it should have no effect on speed
        player.translation += dir * PLAYER_SPEED * time.delta_secs() * Upgrade::Speed.effect(upgrade_info.upgrades[Upgrade::Speed as usize], None).expect("Speed not dependent on time");
    }
}

pub fn camera_follow (
    mut camera_q: Query<(&mut Transform, &MainCamera)>,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {

    if let (Ok((mut camera_transform, camera)), Ok(player_transform)) = (camera_q.get_single_mut(), player_q.get_single()) {

        let a = camera.vert.cos();

        let delta = camera.dist * Vec3::new(camera.horez.sin() * a, camera.vert.sin(), camera.horez.cos() * a);

        camera_transform.translation = player_transform.translation + delta;


        camera_transform.rotation = camera_transform.looking_at(player_transform.translation, Vec3::Y).rotation;

    }
}

pub fn camera_movement (
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut evr_cursor: EventReader<CursorMoved>,

    mut camera_q: Query<&mut MainCamera>,

) {
    if let Ok(mut camera) = camera_q.get_single_mut() {
        for event in evr_cursor.read() {
            if mouse_buttons.pressed(MouseButton::Right) {
                if let Some(delta) = event.delta {
                    let delta_rad = delta * CAMERA_MOVE_SPEED;

                    camera.horez = (camera.horez - delta_rad.x + 2.0 * PI) % (2.0 * PI);


                    camera.vert += delta_rad.y;
                    if camera.vert > CAMERA_LIMIT {
                        camera.vert = CAMERA_LIMIT
                    }

                    if camera.vert < -CAMERA_LIMIT {
                        camera.vert = -CAMERA_LIMIT
                    }
                }
            }
        }
    }
}

pub fn camera_zoom (
    mut camera_q: Query<&mut MainCamera>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    if let Ok(mut camera) = camera_q.get_single_mut() {
        for event in evr_scroll.read() {
            camera.dist -= event.y;
        }
    }
}

pub fn restrict_camera_zoom (
    mut camera_q: Query<&mut MainCamera>,
) {
    if let Ok(mut camera) = camera_q.get_single_mut() {
        if camera.dist < CAMERA_DIST_LIMIT.0 {
            camera.dist = CAMERA_DIST_LIMIT.0;
        } else if camera.dist > CAMERA_DIST_LIMIT.1 {
            camera.dist = CAMERA_DIST_LIMIT.1
        }
    }
}


pub fn jump (
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut Phy, With<Player>>,
) {

    if let Ok(mut phy) = player_q.get_single_mut() {
        if keyboard.just_pressed(KeyCode::Space) && phy.grounded {
            phy.vel.y += JUMP_POWER;
        }
    }
}