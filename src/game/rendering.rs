use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{Cap, DiscPainter},
};

use crate::AppSet;

use super::spawn::planet::{OrbitalPosition, Planet, PlanetProperties};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (render_orbits, render_satellites).chain().in_set(AppSet::Render));
}

fn render_orbits(mut painter: ShapePainter, planet_query: Query<(&Planet, &OrbitalPosition, &PlanetProperties)>) {
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.set_color(Color::srgb(1.0, 1.0, 1.0));
    painter.cap = Cap::None;

    for (_, orbital_position, satellite_properties) in &planet_query {
        // This is the half length because we need to multiply the radius by two but then divide by two to get the half;
        let half_arc_length = (satellite_properties.radius + 10.0) / orbital_position.radius;
        painter.arc(
            orbital_position.radius,
            orbital_position.position + half_arc_length,
            orbital_position.position + 2.0 * PI - half_arc_length,
        );
    }
}

fn render_satellites(mut painter: ShapePainter, planet_query: Query<(&Planet, &OrbitalPosition, &PlanetProperties)>) {
    painter.hollow = false;

    for (_, orbital_position, satellite_properties) in &planet_query {
        let mut position = Vec3::Y * orbital_position.radius;
        let rotation = Quat::from_rotation_z(-orbital_position.position);

        position = rotation * position;

        painter.set_translation(position);
        painter.set_color(satellite_properties.color);
        painter.circle(satellite_properties.radius);
    }

    painter.set_translation(Vec3::ZERO);
}