//! Player-specific behavior.

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use bevy_trenchbroom::prelude::*;

use crate::demo::input::PlayerInput;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.add_observer(spawn_player);
}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(QuakeClass, Component)]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
#[model("models/capsule/capsule.gltf")]
#[base(Visibility, Transform)]
pub struct Player;

const CAPSULE_LEN: f32 = 1.0;
const CAPSULE_RADIUS: f32 = 0.4;
pub const PLAYER_HEIGHT: f32 = CAPSULE_LEN + 2.0 * CAPSULE_RADIUS;
pub const PLAYER_FLOAT_OFFSET: f32 = 0.3;

fn spawn_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert((
        RigidBody::Dynamic,
        Collider::capsule(CAPSULE_RADIUS, CAPSULE_LEN),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(CAPSULE_RADIUS - 0.01, 0.0)),
        LockedAxes::ROTATION_LOCKED,
        Actions::<PlayerInput>::default(),
        ColliderDensity(100.0),
    ));
}
