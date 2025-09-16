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

use crate::demo::{
    camera::PlayerCamera,
    //input::{Jump, PlayerInput},
    player::{PLAYER_FLOAT_OFFSET, PLAYER_HEIGHT, Player},
};

//use super::input::Move;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
//use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        apply_movement.in_set(TnuaUserControlsSystemSet),
    );
    app.add_systems(
        RunFixedMainLoop,
        jump.run_if(input_just_pressed(KeyCode::Space))
            .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
    );
}

fn apply_movement(
    player: Single<(&mut TnuaController,), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    camera: Single<&Transform, With<PlayerCamera>>,
) {
    let (mut controller,) = player.into_inner();
    //let move_input = actions.get::<Move>().unwrap();
    let move_input = {
        let mut move_input = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) {
            move_input -= Vec3::Z;
        }
        if input.pressed(KeyCode::KeyS) {
            move_input += Vec3::Z;
        }
        if input.pressed(KeyCode::KeyA) {
            move_input -= Vec3::X;
        }
        if input.pressed(KeyCode::KeyD) {
            move_input += Vec3::X;
        }
        move_input
    };
    let yaw = camera.rotation.to_euler(EulerRot::YXZ).0;
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: yaw_quat * move_input * 10.0,
        float_height: PLAYER_HEIGHT + PLAYER_FLOAT_OFFSET,
        max_slope: TAU / 8.0,
        ..default()
    });
}

fn jump(mut controller: Single<&mut TnuaController>) {
    controller.action(TnuaBuiltinJump {
        height: 1.5,
        ..default()
    });
}
