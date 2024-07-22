use bevy::prelude::*;

use crate::screen::Screen;

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

fn spawn_planets(_trigger: Trigger<SpawnPlanets>, mut commands: Commands) {
    commands.spawn((
        Name::new("Planet"),
        Planet,
        StateScoped(Screen::Playing),
        OrbitalMovement { speed: 0.2 },
        OrbitalPosition {
            position: 1.23,
            radius: 64.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        StateScoped(Screen::Playing),
        OrbitalMovement { speed: 0.1 },
        OrbitalPosition {
            position: 4.22,
            radius: 128.0,
        },
    ));

    commands.spawn((
        Name::new("Planet"),
        Planet,
        StateScoped(Screen::Playing),
        OrbitalMovement { speed: 0.05 },
        OrbitalPosition {
            position: 0.3,
            radius: 256.0,
        },
    ));
}
