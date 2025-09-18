use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.add_observer(enable_interpolation);
}

fn enable_interpolation(
    trigger: On<Add, RigidBody>,
    rigid_body: Query<&RigidBody>,
    mut commands: Commands,
) {
    let Ok(rigid_body) = rigid_body.get(trigger.entity) else {
        return;
    };
    if rigid_body.is_dynamic() {
        commands
            .entity(trigger.entity)
            .insert(TransformInterpolation);
    }
}
