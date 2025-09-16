use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugins(
        TrenchBroomConfig::new("presentation").default_solid_spawn_hooks(|| {
            SpawnHooks::new()
                .smooth_by_default_angle()
                .convex_collider()
        }),
    ));
    app.add_systems(Startup, write_trenchbroom_config);
}

fn write_trenchbroom_config(server: Res<TrenchBroomServer>, type_registry: Res<AppTypeRegistry>) {
    // This will write <TB folder>/games/example_game/GameConfig.cfg,
    // and <TB folder>/games/example_game/example_game.fgd
    if let Err(err) = server
        .config
        .write_game_config_to_default_directory(&type_registry.read())
    {
        error!("Could not write TrenchBroom game config: {err}");
    }

    // And this will add our game to <TB folder>/Preferences.json
    if let Err(err) = server.config.add_game_to_preferences_in_default_directory() {
        error!("Could not write TrenchBroom preferences: {err}");
    }
}
