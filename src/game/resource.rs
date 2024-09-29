use std::collections::HashSet;

use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::spawn::{
    connection::{ConnectionAnchor, ConnectionTarget, ConnectionUnderConstruction},
    planet::OrbitalPosition,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ResourceSpawnTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

    app.insert_resource(ResourceTransportTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

    app.add_systems(Update, tick_resource_timers.in_set(AppSet::TickTimers));
    app.add_systems(Update, tick_transport_timers.in_set(AppSet::TickTimers));
    app.add_systems(
        Update,
        (
            process_unclaimed_resources,
            (update_transport, process_transit_stops).chain(),
        )
            .in_set(AppSet::Update),
    );
    app.add_systems(Update, process_demands.in_set(AppSet::PrepareUpdate));

    app.observe(process_spawn_resource);
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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResource {
    Material,
}

#[derive(Component)]
pub struct GameResourceInStorage {
    pub satellite: Entity,
}

#[derive(Component)]
pub struct GameResourceInTransit {
    pub route: Vec<Entity>,
    pub claim: Entity,
    pub position: f32,
}

#[derive(Component)]
struct UpdateProgress;

#[derive(Component)]
pub struct GameResourceDemand {
    pub satellite: Entity,
    pub claim: Option<Entity>,
}

#[derive(Component)]
pub struct ResourceContainer {
    // pub resources: Vec<GameResource>,
    pub storage_count: usize,
    pub storage_size: usize,
}

#[derive(Bundle)]
struct GameResourceBundle {
    resource: GameResource,
    storage: GameResourceInStorage,
}

#[derive(Bundle)]
struct GameResourceDemandBundle {
    resource: GameResource,
    demand: GameResourceDemand,
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

fn process_demands(
    mut commands: Commands,
    mut consumer_query: Query<(Entity, &mut ResourceConsumer), Changed<ResourceConsumer>>,
) {
    for (satellite, mut consumer) in consumer_query.iter_mut() {
        if !consumer.demands.is_empty() {
            for demand in consumer.demands.iter() {
                commands.spawn((
                    GameResourceDemandBundle {
                        resource: *demand,
                        demand: GameResourceDemand {
                            satellite,
                            claim: None,
                        },
                    },
                    StateScoped(Screen::Playing),
                ));
            }

            consumer.demands.clear();
        }
    }
}

fn process_spawn_resource(
    _trigger: Trigger<DoResourceSpawn>,
    mut commands: Commands,
    mut spawner_query: Query<(Entity, &ResourceSpawner, &mut ResourceContainer)>,
) {
    for (entity, spawner, mut container) in &mut spawner_query {
        if container.storage_count < container.storage_size {
            commands.spawn((
                GameResourceBundle {
                    resource: spawner.spawn_type,
                    storage: GameResourceInStorage { satellite: entity },
                },
                StateScoped(Screen::Playing),
            ));
            container.storage_count += 1;

            info!("Created game resource: {:?}", spawner.spawn_type);
        }
    }
}

fn process_unclaimed_resources(
    mut commands: Commands,
    resource_in_storage_query: Query<(Entity, &GameResource, &GameResourceInStorage)>,
    mut demand_query: Query<(&mut GameResourceDemand, &GameResource, Entity)>,
    mut container_query: Query<&mut ResourceContainer>,
    connection_query: Query<
        (&ConnectionAnchor, &ConnectionTarget, Entity),
        Without<ConnectionUnderConstruction>,
    >,
) {
    let mut claimed_this_frame = HashSet::new();

    for (mut demand, demanded_resource, demand_entity) in &mut demand_query {
        if demand.claim.is_none() {
            // Search for a resource in storage that is unclaimed
            let mut visited = HashSet::new();
            let mut open_list: Vec<(Entity, Vec<Entity>)> = Vec::new();
            open_list.push((demand.satellite, vec![demand.satellite]));

            'search_for_claim: while !open_list.is_empty() {
                let (cur_planet, mut planet_path) = open_list.remove(0);
                visited.insert(cur_planet);

                // Check for any resources
                for (resource_entity, resource, storage) in &resource_in_storage_query {
                    if !claimed_this_frame.contains(&resource_entity)
                        && resource == demanded_resource
                        && storage.satellite == cur_planet
                    {
                        demand.claim = Some(resource_entity);

                        planet_path.reverse();
                        commands
                            .entity(resource_entity)
                            .remove::<GameResourceInStorage>()
                            .insert(GameResourceInTransit {
                                route: planet_path,
                                claim: demand_entity,
                                position: 0.0,
                            });

                        if let Ok(mut container) = container_query.get_mut(storage.satellite) {
                            if container.storage_count > 0 {
                                container.storage_count -= 1;
                            } else {
                                error!("Storage was empty when resource was removed!")
                            }
                        }

                        claimed_this_frame.insert(resource_entity);
                        break 'search_for_claim;
                    }
                }

                // Otherwise, keep searching
                for (anchor, target, _) in &connection_query {
                    if let ConnectionTarget::Satellite(target_entity) = target {
                        if anchor.satellite == cur_planet {
                            if !visited.contains(target_entity) {
                                let mut new_path = planet_path.clone();
                                new_path.push(*target_entity);
                                open_list.push((*target_entity, new_path));
                            }
                        } else if *target_entity == cur_planet
                            && !visited.contains(&anchor.satellite)
                        {
                            let mut new_path = planet_path.clone();
                            new_path.push(anchor.satellite);
                            open_list.push((anchor.satellite, new_path));
                        }
                    }
                }
            }
        }
    }
}

const TRANSPORT_SPEED: f32 = 40.0;

fn update_transport(
    mut commands: Commands,
    time: Res<Time>,
    mut transporting_query: Query<(Entity, &mut GameResourceInTransit)>,
    planet_query: Query<&OrbitalPosition>,
) {
    for (resource_entity, mut transit) in transporting_query.iter_mut() {
        let start = planet_query
            .get(transit.route[0])
            .unwrap()
            .get_euclidean_position();
        let end = planet_query
            .get(transit.route[1])
            .unwrap()
            .get_euclidean_position();
        let distance = start.distance(end);

        let current = distance * transit.position;
        let next = current + time.delta().as_secs_f32() * TRANSPORT_SPEED;

        transit.position = next / distance;

        if transit.position >= 1.0 {
            commands.entity(resource_entity).insert(UpdateProgress);
        }
    }
}

fn process_transit_stops(
    mut commands: Commands,
    mut transporting_query: Query<(Entity, &mut GameResourceInTransit), With<UpdateProgress>>,
) {
    for (entity, mut transit) in transporting_query.iter_mut() {
        transit.route.remove(0);

        if transit.route.len() < 2 {
            // We have arrived at our destination! Attempt to process the claim!
            commands.entity(transit.claim).despawn();
            commands.entity(entity).despawn();
        } else {
            info!("Resource arrived at mid point, time to continue");
            // We are part way to our destination... verify our path's integrity
            transit.position = 0.0;
            commands.entity(entity).remove::<UpdateProgress>();
        }
    }
}
