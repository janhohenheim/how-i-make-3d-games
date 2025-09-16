use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_crate);
}

#[point_class(base(Visibility, Transform), model("models/crate/crate01.gltf"))]
struct Crate;

fn spawn_crate(trigger: Trigger<OnAdd, Crate>, mut commands: Commands, assets: Res<AssetServer>) {
    let model_path = Crate::CLASS_INFO.model_path().unwrap().to_string();
    let scene_path = GltfAssetLabel::Scene(0).from_asset(model_path);
    commands.entity(trigger.entity).insert((
        SceneRoot(assets.load(scene_path)),
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_density(ColliderDensity(100.0)),
    ));
}
