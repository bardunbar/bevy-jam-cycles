use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ResourceSpawnTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

    app.add_systems(Update, tick_resource_timer.in_set(AppSet::TickTimers));
    app.observe(process_spawn_resource);
}

#[derive(Resource)]
pub struct ResourceSpawnTimer {
    pub timer: Timer,
}

#[derive(Event)]
pub struct DoResourceSpawn;

#[derive(Component)]
pub struct ResourceSpawner {
    pub spawn_type: GameResource,
}

#[derive(Component)]
pub struct ResourceConsumer {
    pub demands: Vec<GameResource>,
}

#[derive(Debug, Clone, Copy)]
pub enum GameResource {
    Material,
}

#[derive(Component)]
pub struct ResourceContainer {
    pub resources: Vec<GameResource>,
}

fn tick_resource_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut resource_time: ResMut<ResourceSpawnTimer>,
) {
    resource_time.timer.tick(time.delta());

    if resource_time.timer.finished() {
        commands.trigger(DoResourceSpawn);
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

// fn calculate_resource_gradients(
// )
