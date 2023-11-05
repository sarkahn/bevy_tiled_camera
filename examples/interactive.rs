/// A simple interactive demo. Resize the window to see the viewport auto-adjust.
///
/// # Controls:
/// Spacebar - Toggle camera between centered or bottom-left origin
/// Arrow Keys - Adjust the number of tiles
/// Tab - Change the current tile textures
use bevy::{
    ecs::prelude::*,
    input::Input,
    math::IVec2,
    prelude::*,
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    utils::HashMap,
    DefaultPlugins,
};

use bevy_tiled_camera::{TiledCamera, TiledCameraBundle, TiledCameraPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0, 68, 153)))
        .add_plugins((
            TiledCameraPlugin,
            DefaultPlugins.set(ImagePlugin::default_nearest()),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, spawn_sprites))
        .run();
}

#[derive(Resource)]
struct SpriteTextures {
    pub tex_4x8: Handle<Image>,
    pub tex_8x8: Handle<Image>,
    pub tex_16x16: Handle<Image>,
    pub tex_32x32: Handle<Image>,
    pub current: u32,
}

#[derive(Component)]
struct GridSprite;

#[derive(Component)]
struct Cursor;

#[derive(Resource)]
struct GridEntities(HashMap<IVec2, Entity>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_count = [3, 3];
    let cam_bundle = TiledCameraBundle::new()
        .with_pixels_per_tile([4, 8])
        .with_tile_count(tile_count);

    commands.spawn(cam_bundle);

    let textures = SpriteTextures {
        tex_4x8: asset_server.load("4x8.png"),
        tex_8x8: asset_server.load("8x8.png"),
        tex_16x16: asset_server.load("16x16.png"),
        tex_32x32: asset_server.load("32x32.png"),
        current: 0,
    };

    commands.insert_resource(textures);

    let grid = GridEntities(HashMap::default());
    commands.insert_resource(grid);
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut q_cam: Query<&mut TiledCamera>,
    mut sprite_textures: ResMut<SpriteTextures>,
) {
    if input.just_pressed(KeyCode::Up) {
        q_cam.single_mut().tile_count.y += 1;
    }

    if input.just_pressed(KeyCode::Down) {
        let count = &mut q_cam.single_mut().tile_count;
        count.y = count.y.saturating_sub(1).max(1);
    }

    if input.just_pressed(KeyCode::Left) {
        let count = &mut q_cam.single_mut().tile_count;
        count.x = count.x.saturating_sub(1).max(1);
    }

    if input.just_pressed(KeyCode::Right) {
        q_cam.single_mut().tile_count.x += 1;
    }

    if input.just_pressed(KeyCode::Tab) {
        sprite_textures.current = (sprite_textures.current + 1) % 4;
        q_cam.single_mut().pixels_per_tile = match sprite_textures.current {
            1 => [16, 16].into(),
            2 => [32, 32].into(),
            3 => [8, 8].into(),
            _ => [4, 8].into(),
        };
    }

    if input.just_pressed(KeyCode::W) {
        let cam = q_cam.single_mut();
        let space = cam.world_space();
        q_cam.single_mut().set_world_space(space.other());
    }
}

fn spawn_sprites(
    mut commands: Commands,
    mut grid: ResMut<GridEntities>,
    q_camera_changed: Query<&TiledCamera, Changed<TiledCamera>>,
    q_camera: Query<(&GlobalTransform, &TiledCamera)>,
    sprites_query: Query<Entity, (With<Sprite>, With<GridSprite>)>,
    sprite_textures: Res<SpriteTextures>,
) {
    if !q_camera_changed.is_empty() {
        for entity in sprites_query.iter() {
            commands.entity(entity).despawn();
        }

        let (cam_transform, cam) = q_camera.single();
        grid.0.clear();

        for p in cam.tile_center_iter(cam_transform) {
            let sprite = Sprite {
                custom_size: cam.unit_size(),
                ..Default::default()
            };
            let texture = match sprite_textures.current {
                1 => sprite_textures.tex_16x16.clone(),
                2 => sprite_textures.tex_32x32.clone(),
                3 => sprite_textures.tex_8x8.clone(),
                _ => sprite_textures.tex_4x8.clone(),
            };
            let bundle = SpriteBundle {
                sprite,
                texture,
                transform: Transform::from_translation(p.extend(0.0)),
                ..Default::default()
            };
            commands.spawn((bundle, GridSprite));
        }
    }
}
