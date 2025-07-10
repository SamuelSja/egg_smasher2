


use bevy::{color::palettes::css::{GREEN, RED, WHITE}, prelude::*, render::primitives::Aabb};

use crate::lib::{helper::restrict_transform_movement, player::{structs::YVel, GRAVITY}};

use super::structs::Solid;



pub fn spawn_ground(
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    coms.spawn((
        Solid {},
        Mesh3d(meshes.add(Cuboid::new(40.0, 2.0, 40.0))),
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

/// blocks YVels from hitting things
pub fn restrict_yvel (
    mut player_q: Query<(&mut Transform, &Aabb, &mut YVel)>,
    solids_q: Query<(&Transform, &Aabb), (With<Solid>, Without<YVel>)>
) {
    for (mut transform, aabb, mut y_vel) in player_q.iter_mut() {
        for (solid_transform, solid_aabb) in solids_q.iter() {
            
            let mut player_size: Vec3 = aabb.half_extents.into();
            player_size *= 2.0;

            let mut solid_size: Vec3 = solid_aabb.half_extents.into();
            solid_size *= 2.0;


            let (_, y_restrict, _) = restrict_transform_movement(&mut transform, player_size, solid_transform, solid_size);

            if let Some(y_restrict) = y_restrict {

                if y_restrict != 0.0 {
                    y_vel.vel = 0.0;
                }

                if y_restrict > 0.0 {
                    y_vel.grounded = true; 
                } else {
                    y_vel.grounded = false;
                }
            } else {
                y_vel.grounded = false;
            }
        }
    }
}


/// Applies YVel to the Transform
pub fn apply_yvel (
    mut player_q: Query<(&mut YVel, &mut Transform)>,
    time: Res<Time>,
) {
    for (vel, mut transform) in player_q.iter_mut() {
        transform.translation.y += vel.vel * time.delta_secs();
    }
}

/// Adds gravity to a YVel
pub fn gravity (
    mut player_q: Query<&mut YVel>,
    time: Res<Time>,
) {
    for mut vel in player_q.iter_mut() {
        vel.vel -= GRAVITY * time.delta_secs();
    }
}
