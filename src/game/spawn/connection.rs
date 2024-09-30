use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};

use crate::{
    game::interaction::{InteractionState, MousePosition},
    AppSet,
};

use super::planet::{OrbitalPosition, Planet};

#[derive(Event, Debug)]
pub struct InitiateConnection(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConnectionAnchor {
    pub satellite: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum ConnectionTarget {
    Satellite(Entity),
    Position(Vec3),
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConnectionProperties {
    pub color: Color,
    pub invalid_color: Color,
    pub range: f32,
}

impl ConnectionProperties {
    pub fn is_valid_range_sqr(&self, range_sqr: f32) -> bool {
        range_sqr <= self.range * self.range
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConnectionUnderConstruction;

#[derive(Resource, Default)]
pub struct ConnectionConfig {
    pub range: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ConnectionConfig { range: 200.0 });
    app.observe(initiate_connection);
    app.add_systems(
        Update,
        update_connections.run_if(resource_changed::<MousePosition>),
    );
    app.add_systems(Update, check_for_invalid_connections.in_set(AppSet::PrepareUpdate));
}

fn initiate_connection(
    trigger: Trigger<InitiateConnection>,
    mouse_pos: Res<MousePosition>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Connection"),
        ConnectionAnchor {
            satellite: trigger.event().0,
        },
        ConnectionTarget::Position(mouse_pos.get_pos_3d()),
        ConnectionProperties {
            color: Color::Srgba(WHITE),
            invalid_color: Color::Srgba(RED),
            range: 200.0,
        },
        ConnectionUnderConstruction,
        InteractionState::default(),
    ));
}

fn update_connections(
    mouse_position: Res<MousePosition>,
    mut query: Query<&mut ConnectionTarget, With<ConnectionUnderConstruction>>,
) {
    for mut connection_target in &mut query {
        *connection_target = ConnectionTarget::Position(mouse_position.get_pos_3d());
    }
}

fn check_for_invalid_connections(
    mut commands: Commands,
    connection_query: Query<(
        &ConnectionAnchor,
        &ConnectionTarget,
        &ConnectionProperties,
        Entity,
    )>,
    planet_query: Query<&OrbitalPosition, With<Planet>>,
) {
    for (anchor, target, properties, entity) in &connection_query {
        // Check the length of the connection
        if let Ok(orbital_position) = planet_query.get(anchor.satellite) {
            let start = orbital_position.get_euclidean_position();
            let end = match *target {
                ConnectionTarget::Satellite(target_planet) => {
                    if let Ok(target_position) = planet_query.get(target_planet) {
                        target_position.get_euclidean_position()
                    } else {
                        Vec3::ZERO
                    }
                }
                ConnectionTarget::Position(pos) => pos,
            };

            if !properties.is_valid_range_sqr((end - start).length_squared()) {
                if let ConnectionTarget::Satellite(_) = target {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
