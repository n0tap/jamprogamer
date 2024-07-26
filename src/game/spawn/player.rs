//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap,SceneKey,Action,NlaTrack},
        movement::{Movement, MovementController},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

//#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
//#[reflect(Component)]
//pub struct CameraPosition{
//    player:Vec3,
//}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
 //   camera:Query<Entity,With<Camera3d>>,
    scene_handles: Res<HandleMap<SceneKey>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs



    commands.spawn((
        Name::new("Player"),
        SceneBundle{
            scene:scene_handles[&SceneKey::Character].clone_weak(),
            transform:Transform::from_translation(Vec3::new(-2.0,0.0,-2.0)),
            ..Default::default()
        },
        MovementController::default(),
        Movement { speed: 5.5, rotation:3.0 },
        Action{
            current_track:NlaTrack::Idle,
            new_track:NlaTrack::Idle,
        },
        StateScoped(Screen::Playing),
        Player,
    )).with_children( |child_builder|   {
        child_builder.spawn((Name::new("Light"),
            PointLightBundle{
                point_light:PointLight{
                    color:Color::srgb(1.0,1.0,1.0),
                    intensity:10000000.0,
                    shadows_enabled:true,
                    range:50.0,
                    ..Default::default()
                },
                transform:Transform::from_translation(Vec3::new(0.0,7.5,0.0)),
                ..Default::default()


        },
        StateScoped(Screen::Playing),
    ));
    });

    //commands.entity(camera.single()).insert(CameraPosition{player:Vec3::new(-5.7, 20.7,-20.0)});
}
