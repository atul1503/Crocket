use bevy::prelude::*;
use rand::Rng;

use crate::GameWindow;

/// Gets directions between point a and b with randomness such that the more the randomness the more the direction might point away from the actual direction.
/// Positive negative for direction will mean left or right.
pub fn get_direction(point_a: Vec3, point_b: Vec3,randomness: f32) -> Vec3 {
    if (point_a-point_b).length() == 0. {
        println!("{} and {} are the same bro!",point_a,point_b);
        return Vec3::ZERO;
    }
    else {
        let mut rng = rand::rng();
        let skew= rng.random_range(-randomness..randomness);
        //println!("skew, {}",skew);
        let rotation_quat=Quat::from_rotation_z(skew);
        return rotation_quat*(point_b-point_a).normalize();
    }
}


