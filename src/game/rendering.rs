use std::f32::consts::PI;

use bevy::{color::palettes::css::DARK_ORANGE, ecs::query::QueryEntityError, prelude::*};
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{Cap, DiscPainter, LinePainter},
};

use crate::AppSet;

use super::spawn::{
    connection::{
        ConnectionAnchor, ConnectionProperties, ConnectionTarget, ConnectionUnderConstruction,
    },
    planet::{OrbitalPosition, Planet, SatelliteProperties},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            render_orbits,
            render_satellites,
            render_connections,
            render_construction_range,
        )
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
        let position = orbital_position.get_euclidean_position();

        painter.set_translation(position);
        painter.set_color(satellite_properties.color);
        painter.circle(satellite_properties.radius);
    }

    painter.set_translation(Vec3::ZERO);
}

fn render_connections(
    mut painter: ShapePainter,
    connection_query: Query<(&ConnectionAnchor, &ConnectionTarget, &ConnectionProperties)>,
    planet_query: Query<(&Planet, &OrbitalPosition, &SatelliteProperties)>,
) {
    painter.thickness = 0.5;

    fn get_position_from_planet(
        entity: Entity,
        planet_query: &Query<(&Planet, &OrbitalPosition, &SatelliteProperties)>,
    ) -> Result<Vec3, QueryEntityError> {
        let (_, orbital_position, _properties) = planet_query.get(entity)?;
        let start = Vec3::Y * orbital_position.radius;
        let rotation = Quat::from_rotation_z(-orbital_position.position);
        Ok(rotation * start)
    }

    for (connection_anchor, connection_target, connection_properties) in &connection_query {
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

            let color = if connection_properties.is_valid_range_sqr((end - start).length_squared())
            {
                connection_properties.color
            } else {
                connection_properties.invalid_color
            };

            painter.set_color(color);
            painter.line(start, end);
        }
    }
}

fn render_construction_range(
    mut painter: ShapePainter,
    construction_query: Query<
        (&ConnectionAnchor, &ConnectionProperties),
        With<ConnectionUnderConstruction>,
    >,
    planet_query: Query<&OrbitalPosition, With<Planet>>,
) {
    if let Ok((anchor, properties)) = construction_query.get_single() {
        if let Ok(orbital_position) = planet_query.get(anchor.satellite) {
            painter.thickness = 1.0;
            painter.hollow = true;
            painter.set_color(Color::Srgba(DARK_ORANGE));
            painter.set_translation(orbital_position.get_euclidean_position());
            painter.circle(properties.range);
            painter.set_translation(Vec3::ZERO);
        }
    }
}
