/// A simple interactive demo. Resize the window to see the viewport auto-adjust.
/// 
/// # Controls:
/// Spacebar - Toggle camera between centered or bottom-left origin
/// Arrow Keys - Adjust the number of tiles
/// Tab - Change the current tile textures
use bevy::{
    ecs::prelude::*,
    input::Input,
    math::{IVec2, UVec2, Vec2},
    prelude::{App, AssetServer, Handle, KeyCode, Transform},
    render2::texture::Image,
    sprite2::{PipelinedSpriteBundle, Sprite},
    PipelinedDefaultPlugins,
};

use bevy_tiled_camera::{TiledProjection, TiledCameraBuilder, TiledCameraPlugin};

fn main() {
    App::new()
        .add_plugins(PipelinedDefaultPlugins)
        .add_plugin(TiledCameraPlugin)
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(spawn_sprites)
        .run();
}

struct SpriteTextures {
    pub tex_8x8: Handle<Image>,
    pub tex_16x16: Handle<Image>,
    pub tex_32x32: Handle<Image>,
    pub current: u32,
}
struct TileCount {
    pub count: UVec2,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cam_bundle = TiledCameraBuilder::new()
        .with_centered(true)
        .with_tile_settings(8, (10, 10).into())
        .camera_bundle;

    commands.spawn_bundle(cam_bundle);

    commands.insert_resource(SpriteTextures {
        tex_8x8: asset_server.load("8x8.png"),
        tex_16x16: asset_server.load("16x16.png"),
        tex_32x32: asset_server.load("32x32.png"),
        current: 0,
    });

    commands.spawn().insert(TileCount {
        count: (10, 10).into(),
    });

    println!("Resize the window to see auto-scaling.");
    println!("Press spacebar to toggle camera center. Arrow keys to adjust number of tiles. Tab to change textures.");
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut q_cam: Query<&mut TiledProjection>,
    mut q_tile_count: Query<&mut TileCount>,
    mut sprite_textures: ResMut<SpriteTextures>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut cam = q_cam.single_mut();

        cam.centered = !cam.centered;
    }

    if input.just_pressed(KeyCode::Up) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.y += 1;
        q_cam.single_mut().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Down) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.y = (tile_count.count.y - 1).max(1);
        q_cam.single_mut().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Left) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.x = (tile_count.count.x - 1).max(1);
        q_cam.single_mut().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Right) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.x += 1;
        q_cam.single_mut().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Tab) {
        sprite_textures.current = (sprite_textures.current + 1) % 3;
        q_cam.single_mut().pixels_per_tile = match sprite_textures.current {
            1 => 16,
            2 => 32,
            _ => 8,
        };
    }
}

fn spawn_sprites(
    mut commands: Commands,
    q_sprite_count_changed: Query<&TileCount, Changed<TileCount>>,
    q_camera_changed: Query<&TiledProjection, Changed<TiledProjection>>,
    q_sprite_count: Query<&TileCount>,
    q_camera: Query<&TiledProjection>,
    sprites_query: Query<Entity, With<Sprite>>,
    sprite_textures: Res<SpriteTextures>,
) {
    let sprite_count_changed = q_sprite_count_changed.get_single().is_ok();
    let cam_changed = q_camera_changed.get_single().is_ok();

    if sprite_count_changed || cam_changed {
        for entity in sprites_query.iter() {
            commands.entity(entity).despawn();
        }

        let sprite_count = q_sprite_count.single().count.as_ivec2();
        let cam = q_camera.single();

        let min = match cam.centered {
            true => -(sprite_count / 2),
            false => IVec2::ZERO,
        };
        let max = match cam.centered {
            true => min + sprite_count,
            false => sprite_count,
        };
        for x in min.x..max.x {
            for y in min.y..max.y {
                let sprite = Sprite {
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                };
                let texture = match sprite_textures.current {
                    1 => sprite_textures.tex_16x16.clone(),
                    2 => sprite_textures.tex_32x32.clone(),
                    _ => sprite_textures.tex_8x8.clone(),
                };
                let transform = Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, 0.0);

                let bundle = PipelinedSpriteBundle {
                    sprite,
                    texture,
                    transform,
                    ..Default::default()
                };
                commands.spawn_bundle(bundle);
            }
        }
    }
}
