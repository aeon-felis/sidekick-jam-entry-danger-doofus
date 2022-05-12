use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;
use bevy_yoleck::{YoleckEditorState, YoleckManaged};

use crate::utils::some_or;

pub struct CameraPlugin {
    pub is_editor: bool,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_system_set(
            SystemSet::on_update(YoleckEditorState::GameActive)
                .with_system(update_camera_transform),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.z = 100.0;
    camera.transform.scale = Vec3::new(0.016, 0.016, 1.0);
    commands.spawn_bundle(camera);
}

fn update_camera_transform(
    mut cameras_query: Query<(&Camera, &mut Transform, &OrthographicProjection)>,
    non_dynamic_objects_query: Query<(&GlobalTransform, &Sprite, &RigidBody), With<YoleckManaged>>,
) {
    let mut minmax: Option<[f32; 4]> = None;
    for (global_transform, sprite, rigid_body) in non_dynamic_objects_query.iter() {
        if *rigid_body != RigidBody::Fixed {
            continue;
        }
        let half_size = 0.5 * sprite.custom_size.unwrap().extend(0.0);
        let min_corner = global_transform.mul_vec3(-half_size);
        let max_corner = global_transform.mul_vec3(half_size);
        minmax = if let Some([l, b, r, t]) = minmax {
            Some([
                l.min(min_corner.x),
                b.min(min_corner.y),
                r.max(max_corner.x),
                t.max(max_corner.y),
            ])
        } else {
            Some([min_corner.x, min_corner.y, max_corner.x, max_corner.y])
        };
    }
    let minmax = some_or!(minmax; return);
    let world_width = minmax[2] - minmax[0];
    let world_height = minmax[3] - minmax[1];
    for (_camera, mut transform, projection) in cameras_query.iter_mut() {
        let projection_width = projection.right - projection.left;
        let projection_height = projection.top - projection.bottom;
        let width_ratio = world_width / projection_width;
        let height_ratio = world_height / projection_height;
        let chosen_ratio = width_ratio.max(height_ratio) * 1.1;
        transform.scale = Vec3::new(chosen_ratio, chosen_ratio, 1.0);
        transform.translation.x = 0.5 * (minmax[0] + minmax[2]);
        transform.translation.y = 0.5 * (minmax[1] + minmax[3]);
    }
}
