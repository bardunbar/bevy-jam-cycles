use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, ecs::query::QueryEntityError, prelude::*};
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{Cap, DiscPainter, LinePainter},
};

use crate::AppSet;

use super::spawn::{
    connection::{ConnectionAnchor, ConnectionTarget},
    planet::{OrbitalPosition, Planet, SatelliteProperties},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (render_orbits, render_satellites, render_connections)
            .chain()
            .in_set(AppSet::Render),
    );
}

fn render_orbits(
    mut painter: ShapePainter,
    planet_query: Query<(&Planet, &OrbitalPosition, &SatelliteProperties)>,
) {
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.set_color(Color::srgb(0.5, 0.5, 0.5));
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

fn render_satellites(
    mut painter: ShapePainter,
    planet_query: Query<(&Planet, &OrbitalPosition, &SatelliteProperties)>,
) {
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

fn render_connections(
    mut painter: ShapePainter,
    connection_query: Query<(&ConnectionAnchor, &ConnectionTarget)>,
    planet_query: Query<(&Planet, &OrbitalPosition, &SatelliteProperties)>,
) {
    painter.thickness = 0.5;
    painter.set_color(Color::Srgba(WHITE));

    fn get_position_from_planet(
        entity: Entity,
        planet_query: &Query<(&Planet, &OrbitalPosition, &SatelliteProperties)>,
    ) -> Result<Vec3, QueryEntityError> {
        let (_, orbital_position, _properties) = planet_query.get(entity)?;
        let start = Vec3::Y * orbital_position.radius;
        let rotation = Quat::from_rotation_z(-orbital_position.position);
        Ok(rotation * start)
    }

    for (connection_anchor, connection_target) in &connection_query {
        if let Ok(start) = get_position_from_planet(connection_anchor.satellite, &planet_query) {
            let end = match connection_target {
                ConnectionTarget::Satellite(target) => {
                    match get_position_from_planet(*target, &planet_query) {
                        Ok(pos) => pos,
                        Err(_) => Vec3::ZERO,
                    }
                }
                ConnectionTarget::Position(pos) => *pos,
            };

            painter.line(start, end);
        }
    }
}
