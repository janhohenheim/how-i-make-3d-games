use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FramepacePlugin);
}
