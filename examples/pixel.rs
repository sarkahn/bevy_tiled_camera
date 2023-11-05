use bevy::{prelude::*, sprite::Anchor};
use bevy_tiled_camera::*;

fn main() {
    App::new()
        .add_plugins((
            TiledCameraPlugin,
            DefaultPlugins.set(ImagePlugin::default_nearest()),
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    // Defaults to 8x8 pixels per tile
    commands.spawn(TiledCameraBundle::pixel_cam([10, 10]));

    commands.spawn(SpriteBundle {
        texture: server.load("8x8.png"),
        sprite: Sprite {
            anchor: Anchor::TopRight,
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: server.load("16x16.png"),
        sprite: Sprite {
            anchor: Anchor::TopLeft,
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: server.load("32x32.png"),
        sprite: Sprite {
            anchor: Anchor::BottomCenter,
            ..default()
        },
        ..default()
    });
}
