use bevy::{
    color::palettes::css::{DARK_RED, RED, WHITE},
    prelude::*,
    window::PrimaryWindow,
};

use crate::AppSet;

use super::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    spawn::{
        connection::{
            ConnectionAnchor, ConnectionProperties, ConnectionTarget, ConnectionUnderConstruction,
            InitiateConnection,
        },
        planet::{OrbitalPosition, SatelliteProperties},
    },
};

#[derive(Resource, Default)]
pub struct MousePosition(Vec2);

impl MousePosition {
    pub fn get_pos_3d(&self) -> Vec3 {
        Vec3::new(self.0.x, self.0.y, 0.0)
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub enum InteractionState {
    Pressed,
    Hovered,
    None,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl InteractionState {
    const DEFAULT: Self = Self::None;
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    app.add_systems(
        Update,
        (
            process_mouse,
            process_satellite_interactions,
            process_connection_interactions,
        )
            .chain()
            .in_set(AppSet::RecordInput),
    );
    app.add_systems(
        Update,
        (
            handle_interaction,
            play_interaction_sfx,
            spawn_connections,
            remove_connections,
        )
            .in_set(AppSet::Update),
    );
}

fn process_satellite_interactions(
    mouse_position: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut interaction_query: Query<(
        &OrbitalPosition,
        &SatelliteProperties,
        &mut InteractionState,
    )>,
) {
    for (orbital_position, satellite_properties, mut satellite_interaction) in
        &mut interaction_query
    {
        let position = orbital_position.get_euclidean_position();

        let effective_radius = satellite_properties.radius + 10.0;
        let delta = mouse_position.0 - position.xy();
        if delta.length_squared() < (effective_radius * effective_radius) {
            if mouse_button.pressed(MouseButton::Left) {
                if *satellite_interaction != InteractionState::Pressed {
                    *satellite_interaction = InteractionState::Pressed;
                }
            } else if *satellite_interaction != InteractionState::Hovered {
                *satellite_interaction = InteractionState::Hovered;
            }
        } else if *satellite_interaction != InteractionState::None {
            *satellite_interaction = InteractionState::None;
        }
    }
}

fn process_connection_interactions(
    mouse_position: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    satellite_query: Query<(&OrbitalPosition, &SatelliteProperties)>,
    mut interaction_query: Query<
        (&ConnectionAnchor, &ConnectionTarget, &mut InteractionState),
        Without<ConnectionUnderConstruction>,
    >,
) {
    for (connection_anchor, connection_target, mut interaction) in &mut interaction_query {
        // Get the endpoints of the line and check to see if the mouse is within a certain distance from the line
        if let Ok((anchor_position, _)) = satellite_query.get(connection_anchor.satellite) {
            if let ConnectionTarget::Satellite(target) = connection_target {
                if let Ok((target_position, _)) = satellite_query.get(*target) {
                    let start = anchor_position.get_euclidean_position().xy();
                    let end = target_position.get_euclidean_position().xy();
                    let mouse = mouse_position.0;

                    let line = end - start;
                    let p = mouse - start;

                    let h = (p.dot(line) / line.dot(line)).clamp(0.3, 0.7);
                    let dist = (p - line * h).length_squared();

                    if dist < (10.0 * 10.0) {
                        if mouse_button.pressed(MouseButton::Left) {
                            if *interaction != InteractionState::Pressed {
                                *interaction = InteractionState::Pressed;
                            }
                        } else if *interaction != InteractionState::Hovered {
                            *interaction = InteractionState::Hovered;
                        }
                    } else if *interaction != InteractionState::None {
                        *interaction = InteractionState::None;
                    }
                }
            }
        }
    }
}

fn process_mouse(
    mut mouse_position: ResMut<MousePosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_position.0 = world_position;
    }
}

fn handle_interaction(
    mut planet_query: Query<
        (&mut SatelliteProperties, &InteractionState),
        Changed<InteractionState>,
    >,
    mut connection_query: Query<
        (&mut ConnectionProperties, &InteractionState),
        Changed<InteractionState>,
    >,
) {
    for (mut satellite_properties, interaction) in &mut planet_query {
        satellite_properties.color = match interaction {
            InteractionState::Pressed => Color::Srgba(RED),
            InteractionState::Hovered => Color::Srgba(DARK_RED),
            InteractionState::None => Color::Srgba(WHITE),
        };
    }

    for (mut connection_properties, interaction) in &mut connection_query {
        connection_properties.color = match interaction {
            InteractionState::Pressed => Color::Srgba(RED),
            InteractionState::Hovered => Color::Srgba(DARK_RED),
            InteractionState::None => Color::Srgba(WHITE),
        };
    }
}

fn play_interaction_sfx(
    mut commands: Commands,
    planet_query: Query<&InteractionState, Changed<InteractionState>>,
) {
    for satllite_interaction in &planet_query {
        match satllite_interaction {
            InteractionState::Hovered => commands.trigger(PlaySfx::Key(SfxKey::ButtonHover)),
            InteractionState::Pressed => commands.trigger(PlaySfx::Key(SfxKey::ButtonPress)),
            _ => (),
        }
    }
}

fn spawn_connections(
    mut commands: Commands,
    mut connection_query: Query<
        (Entity, &mut ConnectionTarget, &ConnectionAnchor),
        With<ConnectionUnderConstruction>,
    >,
    satellite_query: Query<
        (Entity, &InteractionState),
        (With<SatelliteProperties>, Changed<InteractionState>),
    >,
) {
    for (entity, interaction) in &satellite_query {
        if *interaction == InteractionState::Pressed {
            if connection_query.is_empty() {
                commands.trigger(InitiateConnection(entity));
            } else if let Ok((connection, mut target, anchor)) = connection_query.get_single_mut() {
                if anchor.satellite == entity {
                    commands.entity(connection).despawn();
                } else {
                    *target = ConnectionTarget::Satellite(entity);
                    commands
                        .entity(connection)
                        .remove::<ConnectionUnderConstruction>();
                }
            }
        }
    }
}

fn remove_connections(
    mut commands: Commands,
    connection_query: Query<
        (Entity, &InteractionState),
        (With<ConnectionProperties>, Changed<InteractionState>),
    >,
) {
    for (entity, interaction) in &connection_query {
        if *interaction == InteractionState::Pressed {
            commands.entity(entity).despawn();
        }
    }
}
