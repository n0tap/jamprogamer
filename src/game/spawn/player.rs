//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap,SceneKey},
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

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,

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
        Movement { speed: 2.8, rotation:3.0 },
        StateScoped(Screen::Playing),
        Player,
    ));
}
