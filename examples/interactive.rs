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
    prelude::*,
    render::texture::Image,
    sprite::{SpriteBundle, Sprite},
    DefaultPlugins,
};

use bevy_tiled_camera::{TiledProjection, TiledCameraPlugin, TiledCameraBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledCameraPlugin)
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(spawn_sprites)
        .add_system(cursor_system)
        .run();
}

struct SpriteTextures {
    pub tex_8x8: Handle<Image>,
    pub tex_16x16: Handle<Image>,
    pub tex_32x32: Handle<Image>,
    pub current: u32,
}

#[derive(Component)]
struct TileCount {
    pub count: UVec2,
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let tile_count = (2,2);
    let cam_bundle = TiledCameraBundle::new()
        .with_centered(true)
        .with_pixels_per_tile(8)
        .with_tile_count(tile_count);

    commands.spawn_bundle(cam_bundle);

    let textures = SpriteTextures {
        tex_8x8: asset_server.load("8x8.png"),
        tex_16x16: asset_server.load("16x16.png"),
        tex_32x32: asset_server.load("32x32.png"),
        current: 0,
    };

    commands.spawn().insert(TileCount {
        count: tile_count.into(),
    });
    
    commands.insert_resource(textures);

    println!("Resize the window to see auto-scaling.");
    println!("Press spacebar to toggle camera center. Arrow keys to adjust number of tiles. Tab to change textures.");

    let col = Color::rgba(1.0,1.0,1.0,0.35);
    let cursor = SpriteBundle {
        sprite: Sprite {
            color: col,
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0,0.0,2.0),
        ..Default::default()
    };
    commands.spawn_bundle(cursor).insert(Cursor);
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
        q_cam.single_mut().set_tile_count(tile_count.count.into());
    }

    if input.just_pressed(KeyCode::Down) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.y = (tile_count.count.y - 1).max(1);
        q_cam.single_mut().set_tile_count(tile_count.count.into());
    }

    if input.just_pressed(KeyCode::Left) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.x = (tile_count.count.x - 1).max(1);
        q_cam.single_mut().set_tile_count(tile_count.count.into());
    }

    if input.just_pressed(KeyCode::Right) {
        let mut tile_count = q_tile_count.single_mut();
        tile_count.count.x += 1;
        q_cam.single_mut().set_tile_count(tile_count.count.into());
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

#[derive(Component)]
struct GridSprite;

fn spawn_sprites(
    mut commands: Commands,
    q_sprite_count_changed: Query<&TileCount, Changed<TileCount>>,
    q_camera_changed: Query<&TiledProjection, Changed<TiledProjection>>,
    q_sprite_count: Query<&TileCount>,
    q_camera: Query<(&GlobalTransform, &TiledProjection)>,
    sprites_query: Query<Entity, (With<Sprite>, With<GridSprite>)>,
    sprite_textures: Res<SpriteTextures>,
) {
    let sprite_count_changed = q_sprite_count_changed.get_single().is_ok();
    let cam_changed = q_camera_changed.get_single().is_ok();

    if sprite_count_changed || cam_changed {
        for entity in sprites_query.iter() {
            commands.entity(entity).despawn();
        }

        let sprite_count = q_sprite_count.single().count.as_ivec2();
        let (transform, proj) = q_camera.single();

        let min = match proj.centered {
            true => -(sprite_count / 2),
            false => IVec2::ZERO,
        };
        let max = match proj.centered {
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

                if let Some(p) = proj.tile_center_world(transform, (x,y)) {
                    let transform = Transform::from_translation(p);

                    let bundle = SpriteBundle {
                        sprite,
                        texture,
                        transform,
                        ..Default::default()
                    };
                    commands.spawn_bundle(bundle).insert(GridSprite);
                }
            }
        }
    }
}

#[derive(Component)]
struct Cursor;

fn cursor_system(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, &TiledProjection)>,
    mut q_cursor: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        for (cam, t, proj) in q_camera.iter() {
            if let Some(p) = proj.screen_to_world(cam, &windows, t, pos) {

                if let Some(mut p) = proj.world_to_tile_center(t, p) {
                    p.z = 2.0;

                    let (mut t, mut v) = q_cursor.single_mut(); 
                    v.is_visible = true;
    
                    t.translation = p;
                    return;
                }

            }
        }
    }

    println!("Not visible");
    let (_, mut v) = q_cursor.single_mut();
    v.is_visible = false;
}