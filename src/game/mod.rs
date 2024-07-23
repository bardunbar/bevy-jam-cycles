//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod interaction;
mod movement;
pub mod rendering;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        rendering::plugin,
        interaction::plugin,
    ));
}
