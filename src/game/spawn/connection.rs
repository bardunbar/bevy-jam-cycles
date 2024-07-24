use bevy::prelude::*;

use crate::game::interaction::MousePosition;

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
pub struct ConnectionUnderConstruction;

pub(super) fn plugin(app: &mut App) {
    app.observe(initiate_connection);
    app.add_systems(
        Update,
        update_connections.run_if(resource_changed::<MousePosition>),
    );
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
        ConnectionUnderConstruction,
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
