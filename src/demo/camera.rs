//! Player-specific behavior.

use std::f32::consts::TAU;

use bevy::{
    input::{common_conditions::input_just_pressed, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
//use bevy_enhanced_input::prelude::*;

use crate::{
    demo::{
        //input::{CameraInput, CaptureCursor, ReleaseCursor, Rotate},
        player::{PLAYER_FLOAT_OFFSET, PLAYER_HEIGHT, Player},
    },
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_camera);
    app.add_systems(Update, position_camera_at_player);
    app.add_systems(
        RunFixedMainLoop,
        rotate_camera.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
    );
    app.add_observer(capture_cursor);
    app.add_systems(
        RunFixedMainLoop,
        release_cursor
            .run_if(input_just_pressed(KeyCode::Escape))
            .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera;

fn spawn_camera(_trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        PlayerCamera,
        DespawnOnExit(Screen::Gameplay),
        //Actions::<CameraInput>::default(),
    ));
}

fn rotate_camera(
    //trigger: Trigger<Fired<Rotate>>,
    trigger: Res<AccumulatedMouseMotion>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
    cursor_options: Single<&CursorOptions>,
    time: Res<Time<Virtual>>,
) {
    if time.is_paused() {
        return;
    }
    if cursor_options.visible {
        return;
    }

    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

    yaw -= trigger.delta.x.to_radians() * 0.1;
    pitch -= trigger.delta.y.to_radians() * 0.1;
    pitch = pitch.clamp(-TAU / 4.0 + 0.01, TAU / 4.0 - 0.01);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn capture_cursor(
    _trigger: Trigger<Pointer<Press>>,
    mut cursor_options: Single<&mut CursorOptions>,
    menu: Res<State<Menu>>,
    next_menu: Res<NextState<Menu>>,
) {
    if **menu == Menu::None || matches!(*next_menu, NextState::Pending(Menu::None)) {
        grab_cursor(&mut cursor_options, true);
    }
}

fn release_cursor(
    //_trigger: Trigger<Pointer<Release>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut cursor_options, false);
}

fn grab_cursor(cursor_options: &mut CursorOptions, grab: bool) {
    cursor_options.grab_mode = if grab {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    cursor_options.visible = !grab;
}

fn position_camera_at_player(
    mut camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    camera.translation =
        player.translation + Vec3::Y * (1.8 - PLAYER_HEIGHT / 2.0 - PLAYER_FLOAT_OFFSET);
}
