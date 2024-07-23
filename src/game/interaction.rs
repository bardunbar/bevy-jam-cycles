
use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

#[derive(Resource, Default)]
pub struct MousePosition(Vec2);

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    app.add_systems(Update, process_mouse.in_set(AppSet::RecordInput));
}

fn process_mouse(
    mut mouse_position: ResMut<MousePosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>
) {

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
            mouse_position.0 = world_position;
        }
}