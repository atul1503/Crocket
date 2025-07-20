use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct PlayingCondition {
    pub gravity: f32,
    pub wind: f32,
    pub pitch_friction: f32,
    pub pitch_hardness: f32
}

impl Default for PlayingCondition {
     fn default() -> Self {
        Self {
            gravity: 9.81,
            /// wind drift for x axis
            wind: 0.05,
            /// affects ball speed after pitch and grip
            pitch_friction: 0.05,
            pitch_hardness: 0.95
        }
    }


}


/// Gets velocity vector between point a and b with randomness such that the more the randomness the more the velocity might point away from the actual velocity.
/// Positive negative for velocity will mean left or right.
pub fn get_velocity(point_a: Vec3, point_b: Vec3,randomness: f32) -> Vec3 {
    //println!("point_a {:?}, point_b {:?}",point_a,point_b);
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


