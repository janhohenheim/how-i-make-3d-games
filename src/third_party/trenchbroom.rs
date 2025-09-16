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
}
