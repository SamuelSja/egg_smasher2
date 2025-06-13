


use std::u32;

use bevy::{color::palettes::css::SILVER, prelude::*};
use rand::RngCore;

use crate::lib::{helper::collide, player::structs::{Player, Shells, YVel}, upgrades::{base::UpgradeInfo, Upgrade}};

use super::{structs::{Egg, EggGenerationInfo}, EGG_SPAWN_POS, EGG_SPAWN_SIZE};


pub fn spawn_egg(coms: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, position: Vec2, egg_info: &mut EggGenerationInfo) {
    coms.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        Transform::from_xyz(position.x, 0.5, position.y),
        Egg::new(10.0, 8.0, Color::from(SILVER)),
    ));
    egg_info.eggs += 1;
}


pub fn spawn_test_egg (
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egg_info: ResMut<EggGenerationInfo>,
) {
    spawn_egg(&mut coms, &mut meshes, &mut materials, Vec2::new(5.0, 5.0), &mut egg_info);
}

pub fn smash_egg (
    mut coms: Commands,
    mut player_q: Query<(&Transform, &mut YVel), With<Player>>,
    mut egg_q: Query<(Entity, &mut Egg, &Transform, &mut MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

            if let Some(y_collide) = y_collide {
                if y_collide > 0.0 && y_vel.vel <= -egg.vel_thresh {

                    let damage = -y_vel.vel.min(egg.health);
                    y_vel.vel += damage;

                    // time = 0.0 because it should have no effect on damage
                    let damage = damage * Upgrade::Damage.effect(upgrade_info.upgrades[Upgrade::Damage as usize], 0.0);

                    egg.health -= damage;

                    material.0 = materials.add(egg.health_color());

                    if egg.health <= 0.0 {


                        let source: Handle<AudioSource> = assets.load(&format!("audio/effects/egg_crack{}.mp3", rand::random_range(0..4)));

                        coms.spawn((AudioPlayer(
                            source,
                        ),
                            PlaybackSettings::ONCE,
                        ));


                        coms.get_entity(entity).unwrap().despawn();

                        shells.val += 1;
                        egg_info.eggs -= 1;

                    }
                }
            }
        }
    }
}

pub fn egg_generation(
    mut coms: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egg_info: ResMut<EggGenerationInfo>,
    time: Res<Time>,
) {

    if egg_info.timer.tick(time.delta()).just_finished() {
        spawn_egg(&mut coms, &mut meshes, &mut materials, Vec2::new(5.0, 5.0), &mut egg_info);

        let start = EGG_SPAWN_POS - EGG_SPAWN_SIZE; 
        let end = EGG_SPAWN_POS + EGG_SPAWN_SIZE; 

        let mut rng = rand::rng();
        let ran = (rng.next_u32(), rng.next_u32());
        let ran = Vec2::new(ran.0 as f32, ran.1 as f32);

        let ran = (ran / Vec2::splat(u32::MAX as f32)) * (end - start) + start;

        spawn_egg(&mut coms, &mut meshes, &mut materials, ran, &mut egg_info);


    }


}

