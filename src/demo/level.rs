//! Spawn the main level.

use bevy::{color::palettes::tailwind, prelude::*};

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    map: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            map: assets.load("maps/main_level.map#Scene"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        SceneRoot(level_assets.map.clone()),
        StateScoped(Screen::Gameplay),
        children![(
            DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::default().looking_to(Vec3::new(2.0, -6.0, -1.0), Vec3::Y)
        )],
    ));
    commands.insert_resource(AmbientLight {
        color: tailwind::AMBER_200.into(),
        brightness: 150.0,
        ..default()
    });
}
