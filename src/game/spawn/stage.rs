use bevy::{
    prelude::*,
//    core_pipeline::Skybox,
};

use crate::{
    game::{
        assets::{HandleMap, MeshKey,MaterialKey,SceneKey,
//            ImageKey
        },
        movement::{Npc,Path,Timeloop},
    },
    screen::Screen,
    
};

pub(super) fn plugin(app: &mut App) {

    app.observe(spawn_stage);
    app.register_type::<Wall>();

}

#[derive(Event, Debug)]
pub struct SpawnStage;

fn spawn_stage(
    _trigger: Trigger<SpawnStage>,
    mut commands: Commands,
    mesh_handles: Res<HandleMap<MeshKey>>,
    material_handles: Res<HandleMap<MaterialKey>>,
    scene_handles: Res<HandleMap<SceneKey>>,
//    image_handles:Res<HandleMap<ImageKey>>,
//    camera:Query<Entity,With<Camera>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
//    commands.entity(camera.single()).
//        insert(Skybox{
//            image:image_handles[&ImageKey::Black].clone_weak(),
//            brightness:1.0,
//        }
//    );
    commands.spawn((
        Name::new("Floor"),
        MaterialMeshBundle{
            mesh: mesh_handles[&MeshKey::Floor].clone_weak(),
            material:material_handles[&MaterialKey::Red].clone_weak(),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));


    commands.spawn((
        Name::new("Enemy"),
        SceneBundle{
            scene:scene_handles[&SceneKey::Character].clone_weak(),
            ..Default::default()
        },
        Npc,
        Path{
            points: vec![
                (2.0,Vec3::new(1.0,0.0,3.0)),
                (10.0,Vec3::new(10.0,0.0,2.0)),
                (15.0,Vec3::new(-3.0,0.0,2.0)),
                ]
        },
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        Name::new("Enemy2"),
        SceneBundle{
            scene:scene_handles[&SceneKey::Character].clone_weak(),
            ..Default::default()
        },
        Npc,
        Path{
            points:vec![
                (3.0,Vec3::new(1.0,0.0,1.0)),
                (6.0,Vec3::new(4.0,0.0,10.0)),
                (13.0,Vec3::new(-10.0,0.0,5.5)),
                (14.5,Vec3::new(-10.0,0.0,5.5)),
            ]
        },
        StateScoped(Screen::Playing),

    ));
    commands.insert_resource(Timeloop{
        current_time:0.0,
        max_time:20.0,
    });

//WALLS    
    commands.spawn((
        Name::new("Wall"),

        Wall,
        MaterialMeshBundle{
            transform:Transform{
                translation:(Vec3::new(-9.0,1.0,1.0)),
                scale:(Vec3::new(9.0,5.0,1.0)),
                ..default()
            },
            mesh: mesh_handles[&MeshKey::Wall].clone_weak(),
            material:material_handles[&MaterialKey::Blue].clone_weak(),
            ..default()
        },
        StateScoped(Screen::Playing),

    ));
    commands.spawn((
        Name::new("Wall"),

        Wall,
        MaterialMeshBundle{
            transform:Transform{
                translation:(Vec3::new(-1.0,1.0,5.0)),
                scale:(Vec3::new(1.0,5.0,3.0)),
                ..default()
            },
            mesh: mesh_handles[&MeshKey::Wall].clone_weak(),
            material:material_handles[&MaterialKey::Blue].clone_weak(),
            ..default()
        },
        StateScoped(Screen::Playing),

    ));
    commands.spawn((
        Name::new("Wall"),

        Wall,
        MaterialMeshBundle{
            transform:Transform{
                translation:(Vec3::new(-1.0,1.0,13.0)),
                scale:(Vec3::new(1.0,5.0,3.0)),
                ..default()
            },
            mesh: mesh_handles[&MeshKey::Wall].clone_weak(),
            material:material_handles[&MaterialKey::Blue].clone_weak(),
            ..default()
        },
        StateScoped(Screen::Playing),

    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Wall;