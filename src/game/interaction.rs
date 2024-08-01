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
            ConnectionAnchor, ConnectionTarget, ConnectionUnderConstruction, InitiateConnection,
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
    app.add_systems(Update, process_mouse.in_set(AppSet::RecordInput));
    app.add_systems(
        Update,
        (handle_interaction, play_interaction_sfx, spawn_connections).in_set(AppSet::Update),
    );
}

fn process_mouse(
    mut mouse_position: ResMut<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut interaction_query: Query<(
        &OrbitalPosition,
        &SatelliteProperties,
        &mut InteractionState,
    )>,
    // mut connection_interaction_query: Query<(
    //     &ConnectionAnchor,
    //     &ConnectionTarget,
    //     &mut InteractionState,
    // ), Without<ConnectionUnderConstruction>>
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_position.0 = world_position;
    }

    for (orbital_position, satellite_properties, mut satellite_interaction) in
        &mut interaction_query
    {
        let mut position = Vec3::Y * orbital_position.radius;
        let rotation = Quat::from_rotation_z(-orbital_position.position);
        position = rotation * position;

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

    // for (connection_anchor, connection_target, interaction) in &mut connection_interaction_query {
    //     // Get the endpoints of the line and check to see if the mouse is within a certain distance from the line

    // }
}

fn handle_interaction(
    mut planet_query: Query<
        (&mut SatelliteProperties, &InteractionState),
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
    satellite_query: Query<(Entity, &InteractionState), Changed<InteractionState>>,
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
