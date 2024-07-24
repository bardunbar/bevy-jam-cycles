use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{game::interaction::SatelliteInteraction, screen::Screen};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_planets);
}

#[derive(Event, Debug)]
pub struct SpawnPlanets;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Planet;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct OrbitalPosition {
    pub position: f32,
    pub radius: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct OrbitalMovement {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SatelliteProperties {
    pub radius: f32,
    pub color: Color,
}

fn spawn_planets(_trigger: Trigger<SpawnPlanets>, mut commands: Commands) {
    commands.spawn((
        Name::new("Planet"),
        Planet,
        SatelliteProperties {
            radius: 4.0,
            color: Color::Srgba(WHITE),
        },
        StateScoped(Screen::Playing),
        SatelliteInteraction::default(),
        OrbitalMovement { speed: 0.2 },
        OrbitalPosition {
            position: 1.23,
            radius: 64.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        SatelliteProperties {
            radius: 8.0,
            color: Color::Srgba(WHITE),
        },
        StateScoped(Screen::Playing),
        SatelliteInteraction::default(),
        OrbitalMovement { speed: 0.1 },
        OrbitalPosition {
            position: 4.22,
            radius: 128.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        SatelliteProperties {
            radius: 18.0,
            color: Color::Srgba(WHITE),
        },
        StateScoped(Screen::Playing),
        SatelliteInteraction::default(),
        OrbitalMovement { speed: 0.07 },
        OrbitalPosition {
            position: 5.22,
            radius: 200.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        SatelliteProperties {
            radius: 12.0,
            color: Color::Srgba(WHITE),
        },
        StateScoped(Screen::Playing),
        SatelliteInteraction::default(),
        OrbitalMovement { speed: 0.05 },
        OrbitalPosition {
            position: 0.3,
            radius: 256.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        SatelliteProperties {
            radius: 3.0,
            color: Color::Srgba(WHITE),
        },
        StateScoped(Screen::Playing),
        SatelliteInteraction::default(),
        OrbitalMovement { speed: 0.025 },
        OrbitalPosition {
            position: 5.4,
            radius: 300.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        SatelliteProperties {
            radius: 3.0,
            color: Color::Srgba(WHITE),
        },
        StateScoped(Screen::Playing),
        SatelliteInteraction::default(),
        OrbitalMovement { speed: 0.02 },
        OrbitalPosition {
            position: 5.5,
            radius: 336.0,
        },
    ));
}
