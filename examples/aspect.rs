use bevy::{prelude::*, render::{camera::ScalingMode, texture::ImageSettings}};
use sark_grids::Size2d;

fn main() {
    App::new()
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: server.load("4x8.png"),
        ..default()
    });

    let cam = Camera2dBundle {
        projection: make_proj([8,8]),
        ..default()
    };

    commands.spawn_bundle(cam);
}

fn make_proj(world_size: impl Size2d) -> OrthographicProjection {
    let world_size = world_size.as_vec2();

    let half = world_size / 2.0;

    OrthographicProjection {
        scaling_mode: ScalingMode::None,
        left : -half.x,
        bottom : -half.y,
        right : -half.x + world_size.x,
        top : -half.y + world_size.y,
        ..default()
    }
}