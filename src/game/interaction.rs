use bevy::{
    color::palettes::css::{DARK_RED, RED, WHITE},
    prelude::*,
    window::PrimaryWindow,
};

use crate::AppSet;

use super::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    spawn::planet::{OrbitalPosition, SatelliteProperties},
};

#[derive(Resource, Default)]
pub struct MousePosition(Vec2);

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub enum SatelliteInteraction {
    Pressed,
    Hovered,
    None,
}

impl Default for SatelliteInteraction {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl SatelliteInteraction {
    const DEFAULT: Self = Self::None;
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    app.add_systems(Update, process_mouse.in_set(AppSet::RecordInput));
    app.add_systems(
        Update,
        (handle_interaction, play_interaction_sfx).in_set(AppSet::Update),
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
        &mut SatelliteInteraction,
    )>,
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
                if *satellite_interaction != SatelliteInteraction::Pressed {
                    *satellite_interaction = SatelliteInteraction::Pressed;
                }
            } else if *satellite_interaction != SatelliteInteraction::Hovered {
                *satellite_interaction = SatelliteInteraction::Hovered;
            }
        } else if *satellite_interaction != SatelliteInteraction::None {
            *satellite_interaction = SatelliteInteraction::None;
        }
    }
}

fn handle_interaction(
    mut planet_query: Query<
        (&mut SatelliteProperties, &SatelliteInteraction),
        Changed<SatelliteInteraction>,
    >,
) {
    for (mut satellite_properties, interaction) in &mut planet_query {
        satellite_properties.color = match interaction {
            SatelliteInteraction::Pressed => Color::Srgba(RED),
            SatelliteInteraction::Hovered => Color::Srgba(DARK_RED),
            SatelliteInteraction::None => Color::Srgba(WHITE),
        };
    }
}

fn play_interaction_sfx(
    mut commands: Commands,
    planet_query: Query<&SatelliteInteraction, Changed<SatelliteInteraction>>,
) {
    for satllite_interaction in &planet_query {
        match satllite_interaction {
            SatelliteInteraction::Hovered => commands.trigger(PlaySfx::Key(SfxKey::ButtonHover)),
            SatelliteInteraction::Pressed => commands.trigger(PlaySfx::Key(SfxKey::ButtonPress)),
            _ => (),
        }
    }
}
