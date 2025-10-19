//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use std::f32::consts::TAU;

use crate::{
    demo::{
        camera::PlayerCamera,
        input::{Jump, Move},
        player::{PLAYER_FLOAT_OFFSET, PLAYER_HEIGHT, Player},
    },
    fixed_update_inspection::did_fixed_update_happen,
};

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (apply_movement, apply_jump).in_set(TnuaUserControlsSystems),
    );
    app.add_systems(
        Update,
        clear_accumulated_input.run_if(did_fixed_update_happen),
    );
    app.add_systems(
        PreUpdate,
        accumulate_input.after(EnhancedInputSystems::Apply),
    );
    app.add_observer(init_accumulated_input);
}

fn apply_movement(
    controller: Single<(&mut TnuaController, &AccumulatedInput)>,
    transform: Single<&Transform, With<PlayerCamera>>,
) {
    let (mut controller, accumulated_input) = controller.into_inner();
    let last_move = accumulated_input.last_move.unwrap_or_default();
    let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);

    const SPEED: f32 = 10.0;

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: yaw_quat * last_move * SPEED,
        float_height: PLAYER_HEIGHT / 2.0 + PLAYER_FLOAT_OFFSET,
        max_slope: TAU / 8.0,
        ..default()
    });
}

fn apply_jump(controller: Single<(&mut TnuaController, &AccumulatedInput)>) {
    let (mut controller, input) = controller.into_inner();
    if input.jumped {
        controller.action(TnuaBuiltinJump {
            height: 1.5,
            ..default()
        });
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct AccumulatedInput {
    // The last non-zero move that was inputed since the last fixed update
    last_move: Option<Vec3>,
    // Whether any frame since the fixed update input contained a jump
    jumped: bool,
}

fn accumulate_input(
    mut input: Single<&mut AccumulatedInput>,
    move_: Single<(&Action<Move>, &ActionState)>,
    jump: Single<&ActionState, With<Action<Jump>>>,
) {
    let (action, state) = move_.into_inner();
    if matches!(state, ActionState::Fired) {
        input.last_move.replace(**action);
    }
    let state = jump.into_inner();
    if matches!(state, ActionState::Fired) {
        input.jumped = true;
    }
}

fn init_accumulated_input(trigger: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(AccumulatedInput::default());
}

fn clear_accumulated_input(mut accumulated_inputs: Query<&mut AccumulatedInput>) {
    for mut accumulated_input in &mut accumulated_inputs {
        *accumulated_input = default();
    }
}
