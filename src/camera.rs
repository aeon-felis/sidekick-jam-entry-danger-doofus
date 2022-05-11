use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_rapier2d::prelude::RigidBody;
use bevy_yoleck::{YoleckManaged, YoleckEditorState};

use crate::utils::some_or;

pub struct CameraPlugin {
    pub is_editor: bool,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        if false {
            app.add_startup_system(update_camera_scale_to_window);
            app.add_system_set(SystemSet::on_update(YoleckEditorState::GameActive).with_system(update_camera_transform));
        } else {
            app.add_system_set(SystemSet::new().with_system(update_camera_transform));
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    // camera.transform.translation.y = 0.0;
    // camera.transform.translation.z = 0.0;
    commands.spawn_bundle(camera);
}

fn update_camera_scale_to_window(
    mut query: Query<(&Camera, &mut OrthographicProjection)>,
    windows: Res<Windows>,
) {
    for (camera, mut projection) in query.iter_mut() {
        if let RenderTarget::Window(window_id) = camera.target {
            if let Some(window) = windows.get(window_id) {
                projection.scale = 15.0 / window.width();
            }
        }
    }
}

fn update_camera_transform(
    mut cameras_query: Query<(&Camera, &mut Transform, &OrthographicProjection)>,
    non_dynamic_objects_query: Query<(&GlobalTransform, &Sprite, &RigidBody), With<YoleckManaged>>,
) {
    let mut minmax: Option<[f32; 4]> = None;
    for (global_transform, sprite, rigid_body) in non_dynamic_objects_query.iter() {
        if *rigid_body == RigidBody::Dynamic {
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
    // let objects_botleft = Vec3::new(minmax[0], minmax[1], 0.0);
    // let objects_topright = Vec3::new(minmax[2], minmax[3], 0.0);
    let world_widgth = minmax[2] - minmax[0];
    for (_camera, mut transform, projection) in cameras_query.iter_mut() {
        let projection_width = projection.right - projection.left;
        transform.scale.x = world_widgth / projection_width;
        transform.scale.y = transform.scale.x;
        // info!("{} {}", projection_width, world_widgth);
        // let camera_botleft = Vec3::new(projection.left, projection.bottom, 0.0);
        // let camera_topright = Vec3::new(projection.right, projection.top, 0.0);
        // info!("{:?} {:?} {:?}", objects_topright, transform.mul_vec3(camera_topright), projection);
    }
}
