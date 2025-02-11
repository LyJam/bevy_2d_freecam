use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::window::{Window, WindowMode};
use bevy::{color, prelude::*};

const ZOOM_SPEED: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;
const MIN_ZOOM: f32 = 0.1;

// Component to store camera state
#[derive(Component)]
struct CameraController {
    zoom: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_zoom, camera_pan))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera with controller component
    commands.spawn((Camera2d, CameraController { zoom: 1.0 }));

    commands.spawn(Sprite::from_color(
        color::Srgba::rgb(1.0, 1.0, 1.0),
        Vec2::new(100.0, 100.0),
    ));
}

// Handle mouse wheel for zooming
fn camera_zoom(
    mut query: Query<(&mut OrthographicProjection, &mut CameraController)>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    let scroll: f32 = accumulated_mouse_scroll.delta.y;

    if scroll == 0.0 {
        return;
    }

    for (mut projection, mut controller) in query.iter_mut() {
        // Adjust zoom speed as needed
        controller.zoom += scroll * ZOOM_SPEED;
        // Clamp zoom to reasonable values
        controller.zoom = controller.zoom.clamp(MIN_ZOOM, MAX_ZOOM);

        projection.scale = 1.0 / controller.zoom;
    }
}

// Handle right mouse button for panning
fn camera_pan(
    buttons: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
) {
    if buttons.pressed(MouseButton::Right) {
        let pan = accumulated_mouse_motion.delta;
        if pan == Vec2::ZERO {
            return;
        }

        for (mut transform, projection) in query.iter_mut() {
            // Pan speed is adjusted by zoom level
            transform.translation.x -= pan.x * projection.scale;
            transform.translation.y += pan.y * projection.scale;
        }
    }
}
