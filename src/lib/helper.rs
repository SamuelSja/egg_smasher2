











use std::f32::EPSILON;

use bevy::{math::{Vec2, Vec3}, transform::components::Transform};




/// Checks for an R-1 intersection
pub fn intersect_1d(start1: f32, end1: f32, start2: f32, end2: f32) -> bool {
    start1 < start2 && start2 < end1
    || start2 < start1 && start1 < end2
    || start1 < end2 && end2 < end1
    || start2 < end1 && end1 < end2
}

/// Checks for an R-2 intersection
pub fn intersect_2d(start1: Vec2, end1: Vec2, start2: Vec2, end2: Vec2) -> (bool, bool) {
    let x = intersect_1d(start1.x, end1.x, start2.x, end2.x);
    let y = intersect_1d(start1.y, end1.y, start2.y, end2.y);

    (x, y)
}

/// Checks for an R-3 intersection
pub fn intersect_3d(start1: Vec3, end1: Vec3, start2: Vec3, end2: Vec3) -> (bool, bool, bool) {
    let x = intersect_1d(start1.x, end1.x, start2.x, end2.x);
    let y = intersect_1d(start1.y, end1.y, start2.y, end2.y);
    let z = intersect_1d(start1.z, end1.z, start2.z, end2.z);

    (x, y, z)
}


/// If collides the method returns how much each direction collided.
/// If the objects do not collide this method returns (None, None, None)
/// All dimensions will be 0 except the greatest
pub fn collide(pos1: Vec3, size1: Vec3, pos2: Vec3, size2: Vec3) -> (Option<f32>, Option<f32>, Option<f32>) {
    let start1 = pos1 - size1 / 2.0;
    let end1 = pos1 + size1 / 2.0;
    let start2 = pos2 - size2 / 2.0;
    let end2 = pos2 + size2 / 2.0;


    let data = intersect_3d(start1, end1, start2, end2);

    let mut change_x = None;
    if data.0 {
        let negative = pos1.x < pos2.x;
        change_x = Some(if negative {
            -(end1.x - start2.x)
        } else {
            end2.x - start1.x
        });
    }

    let mut change_y = None;
    if data.1 {
        let negative = pos1.y < pos2.y;
        change_y = Some(if negative {
            -(end1.y - start2.y)
        } else {
            end2.y - start1.y
        });
    }

    let mut change_z = None;
    if data.2 {
        let negative = pos1.z < pos2.z;
        change_z = Some(if negative {
            -(end1.z - start2.z)
        } else {
            end2.z - start1.z
        });
    }


    

    if change_x.is_some() && change_y.is_some() && change_z.is_some() {

        // Note: moves pased the closest edge
        if change_x.unwrap().abs() < change_y.unwrap().abs() && change_z.unwrap().abs() < change_z.unwrap().abs() {
            change_y = Some(0.0);
            if change_x.unwrap().abs() < change_z.unwrap().abs() {
                change_z = Some(0.0);
            } else {
                change_x = Some(0.0);
            }
        }
        if change_x.unwrap().abs() < change_z.unwrap().abs() {
            change_z = Some(0.0);

            if change_x.unwrap().abs() < change_y.unwrap().abs() {
                change_y = Some(0.0);
            } else {
                change_x = Some(0.0);

            }

        } else {
            change_x = Some(0.0);
            if change_y.unwrap().abs() < change_z.unwrap().abs() {
                change_z = Some(0.0);
            } else {
                change_y = Some(0.0);
            }
        }



        (change_x, change_y, change_z)
    } else {
        (None, None, None)
    }
}

/// This method return true if the two objects collide and false otherwise
pub fn did_collide(pos1: Vec3, size1: Vec3, pos2: Vec3, size2: Vec3) -> bool {
    let collide = collide(pos1, size1, pos2, size2);

    if let (None, None, None) = collide {
        false
    } else {
        true
    }
}



/// This method moves the moving object back from the static object when they overlap.
pub fn restrict_transform_movement(
    moving_transform: &mut Transform,
    moving_size: Vec3, 
    static_transform: &Transform,
    static_size: Vec3,
) -> (Option<f32>, Option<f32>, Option<f32>) {
    let collide = collide(
        moving_transform.translation,
        moving_size,
        static_transform.translation,
        static_size,
    );

    if let (Some(x), _, _) = collide {
        moving_transform.translation.x += x;
    }
    if let (_, Some(y), _) = collide {
        moving_transform.translation.y += y;
    }
    if let (_, _, Some(z)) = collide {
        moving_transform.translation.z += z;
    }

    collide

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_1d() {
        assert_eq!(intersect_1d(0.0, 1.0, 0.5, 1.5), true);
        assert_eq!(intersect_1d(0.5, 1.5, 0.0, 1.0), true);
        assert_eq!(intersect_1d(0.0, 0.5, 1.0, 2.0), false);
        assert_eq!(intersect_1d(1.0, 2.0, 0.0, 0.5), false);
        assert_eq!(intersect_1d(0.0, 2.0, 0.5, 1.5), true);
        assert_eq!(intersect_1d(0.5, 1.5, 0.0, 2.0), true);
    }

    #[test]
    fn test_intersect_2d() {
        assert_eq!(intersect_2d(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 100.0), Vec2::new(2.0, 200.0)), (true, false));
        assert_eq!(intersect_2d(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(100.0, 0.0), Vec2::new(200.0, 2.0)), (false, true));
    }

    #[test]
    fn test_collide() {
        assert_eq!(collide(Vec3::splat(0.0), Vec3::splat(2.0), Vec3::new(0.0, 5.0, 0.0), Vec3::splat(2.0)), (None, None, None));

        let case2 = collide(Vec3::splat(0.0), Vec3::splat(2.0), Vec3::new(0.0, 1.24, 0.0), Vec3::splat(5.0));
        let (_, y, _) = case2;
        assert_ne!(y, None);

    }


}