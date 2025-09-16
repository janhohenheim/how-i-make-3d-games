use bevy::prelude::*;

mod avian;
//mod enhanced_input;
mod framepace;
mod tnua;
mod trenchbroom;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        avian::plugin,
        trenchbroom::plugin,
        //enhanced_input::plugin,
        tnua::plugin,
        framepace::plugin,
    ));
}
