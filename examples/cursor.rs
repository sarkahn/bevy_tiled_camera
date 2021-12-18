use bevy::prelude::*;
use bevy_tiled_camera::*;

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    let cam_bundle = TiledCameraBundle::new()
    .with_pixels_per_tile(8)
    .with_tile_count((3,3));

    let proj = &cam_bundle.projection;
    let p = proj.tile_center_world(&GlobalTransform::default(), (0,0)).unwrap();

    let tex = server.load("8x8.png");

    let sprite_bundle = SpriteBundle {
        texture: tex,
        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        transform: Transform::from_translation(p),
        ..Default::default()
    };

    commands.spawn_bundle(sprite_bundle);

    commands.spawn_bundle(cam_bundle);
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TiledCameraPlugin)
    .add_startup_system(setup)
    .run();
}