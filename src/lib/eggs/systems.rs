


use std::u32;

use bevy::{color::palettes::css::*, prelude::*};
use rand::{random_range, Rng, RngCore};

use crate::lib::{eggs::structs::{EggParticle, R2Vel}, helper::collide, player::structs::{Player, Shells, YVel}, upgrades::{base::UpgradeInfo, Upgrade}};

use super::{structs::{Egg, EggGenerationInfo}, EGG_SPAWN_POS, EGG_SPAWN_SIZE};

// /// Spawns a plane egg
// pub fn spawn_basic_egg(coms: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, position: Vec2, egg_info: &mut EggGenerationInfo) {
// 
//     coms.spawn((
//         Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
//         MeshMaterial3d(materials.add(Color::from(SILVER))),
//         Transform::from_xyz(position.x, 0.5, position.y),
//         Egg::new(10.0, 8.0, Color::from(SILVER)),
//     ));
//     egg_info.eggs += 1;
// }

/// Spawns a random type of egg
pub fn spawn_random_egg(coms: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, egg_info: &mut EggGenerationInfo) {
    let end = EGG_SPAWN_POS + EGG_SPAWN_SIZE; 
    let start = EGG_SPAWN_POS - EGG_SPAWN_SIZE; 
    let mut rng = rand::rng();
    let ran = (rng.next_u32(), rng.next_u32());
    let ran = Vec2::new(ran.0 as f32, ran.1 as f32);
    let position = (ran / Vec2::splat(u32::MAX as f32)) * (end - start) + start;

    let rarity = rng.random_range(0..1150) as f32;

    let color = rarity_color(rarity);

    coms.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
        MeshMaterial3d(materials.add(color)),
        Transform::from_xyz(position.x, 0.5, position.y),
        Egg::new(rarity_health(rarity), 8.0, color, rarity_shells(rarity)),
    ));
    egg_info.eggs += 1;
}


/// given the rarity this method gives a color for it
pub fn rarity_color(rarity: f32) -> Color {
    if rarity <= 500.0 {
        Color::from(SILVER)
    } else if rarity <= 700.0 {
        Color::from(GREEN)
    } else if rarity <= 1150.0 {
        Color::from(GOLD)
    } else {
        Color::from(RED)
    }
}

/// Give the health based of the rarity
pub fn rarity_health(rarity: f32) -> f32 {
    
    rarity *  
    if rarity <= 500.0 {
        1.0 / 100.0
    } else if rarity <= 700.0 {
        1.0 / 25.0
    } else if rarity <= 1150.0 {
        1.0 / 10.0
    } else {
        1_000_000.0
    }
}

pub fn rarity_shells(rarity: f32) -> i32 {
    (rarity * 
    if rarity <= 500.0 {
        1.0 / 200.0
    } else if rarity < 700.0 {
        1.0 / 35.0
    } else if rarity < 1150.0 {
        1.0 / 10.0
    } else {
        0.0
    }
    
    ).ceil() as i32
}







// pub fn spawn_test_egg (
//     mut coms: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut egg_info: ResMut<EggGenerationInfo>,
// ) {
//     spawn_egg(&mut coms, &mut meshes, &mut materials, Vec2::new(5.0, 5.0), &mut egg_info);
// }

