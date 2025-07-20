mod baller;

use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::sprite::MaterialMesh2dBundle;
use baller::*;
use bevy::window::PrimaryWindow;





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bowler_throw)
        .run();
}

#[derive(Default)]
#[derive(Component)]
struct Player {
    name: String
}





#[derive(Component)]
struct Ball {
    did_bowler_throw: bool,
    /// is ball out of camera window
    is_out_of_bounds: bool,
    did_batsman_hit: bool,
    /// velocity in which the ball is going right now
    velocity: Vec3,
    game_window: GameWindow,
    /// bowler's coordinate
    bowler_coord: Vec3,
    /// initial coordinate of the ball when it was thrown
    initial_coord: Vec3,

}


impl Ball {
    
/// move the ball coord back to the batsman
pub fn move_to_baller(&mut self, coord: &mut Vec3) {
        //println!("setting coord");
        *coord=self.bowler_coord;
        self.did_bowler_throw=false;
} 


/// handles the ball movement
/// x axis is left-right, y axis is up-down, z axis is front-back
pub fn handle_ball_movement(&mut self,time: Time,transform: &mut Transform,conditions: &PlayingCondition) {

    
    let coord= &mut transform.translation;
    if coord.y<=0. {
        self.velocity.y=-self.velocity.y;
        self.velocity*=Vec3::new(1.0-conditions.pitch_friction,1.,1.0-conditions.pitch_friction);
        self.velocity*=Vec3 { x: 1.0,y: conditions.pitch_hardness,z:1.0 };
    }
    
    self.velocity+=Vec3::new(0.,-conditions.gravity,0.)*time.delta_seconds();
    self.velocity+=Vec3::new(conditions.wind,0.,0.)*time.delta_seconds();

    //println!("velocity {:?}",self.velocity);
    *coord+=self.velocity*time.delta_seconds();
    //println!("coord {:?}",coord);


}

    /// move object with speed and in this velocity exactly
pub fn move_object(&mut self,object_vector: &mut Vec3,velocity: Vec3) {
    //*object_vector+=velocity*self.speed;   

    //println!("current z {}, z check at {}",object_vector.z,self.game_window.height);
    if (object_vector.z>self.game_window.width || object_vector.z < -self.game_window.width) || (object_vector.y>self.game_window.height || object_vector.z < -self.game_window.height) {
        //println!("out of bounds");
        self.is_out_of_bounds=true;
    } 
}

}

/// Represents the game window dimensions
#[derive(Default)]
#[derive(Component)]
struct GameWindow {
    height: f32,
    width: f32
}


fn setup(mut commands: Commands,game_window: Query<&GameWindow,With<GameWindow>>,windows: Query<&Window,With<PrimaryWindow>>, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8.0, -20.0)
        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn((PlayingCondition::default()));

    // Add lighting - this is crucial for 3D!
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 1000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    commands.spawn(GameWindow{
        height: windows.single().height(),
        width: windows.single().width()
    });

    // for batsman - make bigger
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 3.0, 1.0))), // Bigger
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.7, 0.9),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 11.0),
            ..default()
        },
        Player {
            name: String::from("Batsman"),
        },
    ));
    
    // for bowler - convert to 3D PbrBundle
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 3.0, 1.0))), // Bigger
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.7, 0.3, 0.9), // Different color from batsman
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -11.0),
            ..default()
        },
        Player {name: String::from("Bowler")},
    ));

    // for ball - make bigger
    let ball_coord=Vec3::new(0.0,3.5, -11.0);
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(Sphere::new(0.3))), // Bigger ball
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.2, 0.9),
            ..default()
        }),
        transform: Transform { translation: ball_coord ,
        ..default() },
        ..default()
    },
    Ball {is_out_of_bounds:false,did_bowler_throw: false, did_batsman_hit: false,game_window: GameWindow { height: 48., width: 48. },velocity: Vec3::ZERO, bowler_coord: Vec3 { x: 0., y: 8.0, z: -8.0 },initial_coord: ball_coord }));


}



/// throws the ball from bowler to batsman
fn bowler_throw(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    players: Query<(&Player,&Transform),(With<Player>,Without<Ball>)>,
    time: Res<Time>,
    game_window: Query<&GameWindow,With<GameWindow>>,
    playing_condition: Query<&PlayingCondition,With<PlayingCondition>>
) {
    let ballres=ball_query.get_single_mut();
    if ballres.is_ok() {
        let (mut ball,mut btransform)=ballres.unwrap();
        //println!("ball is here {:?}",btransform.translation);
        if ball.is_out_of_bounds {
            ball.move_to_baller(&mut btransform.translation);
            ball.is_out_of_bounds=false;
            
        }
        if keyboard_input.just_pressed(KeyCode::KeyG) {
                
                //println!("Just pressed")

                btransform.translation=ball.initial_coord;
                ball.did_bowler_throw=false;

                let mut batterTranslation=Vec3::ZERO;
                for (player,transform) in players.iter() {
                    if player.name=="Batsman" {
                        batterTranslation=transform.translation;
                    }
                }
                let mut velocity=Vec3::ZERO;

                let in_between_pitch_coord=Vec3::new(0.,0.,0.0);
                if !ball.did_bowler_throw { 
                    //println!("getting direction");
                    velocity=get_velocity(btransform.translation, in_between_pitch_coord, 0.00001);
                    //println!("velocity {}",velocity);
                    velocity=20.*velocity;
                    //println!("velocity {}",velocity);
                }
                ball.velocity=velocity;
                //println!("velocity {:?}",velocity);
                ball.handle_ball_movement(*time,&mut btransform,playing_condition.single());
                ball.did_bowler_throw=true;

            
        }
        else if ball.did_bowler_throw {

            
            let mut batterTranslation=Vec3::ZERO;
                for (player,transform) in players.iter() {
                    if player.name=="Batsman" {
                        batterTranslation=transform.translation;
                    }
                }

                let velocity=ball.velocity;

                ball.handle_ball_movement(*time,&mut btransform,playing_condition.single());

        }
    }
    else {
        print!("Didnt get ball");
    }

}
