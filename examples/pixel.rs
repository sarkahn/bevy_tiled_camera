use bevy::{prelude::*, sprite::Anchor};
use bevy_tiled_camera::*;

fn main() {
    App::new()
        .add_plugin(TiledCameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run()
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(TiledCameraBundle::pixel_cam([10, 10], [8,8]));

    commands.spawn_bundle(SpriteBundle {
        texture: server.load("8x8.png"),
        sprite: Sprite {
            anchor: Anchor::TopRight,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: server.load("16x16.png"),
        sprite: Sprite {
            anchor: Anchor::TopLeft,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: server.load("32x32.png"),
        sprite: Sprite {
            anchor: Anchor::BottomCenter,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(500000.0)),
            color: Color::TEAL,
            ..default()
        },
        ..default()
    });
}
