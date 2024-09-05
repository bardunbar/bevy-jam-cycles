use std::collections::HashSet;

use bevy::prelude::*;

use crate::AppSet;

use super::spawn::connection::{ConnectionAnchor, ConnectionTarget, ConnectionUnderConstruction};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ResourceSpawnTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

    app.insert_resource(ResourceTransportTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

    app.add_systems(Update, tick_resource_timers.in_set(AppSet::TickTimers));
    app.add_systems(Update, tick_transport_timers.in_set(AppSet::TickTimers));

    app.observe(process_spawn_resource);
    app.observe(do_resource_transport);
}

#[derive(Resource)]
pub struct ResourceSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct ResourceTransportTimer {
    pub timer: Timer,
}

#[derive(Event)]
pub struct DoResourceSpawn;

#[derive(Event)]
pub struct DoResourceTransport;

#[derive(Component)]
pub struct ResourceSpawner {
    pub spawn_type: GameResource,
}

#[derive(Component)]
pub struct ResourceConsumer {
    pub demands: Vec<GameResource>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResource {
    Material,
}

#[derive(Component)]
pub struct ResourceContainer {
    pub resources: Vec<GameResource>,
}

fn tick_resource_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut resource_time: ResMut<ResourceSpawnTimer>,
) {
    resource_time.timer.tick(time.delta());

    if resource_time.timer.finished() {
        commands.trigger(DoResourceSpawn);
    }
}

fn tick_transport_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut transport_time: ResMut<ResourceTransportTimer>,
) {
    transport_time.timer.tick(time.delta());

    if transport_time.timer.finished() {
        commands.trigger(DoResourceTransport);
    }
}

fn process_spawn_resource(
    _trigger: Trigger<DoResourceSpawn>,
    mut spawner_query: Query<(&ResourceSpawner, &mut ResourceContainer)>,
) {
    for (spawner, mut container) in &mut spawner_query {
        if container.resources.len() < 6 {
            container.resources.push(spawner.spawn_type);
            info!("Created game resource: {:?}", spawner.spawn_type);
        }
    }
}

fn do_resource_transport(
    _trigger: Trigger<DoResourceTransport>,
    mut container_query: Query<(&mut ResourceContainer, Entity)>,
    mut consumer_query: Query<&mut ResourceConsumer>,
    connection_query: Query<
        (&ConnectionAnchor, &ConnectionTarget, Entity),
        Without<ConnectionUnderConstruction>,
    >,
) {
    for (mut container, entity) in &mut container_query {
        if !container.resources.is_empty() {
            // Attempt to find a consumer for the first resource
            let cur_resource = &container.resources[0];

            let mut visited = HashSet::new();
            let mut open_list = Vec::new();
            open_list.push(entity);

            let mut target_consumer: Option<Entity> = None;

            while !open_list.is_empty() {
                let cur_planet = open_list.remove(0);
                visited.insert(cur_planet);

                // Check for any consumers
                if let Ok(consumer) = consumer_query.get(cur_planet) {
                    if consumer.demands.contains(cur_resource) {
                        target_consumer = Some(cur_planet);
                        break;
                    }
                }

                for (anchor, target, _) in &connection_query {
                    if let ConnectionTarget::Satellite(target_entity) = target {
                        if anchor.satellite == cur_planet {
                            if !visited.contains(target_entity) {
                                open_list.push(*target_entity);
                            }
                        } else if *target_entity == cur_planet
                            && !visited.contains(&anchor.satellite)
                        {
                            open_list.push(anchor.satellite);
                        }
                    }
                }
            }

            if let Some(consumer_entity) = target_consumer {
                if let Ok(mut consumer) = consumer_query.get_mut(consumer_entity) {
                    if let Some(index) = consumer.demands.iter().position(|&e| e == *cur_resource) {
                        consumer.demands.remove(index);
                        container.resources.remove(0);
                    }
                }
            }
        }
    }
}

// fn calculate_resource_gradients(
// )
