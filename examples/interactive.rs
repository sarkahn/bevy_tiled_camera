// /// A simple interactive demo. Resize the window to see the viewport auto-adjust.
// ///
// /// # Controls:
// /// Spacebar - Toggle camera between centered or bottom-left origin
// /// Arrow Keys - Adjust the number of tiles
// /// Tab - Change the current tile textures

use bevy::prelude::*;
use bevy_tiled_camera::{TiledCameraBundle, TiledCameraPlugin, TiledProjection};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledCameraPlugin)
        .add_startup_system(setup.system())
        .add_system(handle_input.system())
        .add_system(spawn_sprites.system())
        .run();
}

struct SpriteTextures {
    pub tex_8x8: Handle<ColorMaterial>,
    pub tex_16x16: Handle<ColorMaterial>,
    pub tex_32x32: Handle<ColorMaterial>,
    pub current: u32,
}
struct TileCount {
    pub count: UVec2,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cam_bundle = TiledCameraBundle::new()
        .with_centered(true)
        .with_pixels_per_tile(8)
        .with_tile_count((10, 10).into());

    commands.spawn_bundle(cam_bundle);

    commands.insert_resource(SpriteTextures {
        tex_8x8: materials.add(ColorMaterial::texture(asset_server.load("8x8.png"))),
        tex_16x16: materials.add(ColorMaterial::texture(asset_server.load("16x16.png"))),
        tex_32x32: materials.add(ColorMaterial::texture(asset_server.load("32x32.png"))),
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
        let mut cam = q_cam.single_mut().unwrap();

        cam.centered = !cam.centered;
    }

    if input.just_pressed(KeyCode::Up) {
        let mut tile_count = q_tile_count.single_mut().unwrap();
        tile_count.count.y += 1;
        q_cam.single_mut().unwrap().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Down) {
        let mut tile_count = q_tile_count.single_mut().unwrap();
        tile_count.count.y = (tile_count.count.y - 1).max(1);
        q_cam.single_mut().unwrap().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Left) {
        let mut tile_count = q_tile_count.single_mut().unwrap();
        tile_count.count.x = (tile_count.count.x - 1).max(1);
        q_cam.single_mut().unwrap().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Right) {
        let mut tile_count = q_tile_count.single_mut().unwrap();
        tile_count.count.x += 1;
        q_cam.single_mut().unwrap().target_tile_count = tile_count.count;
    }

    if input.just_pressed(KeyCode::Tab) {
        sprite_textures.current = (sprite_textures.current + 1) % 3;
        q_cam.single_mut().unwrap().pixels_per_tile = match sprite_textures.current {
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
    let sprite_count_changed = q_sprite_count_changed.single().is_ok(); //q_sprite_count_changed.get_single().is_ok();
    let cam_changed = q_camera_changed.single().is_ok();

    if sprite_count_changed || cam_changed {
        for entity in sprites_query.iter() {
            commands.entity(entity).despawn();
        }

        let sprite_count = q_sprite_count.single().unwrap().count;
        let cam = q_camera.single().unwrap();
        let sprite_count = IVec2::new(sprite_count.x as i32, sprite_count.y as i32);

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
                    size: Vec2::ONE,
                    resize_mode: SpriteResizeMode::Manual,
                    //custom_size: Some(Vec2::ONE),
                    ..Default::default()
                };
                let material = match sprite_textures.current {
                    1 => sprite_textures.tex_16x16.clone(),
                    2 => sprite_textures.tex_32x32.clone(),
                    _ => sprite_textures.tex_8x8.clone(),
                };

                let transform = Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, 0.0);

                let bundle = SpriteBundle {
                    sprite,
                    material,
                    transform,
                    ..Default::default()
                };
                commands.spawn_bundle(bundle);
            }
        }
    }
}
