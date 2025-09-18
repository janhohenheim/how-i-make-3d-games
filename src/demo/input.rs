use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>();
    app.add_input_context::<CameraInput>();
    app.add_observer(bind_player);
    app.add_observer(bind_camera);
}

#[derive(InputContext)]
#[input_context(schedule = FixedPreUpdate)]
pub struct PlayerInput;

#[derive(InputContext)]
pub struct CameraInput;

#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
pub struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Jump;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct CaptureCursor;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct ReleaseCursor;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Rotate;

fn bind_player(trigger: On<Bind<PlayerInput>>, mut players: Query<&mut Actions<PlayerInput>>) {
    let mut actions = players.get_mut(trigger.entity).unwrap();

    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), Axial::left_stick()))
        .with_modifiers((
            DeadZone::default(),
            SmoothNudge::default(),
            Negate::y(),
            SwizzleAxis::XZY,
        ));

    actions
        .bind::<Jump>()
        .to((KeyCode::Space, GamepadButton::South));
}

fn bind_camera(trigger: On<Bind<CameraInput>>, mut players: Query<&mut Actions<CameraInput>>) {
    let mut actions = players.get_mut(trigger.entity).unwrap();

    actions.bind::<Rotate>().to((
        // You can attach modifiers to individual inputs as well.
        Input::mouse_motion().with_modifiers((Scale::splat(0.1), Negate::all())),
        Axial::right_stick().with_modifiers_each((Scale::splat(2.0), Negate::x())),
    ));

    actions.bind::<CaptureCursor>().to(MouseButton::Left);
    actions.bind::<ReleaseCursor>().to(KeyCode::Escape);
}
