//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use core::f32;
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    game::{
        spawn::player::Player,
        spawn::stage::Wall,
    },
    screen::Screen,
    
};
use crate::AppSet;




pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<Movement>();
    app.add_systems(
        Update,
        (apply_movement)
            .chain()
            .in_set(AppSet::Update),
    );

    app.register_type::<Path>();
    app.register_type::<Timeloop>();
    app.register_type::<IsDead>();

    app.add_systems(Update, (
        loop_time.run_if(in_state(Screen::Playing)),
        move_npcs.run_if(in_state(Screen::Playing)),
        kill_npcs.run_if(in_state(Screen::Playing)),
        rotate_dead.run_if(in_state(Screen::Playing)),
        detect_player.run_if(in_state(Screen::Playing)),

    ));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec3);

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.z += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x += 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x -= 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
    pub rotation: f32,
}

//fn gravity(
//    time:Res<Time>,
//    mut query: Query<(&mut MovementController, &Transform)>,
//){
//    for (mut mov,trans)in query.iter_mut(){
//        if trans.translation.y > 0{
//            
//        }
//    }
//}
fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &Movement, &mut Transform),Without<Wall>>,
    wall_query: Query<&Transform,With<Wall>>
) {
    for (controller, movement, mut transform) in &mut movement_query {
        let torque = movement.rotation * controller.0.x;
        transform.rotate(Quat::from_axis_angle(Vec3::Y,torque*time.delta_seconds()));
        let velocity = movement.speed * controller.0.z;
        let forward = transform.forward();
        let new_translation = transform.translation + forward * velocity * time.delta_seconds();
        for walltransform in wall_query.iter(){
            if new_translation.x>walltransform.translation.x-walltransform.scale.x &&
                new_translation.x<walltransform.translation.x+walltransform.scale.x &&
                new_translation.z>walltransform.translation.z-walltransform.scale.z &&
                new_translation.z<walltransform.translation.z+walltransform.scale.z{
                    return;
                }
        }
        transform.translation = new_translation;
    }
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Npc;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct IsDead;

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Path{
    pub points:Vec<(f32,Vec3)>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Timeloop {
    pub current_time : f32,
    pub max_time : f32,
}

pub fn loop_time(
    mut timeloop: ResMut<Timeloop>,
    time: Res<Time<Virtual>>,
){
    timeloop.current_time += time.delta_seconds();
    timeloop.current_time %= timeloop.max_time;
}


pub fn move_npcs(
    timeloop:Res<Timeloop>,
    mut npcs:Query<(&Path,&mut Transform),Without<IsDead>>,
){
    for (path,mut transform) in npcs.iter_mut(){
   //     let mut previous_point = path.points.last().expect("path fucking empty bruv");
 //       let mut next_point = previous_point;
        let mut nex_point = path.points.first().expect("first empty");
        let mut prev_point = path.points.last().expect("last empty");
        for (i,i_point) in path.points.iter().enumerate(){
            if timeloop.current_time < i_point.0 {
                nex_point = i_point;
                prev_point = match i==0{
                    true=>path.points.last().expect("last fuckin empty bruv"),
                    false=>path.points.get(i-1).expect("i fucking empty broooo"),
                };
                break;
            }
        }
        let mut diff = nex_point.0-prev_point.0;
        if diff<0.0{diff= timeloop.max_time+nex_point.0-prev_point.0;}
        let point_diff = nex_point.1-prev_point.1;
        let mut time_since_prev = timeloop.current_time-prev_point.0;
        if time_since_prev < 0.0{time_since_prev +=timeloop.max_time};
        transform.translation = prev_point.1 + point_diff*time_since_prev/diff;
        

    }
}

pub fn kill_npcs(
    npcs:Query<(&Npc,&Transform,Entity)>,
    player: Query<(&Player,&Transform)>,
    mut commands:Commands,
){
    for (_,playertransform) in player.iter(){
        for (_, enemytransform,entity) in npcs.iter(){
            let diff = enemytransform.translation-playertransform.translation;
            if diff.length()<1.0{
                commands.entity(entity).insert(IsDead);
            }
        }
    }
}
pub fn rotate_dead(
    mut dead: Query<&mut Transform,Added<IsDead>>,
){
    for mut transform in dead.iter_mut(){
        transform.rotate_local_x(PI/2.0);
    }
}

fn line_collision(a:Vec3,b:Vec3,c:Vec3,d:Vec3)->bool{
    let u_a = ((d.x-c.x)*(a.z-c.z) - (d.z-c.z)*(a.x-c.x)) / ((d.z-c.z)*(b.x-a.x) - (d.x-c.x)*(b.z-a.z));
    let u_b = ((b.x-a.x)*(a.z-c.z) - (b.z-a.z)*(a.x-c.x)) / ((d.z-c.z)*(b.x-a.x) - (d.x-c.x)*(b.z-a.z));
    if (0.0..=1.0).contains(&u_a)&&(0.0..=1.0).contains(&u_b){return true;}
    false
}

pub fn detect_player(
    enemies: Query<&Transform,(With<Npc>,Without<IsDead>)>,
    players: Query<&Transform,With<Player>>,
    walls: Query<&Transform,With<Wall>>,
){
    for player in players.iter(){ 
        for enemy in enemies.iter(){
            let diff = player.translation-enemy.translation;
            let angle = diff.angle_between(*enemy.forward());
            if angle < PI/4.0{
                let mut is_blocked = false;
                for wall in walls.iter(){
                    let wall0 = Vec3::new(wall.translation.x+wall.scale.x,0.0,wall.translation.z+wall.scale.z);
                    let wall1 = Vec3::new(wall.translation.x-wall.scale.x,0.0,wall.translation.z+wall.scale.z);
                    let wall2 = Vec3::new(wall.translation.x-wall.scale.x,0.0,wall.translation.z-wall.scale.z);
                    let wall3 = Vec3::new(wall.translation.x+wall.scale.x,0.0,wall.translation.z-wall.scale.z);

                    if  line_collision(enemy.translation, player.translation, wall0, wall1)||
                        line_collision(enemy.translation, player.translation, wall1, wall2)||
                        line_collision(enemy.translation, player.translation, wall2, wall3)||
                        line_collision(enemy.translation, player.translation, wall3, wall0)
                        {
                            println!("te escondistes");
                            is_blocked = true;
                            continue;
                        }
                    

                }
                if !is_blocked{
                    println!("teveo");
                }
            }
        }
    }
}
