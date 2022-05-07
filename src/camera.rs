use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_system(update_camera_scale_to_window);
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
