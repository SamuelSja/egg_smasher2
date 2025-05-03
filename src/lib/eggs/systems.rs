


use bevy::{color::palettes::css::SILVER, prelude::*};

use crate::lib::{helper::collide, player::structs::{Player, YVel}};

use super::structs::Egg;


pub fn spawn_egg(coms: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, position: Vec2) {
    coms.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        Transform::from_xyz(position.x, 0.5, position.y),
        Egg::new(50.0, 8.0, Color::from(SILVER)),
    ));
}


pub fn spawn_test_egg (
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_egg(&mut coms, &mut meshes, &mut materials, Vec2::new(5.0, 5.0));
}

pub fn smash_egg (
    mut coms: Commands,
    mut player_q: Query<(&Transform, &mut YVel), With<Player>>,
    mut egg_q: Query<(Entity, &mut Egg, &Transform, &mut MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let egg_size = Vec3::new(0.5, 1.0, 0.5);
    let player_size = Vec3::splat(1.0);


    if let Ok((transform, mut y_vel)) = player_q.get_single_mut() {
        for (entity, mut egg, egg_transform, mut material) in egg_q.iter_mut() {
            let (_, y_collide, _) = collide(transform.translation, player_size, egg_transform.translation, egg_size);

            if let Some(y_collide) = y_collide {
                if y_collide > 0.0 && y_vel.vel <= -egg.vel_thresh {

                    let damage = -y_vel.vel.min(egg.health);
                    y_vel.vel += damage;

                    egg.health -= damage;

                    material.0 = materials.add(egg.health_color());

                    if egg.health <= 0.0 {
                        coms.get_entity(entity).unwrap().despawn();

                    }
                }
            }
        }
    }
}



