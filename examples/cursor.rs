use bevy::{prelude::*, sprite::Anchor};
use bevy_tiled_camera::*;

fn main() {
    App::new()
    .add_plugin(TiledCameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(check_cursor)
    .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    commands.spawn_bundle(
        TiledCameraBundle::unit_cam([10,10], 8)
        .with_clear_color(Color::GRAY)
    );

    // commands.spawn_bundle(Camera2dBundle {
    //     projection: OrthographicProjection {
    //         scale: 0.05,
    //         ..default()
    //     },
    //     ..default()
    // });
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(500000.0)),
            color: Color::BLUE,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: server.load("8x8.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        ..default()
    });
}

fn check_cursor(
    windows: Res<Windows>,
    q_cam: Query<(&Camera, &TiledCamera, &GlobalTransform, &Camera2d)>,
    //q_cam2: Query<(&Camera, &GlobalTransform)> 
) {
    for (cam, tcam, t, cam2d) in q_cam.iter() {
    //for (cam,t) in q_cam2.iter() {
        if let Some(window) = windows.get_primary() {
            if let Some(cursor_pos) = window.cursor_position() {

                if let Some(world_pos) = screen_to_world(cursor_pos, &cam, &t) {
                //if let Some(world_pos) = cam.world_to_ndc(camera_transform, world_position)
                //if let Some(world_pos) = from_screenspace(cursor_pos, cam, t) {
                    println!("Cursor pos {}. WorldPos {}", cursor_pos, world_pos);
                }

            }
        }
    }
}