pub fn smash_egg (
    mut coms: Commands,
    mut player_q: Query<(&Transform, &mut YVel), With<Player>>,
    mut egg_q: Query<(Entity, &mut Egg, &Transform, &mut MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shells: ResMut<Shells>,
    mut egg_info: ResMut<EggGenerationInfo>,
    assets: Res<AssetServer>,
    upgrade_info: Res<UpgradeInfo>,
) {
    let egg_size = Vec3::new(0.5, 1.0, 0.5);
    let player_size = Vec3::splat(1.0);


    if let Ok((transform, mut y_vel)) = player_q.get_single_mut() {
        for (entity, mut egg, egg_transform, mut material) in egg_q.iter_mut() {
            let (_, y_collide, _) = collide(transform.translation, player_size, egg_transform.translation, egg_size);

            if let Some(_) = y_collide {

                if y_vel.vel <= -egg.vel_thresh {
                    
                    let upgrade = Upgrade::Damage.effect(upgrade_info.upgrades[Upgrade::Damage as usize], None).expect("Damage is not dependent on time");
                    let damage = -y_vel.vel * upgrade;
                    let damage = damage.min(egg.health);

                    y_vel.vel += damage / upgrade;


                    egg.health -= damage;

                    material.0 = materials.add(egg.health_color());

                    if egg.health <= 0.0 {


                        let source: Handle<AudioSource> = assets.load(&format!("audio/effects/egg_crack{}.mp3", rand::random_range(0..4)));

                        coms.spawn((AudioPlayer(
                            source,
                        ),
                            PlaybackSettings::DESPAWN,
                        ));


                        let translation = egg_transform.translation;

                        let size_range = &(0.1..0.3);
                        for _ in 0..40 {
                            coms.spawn((
                                EggParticle::rand(9.0..10.0),
                                YVel::rand(0.0..12.0),
                                Mesh3d(meshes.add(Cuboid::new(random_range(size_range.clone()), random_range(size_range.clone()), random_range(size_range.clone())))),
                                Transform::from_xyz(translation.x, translation.y, translation.z),
                                MeshMaterial3d(materials.add(egg.color)),
                                R2Vel::rand(-0.050..0.050, -0.050..0.050),
                            ));
                        }

                        coms.get_entity(entity).unwrap().despawn();

                        let new_shells = shells.val as i32 + egg.shells;
                        shells.val = if new_shells < 0 {
                            0
                        } else {
                            new_shells as u32
                        };
                        egg_info.eggs -= 1;

                    }
                }
            }
        }
    }
}










// pub fn basic_egg_generation(
//     mut coms: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut egg_info: ResMut<EggGenerationInfo>,
//     time: Res<Time>,
// ) {
// 
//     if egg_info.timer.tick(time.delta()).just_finished() {
//         let start = EGG_SPAWN_POS - EGG_SPAWN_SIZE; 
//         let end = EGG_SPAWN_POS + EGG_SPAWN_SIZE; 
//         let mut rng = rand::rng();
//         let ran = (rng.next_u32(), rng.next_u32());
//         let ran = Vec2::new(ran.0 as f32, ran.1 as f32);
//         let ran = (ran / Vec2::splat(u32::MAX as f32)) * (end - start) + start;
// 
//         spawn_basic_egg(&mut coms, &mut meshes, &mut materials, ran, &mut egg_info);
//     }
// }

pub fn random_egg_generation(
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egg_info: ResMut<EggGenerationInfo>,
    time: Res<Time>,
) {

    if egg_info.timer.tick(time.delta()).just_finished() {
        spawn_random_egg(&mut coms, &mut meshes, &mut materials, &mut egg_info);
    }
}

pub fn apply_r2vel (
    mut r2vels_q: Query<(&mut Transform, &R2Vel)>
) {
    for (mut transform, vel) in r2vels_q.iter_mut() {
        transform.translation.x += vel.vel.x;
        transform.translation.z += vel.vel.y;
    }
}

/// A naive implementation of static friction
pub fn static_friction (
    mut obj_q: Query<(&mut R2Vel, &YVel)>
) {
    for (mut r2vel, yvel) in obj_q.iter_mut() {
        if yvel.grounded {
            r2vel.vel = Vec2::ZERO;
        }
    }
}

pub fn egg_particle_despawn (
    mut coms: Commands,
    mut particle_q: Query<(Entity, &mut EggParticle)>,
    time: Res<Time>,
) {
    for (entity, mut particle) in particle_q.iter_mut() {
        particle.secs_left -= time.delta_secs();

        if particle.secs_left <= 0.0 {
            coms.get_entity(entity).expect("Entity should exist").despawn();
        }
    }
}