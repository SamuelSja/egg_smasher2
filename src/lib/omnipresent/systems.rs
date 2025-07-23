
use bevy::{prelude::*, render::primitives::Aabb};
use crate::lib::{helper::restrict_transform_movement, omnipresent::GRAVITY};

use super::structs::*;


/// blocks Phy from hitting things
pub fn restrict_phy (
    mut phy_q: Query<(&mut Transform, &Aabb, &mut Phy)>,
    solids_q: Query<(&Transform, &Aabb), (With<Solid>, Without<Phy>)>
) {
    for (mut transform, aabb, mut phy) in phy_q.iter_mut() {
        for (solid_transform, solid_aabb) in solids_q.iter() {
            
            let mut player_size: Vec3 = aabb.half_extents.into();
            player_size *= 2.0;

            let mut solid_size: Vec3 = solid_aabb.half_extents.into();
            solid_size *= 2.0;


            let (x_restrict, y_restrict, z_restrict) = restrict_transform_movement(&mut transform, player_size, solid_transform, solid_size);

            if let Some(y_restrict) = y_restrict {
                if y_restrict != 0.0 {
                    phy.vel.y = 0.0;
                }
                if y_restrict > 0.0 {
                    phy.grounded = true; 
                } else { 
                    phy.grounded = false;
                }
            } else {
                phy.grounded = false;
                
            }

            // todo: DRY
            if let Some(x_restrict) = x_restrict {
                if x_restrict != 0.0 {
                    phy.vel.x = 0.0;
                }
            }

            if let Some(z_restrict) = z_restrict {
                if z_restrict != 0.0 {
                    phy.vel.z = 0.0;
                }
            }
        }
    }
}


/// Applies Phy to the Transform
pub fn apply_phy (
    mut phy_q: Query<(&mut Phy, &mut Transform)>,
    time: Res<Time>,
) {
    for (vel, mut transform) in phy_q.iter_mut() {
        transform.translation += vel.vel * time.delta_secs();
    }
}

/// Adds gravity to a Gravitatable
pub fn gravity (
    mut player_q: Query<&mut Phy, With<Gravitatable>>,
    time: Res<Time>,
) {
    for mut phy in player_q.iter_mut() {
        phy.vel.y -= GRAVITY * time.delta_secs();
    }
}

/// Simple implementation of static friction
pub fn static_friction (
    mut obj_q: Query<&mut Phy, With<Friction>>
) {
    for mut phy in obj_q.iter_mut() {
        if phy.grounded {
            phy.vel.x = 0.0;
            phy.vel.z = 0.0;
        }
    }
}