//! Player-specific behavior.

use std::f32::consts::TAU;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_enhanced_input::prelude::*;

use crate::{
    demo::{
        input::{CameraInput, CaptureCursor, ReleaseCursor, Rotate},
        player::{PLAYER_FLOAT_OFFSET, PLAYER_HEIGHT, Player},
    },
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_camera);
    app.add_systems(Update, position_camera_at_player);
    app.add_observer(rotate_camera);
    app.add_observer(capture_cursor);
    app.add_observer(release_cursor);
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
        StateScoped(Screen::Gameplay),
        Actions::<CameraInput>::default(),
    ));
}

fn rotate_camera(
    trigger: Trigger<Fired<Rotate>>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
    window: Single<&Window>,
    time: Res<Time<Virtual>>,
) {
    if time.is_paused() {
        return;
    }
    if window.cursor_options.visible {
        return;
    }

    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

    yaw += trigger.value.x.to_radians();
    pitch += trigger.value.y.to_radians();
    pitch = pitch.clamp(-TAU / 4.0 + 0.01, TAU / 4.0 - 0.01);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn capture_cursor(
    _trigger: Trigger<Completed<CaptureCursor>>,
    mut window: Single<&mut Window>,
    menu: Res<State<Menu>>,
    next_menu: Res<NextState<Menu>>,
) {
    if **menu == Menu::None || matches!(*next_menu, NextState::Pending(Menu::None)) {
        grab_cursor(&mut window, true);
    }
}

fn release_cursor(_trigger: Trigger<Completed<ReleaseCursor>>, mut window: Single<&mut Window>) {
    grab_cursor(&mut window, false);
}

fn grab_cursor(window: &mut Window, grab: bool) {
    window.cursor_options.grab_mode = if grab {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    window.cursor_options.visible = !grab;
}

fn position_camera_at_player(
    mut camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    camera.translation =
        player.translation + Vec3::Y * (1.8 - PLAYER_HEIGHT / 2.0 - PLAYER_FLOAT_OFFSET);
}
