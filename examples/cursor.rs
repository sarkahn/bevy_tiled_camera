use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tiled_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TiledCameraPlugin))
        .add_systems(Update, test)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TiledCameraBundle::new().with_tile_count([15, 15]));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::splat(15.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn test(
    q_cam: Query<(&Camera, &TiledCamera, &GlobalTransform)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = primary_window.get_single() {
        if let Some(cursor) = window.cursor_position() {
            if let Ok((cam, tcam, t)) = q_cam.get_single() {
                if let Some(cpos) = tcam.screen_to_world(cursor, cam, t) {
                    println!("CPOS {}", cpos);
                }
            }
        }
    }
}
