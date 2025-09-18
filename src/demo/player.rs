//! Player-specific behavior.

use avian3d::prelude::*;
use bevy::prelude::*;
//use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use bevy_trenchbroom::prelude::*;

//use crate::demo::input::PlayerInput;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_player);
}

#[point_class(base(Visibility, Transform), model("models/capsule/capsule.gltf"))]
pub struct Player;

const CAPSULE_LEN: f32 = 1.0;
const CAPSULE_RADIUS: f32 = 0.4;
pub const PLAYER_HEIGHT: f32 = CAPSULE_LEN + 2.0 * CAPSULE_RADIUS;
pub const PLAYER_FLOAT_OFFSET: f32 = 0.01;

fn spawn_player(trigger: On<Add, Player>, mut commands: Commands) {
    commands.entity(trigger.entity).insert((
        RigidBody::Dynamic,
        Collider::capsule(CAPSULE_RADIUS, CAPSULE_LEN),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(CAPSULE_RADIUS - 0.01, 0.0)),
        LockedAxes::ROTATION_LOCKED,
        //Actions::<PlayerInput>::default(),
        ColliderDensity(100.0),
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Multiply,
        },
    ));
}
