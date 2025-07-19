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
    /// speed at which ball should go from bowler to batter
    speed: f32,
    did_bowler_throw: bool,
    /// is ball out of camera window
    is_out_of_bounds: bool,
    did_batsman_hit: bool,
    /// direction in which the ball is going right now
    direction: Vec3,
    game_window: GameWindow,
    /// bowler's coordinate
    bowler_coord: Vec3
}


impl<'a> Ball {
    
/// move the ball coord back to the batsman
pub fn move_to_baller(&mut self, coord: &mut Vec3) {
        //println!("setting coord");
        *coord=self.bowler_coord;
        self.did_bowler_throw=false;
} 

    /// move object with speed and in this direction exactly
pub fn move_object(&mut self,object_vector: &mut Vec3,direction: Vec3) {
    *object_vector+=direction*self.speed;   

    //println!("current y {}, y check at {}",object_vector.y,self.game_window.height);
    if (object_vector.x>self.game_window.width || object_vector.x < -self.game_window.width) || (object_vector.y>self.game_window.height || object_vector.y < -self.game_window.height) {
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


fn setup(mut commands: Commands,game_window: Query<&GameWindow,With<GameWindow>>,windows: Query<&Window,With<PrimaryWindow>>, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(GameWindow{
        height: windows.single().height(),
        width: windows.single().width()
    });

    // for batsman
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(100., 50.)),
                ..default()
            },
            transform: Transform { translation: Vec3 { x: 0., y: windows.single().height()/2.1, z: 1. }, 
                ..default()
             },
             ..default()
        },
        Player {name: String::from("Batsman") },
    ));


    // for bowler
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(100., 50.)),
                ..default()
            },
            transform: Transform { translation: Vec3 { x: 0., y: -windows.single().height()/2.1, z: 1. }, 
                ..default()
             },
             ..default()
        },
        Player {name: String::from("Bowler")},
    ));

    // for ball
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(15.))).into(),
        material: materials.add(Color::rgb(0.8, 0.2,0.9)).into(),
        transform: Transform::from_xyz(-50.,-windows.single().height()/2.3, 2.) ,
        ..default()
    },
    Ball {is_out_of_bounds:false,speed: 10.,did_bowler_throw: false, did_batsman_hit: false,game_window: GameWindow { height: windows.single().height(), width: windows.single().width() },direction: Vec3::ZERO, bowler_coord: Vec3 { x: -50., y: -windows.single().height()/2.3, z: 2. }}
));


}



/// throws the ball from bowler to batsman
fn bowler_throw(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    players: Query<(&Player,&Transform),(With<Player>,Without<Ball>)>,
    time: Res<Time>,
    game_window: Query<&GameWindow,With<GameWindow>>
) {
    let ballres=ball_query.get_single_mut();
    if ballres.is_ok() {
        let (mut ball,mut btransform)=ballres.unwrap();
        if ball.is_out_of_bounds {
            ball.move_to_baller(&mut btransform.translation);
            ball.is_out_of_bounds=false;
            
        }
        if keyboard_input.just_pressed(KeyCode::KeyG) {
                

                let mut batterTranslation=Vec3::ZERO;
                for (player,transform) in players.iter() {
                    if player.name=="Batsman" {
                        batterTranslation=transform.translation;
                    }
                }

                ball.did_bowler_throw=true;
                let direction=get_direction(btransform.translation, batterTranslation, 0.00001);
                ball.move_object(&mut btransform.translation, direction);
                ball.direction=direction;

            
        }
        else if ball.did_bowler_throw {

            
            let mut batterTranslation=Vec3::ZERO;
                for (player,transform) in players.iter() {
                    if player.name=="Batsman" {
                        batterTranslation=transform.translation;
                    }
                }

                let direction=ball.direction;
                ball.move_object(&mut btransform.translation, direction);

        }
    }
    else {
        print!("Didnt get ball");
    }

}
