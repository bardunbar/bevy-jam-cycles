//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::DiscPainter};

use crate::screen::Screen;

// use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    // app.observe(spawn_level);
    app.add_systems(Update, draw_level.run_if(in_state(Screen::Playing)));
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

// fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
//     // The only thing we have in our level is a player,
//     // but add things like walls etc. here.
//     commands.trigger(SpawnPlayer);
// }

fn draw_level(mut painter: ShapePainter) {
    painter.hollow = true;
    painter.thickness = 12.0;
    painter.set_color(Color::srgb(0.9, 0.9, 0.2));
    painter.circle(100.0);

    painter.hollow = false;
    painter.set_color(Color::srgb(1.0, 0.5, 0.0));
    painter.circle(88.0);
}