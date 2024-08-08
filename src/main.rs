// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_jam_cycles::AppPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(AppPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Off)
        .run()
}
