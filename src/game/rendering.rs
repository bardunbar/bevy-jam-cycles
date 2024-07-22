use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{Cap, DiscPainter},
};

use crate::AppSet;

use super::spawn::planet::{OrbitalPosition, Planet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, render_planets.in_set(AppSet::Render));
}

fn render_planets(mut painter: ShapePainter, planet_query: Query<(&Planet, &OrbitalPosition)>) {
    for (_, orbital_position) in &planet_query {
        painter.thickness = 0.5;
        painter.hollow = true;
        painter.set_color(Color::srgb(1.0, 1.0, 1.0));
        painter.cap = Cap::None;

        painter.arc(
            orbital_position.radius,
            orbital_position.position + 0.1,
            orbital_position.position + 2.0 * PI - 0.1,
        );
    }
}
