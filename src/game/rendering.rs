use std::f32::consts::PI;

use bevy::{color::palettes::css::DARK_ORANGE, ecs::query::QueryEntityError, prelude::*};
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{Cap, DiscPainter, LinePainter, RegularPolygonPainter},
};

use crate::AppSet;

use super::{
    interaction::InteractionState, resource::ResourceContainer, spawn::{
        connection::{
            ConnectionAnchor, ConnectionConfig, ConnectionProperties, ConnectionTarget,
            ConnectionUnderConstruction,
        },
        planet::{OrbitalPosition, Planet, SatelliteProperties},
    }
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            render_orbits,
            render_satellites,
            render_connections,
            render_resources,
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

            let distance = (end - start).length();
            let v = (distance - (connection_properties.range * 0.75))
                .clamp(0.0, connection_properties.range * 0.25);
            let nv = (v / (connection_properties.range * 0.25)).clamp(0.0, 1.0);
            let color = Color::srgb(1.0, 1.0 - nv, 1.0 - nv);

            painter.set_color(color);
            painter.line(start, end);
        }
    }
}

fn render_construction_range(
    mut painter: ShapePainter,
    connection_config: Res<ConnectionConfig>,
    construction_query: Query<&ConnectionAnchor, With<ConnectionUnderConstruction>>,
    planet_query: Query<&OrbitalPosition, With<Planet>>,
    hover_planet_query: Query<(&OrbitalPosition, &InteractionState)>,
) {
    if let Ok(anchor) = construction_query.get_single() {
        if let Ok(orbital_position) = planet_query.get(anchor.satellite) {
            painter.thickness = 1.0;
            painter.hollow = true;
            painter.set_color(Color::Srgba(DARK_ORANGE));
            painter.set_translation(orbital_position.get_euclidean_position());
            painter.circle(connection_config.range);
            painter.set_translation(Vec3::ZERO);
        }
    }

    if construction_query.is_empty() {
        for (orbital_position, interaction) in &hover_planet_query {
            if *interaction == InteractionState::Hovered {
                painter.thickness = 1.0;
                painter.hollow = true;
                painter.set_color(Color::Srgba(DARK_ORANGE));
                painter.set_translation(orbital_position.get_euclidean_position());
                painter.circle(connection_config.range);
                painter.set_translation(Vec3::ZERO);
            }
        }
    }
}

fn render_resources(
    mut painter: ShapePainter,
    planet_query: Query<(&OrbitalPosition, &SatelliteProperties, &ResourceContainer)>,
) {
    for (position, properties, container) in &planet_query {
        let pos = position.get_euclidean_position();
        let resource_pos = pos + Vec3::new(-properties.radius - 5.0, properties.radius + 5.0, 0.0);

        painter.set_translation(resource_pos);
        painter.roundness = 0.1;
        painter.hollow = false;

        for _resource in &container.resources {
            painter.ngon(3.0, 5.0);
            painter.translate(Vec3::Y * 5.0 * 2.0);
        }
    }
}
