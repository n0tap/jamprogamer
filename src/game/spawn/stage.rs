use std::time::Duration;

use bevy::{
    prelude::*,
//    core_pipeline::Skybox,
};

use crate::{
    game::{
        assets::{Action, Animations, GraphKey, HandleMap, MaterialKey, MeshKey, SceneKey,NlaTrack
//            ImageKey
        },
        movement::{GhostPath, Npc, Path, Timeloop},
    },
    screen::Screen,
    
};

pub(super) fn plugin(app: &mut App) {

    app.observe(spawn_stage);
    app.register_type::<Wall>();
    app.add_systems(Update,setup_scene_once_loaded.run_if(in_state(Screen::Playing)));
    app.init_resource::<GhostPath>();
    app.init_resource::<Timeloop>();
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
        Action{
            current_track:NlaTrack::Idle,
            new_track:NlaTrack::Walk,
        },
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
        Action{
            current_track:NlaTrack::Idle,
            new_track:NlaTrack::Walk,
        },
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
        gen:0,
    });
    commands.insert_resource(GhostPath{points:vec![(0.0,Vec3::ZERO)]});


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

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    graph_handles: Res<HandleMap<GraphKey>>,

) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(graph_handles[&GraphKey::Character].clone())
            .insert(transitions);
    }
}

impl FromWorld for GhostPath {
    fn from_world(_: &mut World) -> Self {
        GhostPath{points:vec![(0.0,Vec3::ZERO)]}
    }
}
impl FromWorld for Timeloop {
    fn from_world(_: &mut World) -> Self {
        Timeloop { current_time: 0.0, max_time: 0.0, r#gen: 0 }
    }

}