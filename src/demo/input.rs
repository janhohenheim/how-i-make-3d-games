//! Input handling for the player.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::demo::movement::AccumulatedInput;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>();
    app.add_observer(bind_inputs);
}

#[derive(Debug, InputAction)]
#[action_output(Vec3)]
pub(crate) struct Move;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(crate) struct Jump;

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
pub(crate) struct Rotate;

#[derive(Debug, Component, Default)]
#[require(AccumulatedInput)]
pub(crate) struct PlayerInput;

fn bind_inputs(add: On<Add, PlayerInput>, mut commands: Commands) {
    const DEFAULT_SENSITIVITY: f32 = 0.1;
    commands.entity(add.entity).insert(actions!(PlayerInput[
        (
            Action::<Move>::new(), DeadZone::default(),
            SmoothNudge::default(),
            Negate::y(),
            SwizzleAxis::XZY,
            Bindings::spawn((
                Cardinal::wasd_keys(),
                Axial::left_stick()
            ))
        ),
        (Action::<Jump>::new(), bindings![KeyCode::Space, GamepadButton::South]),
        (Action::<Rotate>::new(),Negate::all(), Scale::splat(DEFAULT_SENSITIVITY),
            Bindings::spawn((Spawn(Binding::mouse_motion()), Axial::right_stick()))),
    ]));
}
