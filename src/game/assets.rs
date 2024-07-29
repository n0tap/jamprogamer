use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<HandleMap<MeshKey>>();
    app.init_resource::<HandleMap<MeshKey>>();

    app.register_type::<HandleMap<MaterialKey>>();
    app.init_resource::<HandleMap<MaterialKey>>();

    app.register_type::<HandleMap<SceneKey>>();
    app.init_resource::<HandleMap<SceneKey>>();

    app.register_type::<HandleMap<GraphKey>>();
    app.init_resource::<HandleMap<GraphKey>>();

    app.register_type::<Animations>();
    app.register_type::<Action>();


}
#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct Action{
    pub current_track:NlaTrack,
    pub new_track:NlaTrack,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum NlaTrack{
    Walk,
    Idle,
    Shoot,
    Die,
    Punch
}

#[derive(Resource,Reflect)]
pub struct Animations{
    pub animations:Vec<AnimationNodeIndex>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GraphKey {
    Character,
}

impl AssetKey for GraphKey {
    type Asset = AnimationGraph;
}

impl FromWorld for HandleMap<GraphKey> {
    fn from_world(world: &mut World) -> Self {
        let mut graph = AnimationGraph::new();

        let asset_server: &AssetServer = world.resource::<AssetServer>();
        let animations:Vec<AnimationNodeIndex> = graph.add_clips([
            GltfAssetLabel::Animation(0).from_asset("meshes/jamchar3.glb"),
            GltfAssetLabel::Animation(1).from_asset("meshes/jamchar3.glb"),
            GltfAssetLabel::Animation(2).from_asset("meshes/jamchar3.glb"),
            GltfAssetLabel::Animation(3).from_asset("meshes/jamchar3.glb"),
            GltfAssetLabel::Animation(4).from_asset("meshes/jamchar3.glb"),


        ]            
        .into_iter()
        .map(|path| asset_server.load(path)),
        1.0,
        graph.root,
        ).collect();
//        let asset_server = {};
        let mut commands = world.commands();
        commands.insert_resource(Animations {
            animations,});

        let asset_server: &AssetServer = world.resource::<AssetServer>();

        [
            (GraphKey::Character,asset_server.add(graph)),

        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SceneKey {
    Character,
}

impl AssetKey for SceneKey {
    type Asset = Scene;
}

impl FromWorld for HandleMap<SceneKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (SceneKey::Character,asset_server.load(GltfAssetLabel::Scene(0).from_asset("meshes/jamchar3.glb")))

        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MaterialKey {
    Red,
    Green,
    Blue,
}

impl AssetKey for MaterialKey {
    type Asset = StandardMaterial;
}

impl FromWorld for HandleMap<MaterialKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (MaterialKey::Red,asset_server.add(StandardMaterial::from_color(Color::srgb(1.0, 0.0, 0.0)))),
            (MaterialKey::Green,asset_server.add(StandardMaterial::from_color(Color::srgb(0.0, 1.0, 0.2)))),
            (MaterialKey::Blue,asset_server.add(StandardMaterial::from_color(Color::srgb(1.0, 0.0, 1.0))))

        ]
        .into()
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MeshKey {
    Capsule,
    Floor,
    Wall,
}

impl AssetKey for MeshKey {
    type Asset = Mesh;
}

impl FromWorld for HandleMap<MeshKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (MeshKey::Capsule,asset_server.add(Capsule3d::default().into())),
            (MeshKey::Floor,asset_server.add(Plane3d::new(*Dir3::Y,Vec2::new(200.0,200.0)).into())),
            (MeshKey::Wall,asset_server.add(Cuboid::from_length(2.0).into())),
        ]
        .into()
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
    Black,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            ImageKey::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        ),
        (
            ImageKey::Black,
            asset_server.load(
                "images/black.png"
            ),
        )]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/myheart.ogg"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

