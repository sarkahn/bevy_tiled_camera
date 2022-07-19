/// A simple interactive demo. Resize the window to see the viewport auto-adjust.
///
/// # Controls:
/// Spacebar - Toggle camera between centered or bottom-left origin
/// Arrow Keys - Adjust the number of tiles
/// Tab - Change the current tile textures
use bevy::{
    ecs::prelude::*,
    input::Input,
    math::{IVec2, Vec2},
    prelude::*,
    render::texture::{Image, ImageSampler, ImageSettings},
    sprite::{Sprite, SpriteBundle},
    utils::HashMap,
    DefaultPlugins,
};

use bevy_tiled_camera::{TiledCamera, TiledCameraBundle, TiledCameraPlugin};

fn main() {
    App::new()
        .add_plugin(TiledCameraPlugin)
        .insert_resource(ImageSettings {
            default_sampler: ImageSampler::nearest_descriptor(),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(spawn_sprites)
        //.add_system(update_text)
        .run();
}

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

struct GridEntities(HashMap<IVec2, Entity>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_count = [3, 3];
    let cam_bundle = TiledCameraBundle::new()
        .with_pixels_per_tile([4, 8])
        .with_tile_count(tile_count);

    commands.spawn_bundle(cam_bundle);

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

    //make_ui(&mut commands, asset_server);
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
            commands.spawn_bundle(bundle).insert(GridSprite);
        }

        // Blue Background to show viewport border
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 1.0, 0.015),
                custom_size: Some(Vec2::ONE * 99999999.0),
                ..default()
            },
            ..default()
        });
    }
}

// Need #4007 or #5114 for this to work
//  - https://github.com/bevyengine/bevy/pull/4007
//  - https://github.com/bevyengine/bevy/pull/5114
// fn make_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
//     let font_size = 26.0;
//     let font = asset_server.load("RobotoMono-Regular.ttf");
//     let color = Color::YELLOW;
//     let style = || {
//         TextStyle {
//             font: font.clone(),
//             font_size,
//             color,
//         }
//     };
//     let alignment = TextAlignment {
//         vertical: VerticalAlign::Top,
//         horizontal: HorizontalAlign::Left,
//     };

//     let layer = RenderLayers::layer(1);
//     commands.spawn_bundle(Camera2dBundle {
//         camera: Camera {
//             priority: -1,
//             ..default()
//         },
//         ..default()
//     }).insert(layer);

//     commands.spawn_bundle(Text2dBundle {
//         text: Text {
//             sections: vec![
//                 TextSection {
//                     value: "Controls:\n  -Resize the window to see auto-scaling.".to_string(),
//                     style: style(),
//                 },
//                 TextSection {
//                     value: "\n  -Arrow keys to adjust number of tiles.\n  -Tab to change sprites."
//                         .to_string(),
//                         style: style(),
//                 },
//                 // Tile count/ppu
//                 TextSection {
//                     value: String::default(),
//                     style: style(),
//                 },
//                 // Window resolution
//                 TextSection {
//                     value: String::default(),
//                     style: style(),
//                 },
//                 // Camera zoom
//                 TextSection {
//                     value: String::default(),
//                     style: style(),
//                 },
//                 // Cursor world pos
//                 TextSection {
//                     value: String::default(),
//                     style: style(),
//                 },
//                 // Cursor tile pos
//                 TextSection {
//                     value: String::default(),
//                     style: style(),
//                 },
//             ],
//             alignment,
//         },
//         ..default()
//     }).insert(layer);
// }

// fn update_text(
//     mut q_text: Query<&mut Text>,
//     windows: Res<Windows>,
//     q_camera: Query<(&Camera, &GlobalTransform, &TiledCamera)>,
// ) {
//     let mut text = q_text.single_mut();

//     if let Some(window) = windows.get_primary() {
//         if let Some(pos) = window.cursor_position() {
//             for (cam, t, tcam) in q_camera.iter() {
//                 if let Some(cursor_world) = tcam.screen_to_world(pos, &cam, &t) {
//                     let zoom = tcam.zoom();
//                     let tile_count = tcam.tile_count;
//                     let ppu = tcam.pixels_per_tile;
//                     let target_res = tcam.target_resolution();
//                     let cursor_x = format!("{:.2}", cursor_world.x);
//                     let cursor_y = format!("{:.2}", cursor_world.y);
//                     let window_res = Vec2::new(window.width(), window.height()).as_uvec2();

//                     text.sections[1].value = format!(
//                         "\nProjection tiles: {}. Pixels Per Tile: {}",
//                         tile_count, ppu
//                     );

//                     text.sections[2].value = format!(
//                         "\nTarget resolution: {}.\nWindow resolution: {}. ",
//                         target_res, window_res
//                     );

//                     text.sections[3].value = format!("\nProjection zoom: {}", zoom);

//                     text.sections[4].value =
//                         format!("\nCursor world pos: [{},{}]", cursor_x, cursor_y);

//                     text.sections[5].value =
//                         format!("\nCursor tile pos: {}", tcam.world_to_tile(t, cursor_world));
//                 }
//             }
//         }
//     }
// }

// fn cursor_system(
//     input: Res<Input<MouseButton>>,
//     windows: Res<Windows>,
//     q_camera: Query<(&Camera, &GlobalTransform, &TiledProjection)>,
//     mut q_cursor: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
//     mut q_sprite: Query<&mut Sprite>,
//     grid: Res<GridEntities>,
// ) {
//     let window = windows.get_primary().unwrap();

//     if let Some(pos) = window.cursor_position() {
//         for (cam, cam_transform, proj) in q_camera.iter() {
//             if let Some(p) = proj.screen_to_world(cam, &windows, cam_transform, pos) {
//                 if let Some(mut p) = proj.world_to_tile_center(cam_transform, p) {
//                     p.z = 2.0;

//                     let (mut cursor_transform, mut v) = q_cursor.single_mut();
//                     v.is_visible = true;

//                     cursor_transform.translation = p;

//                     if input.just_pressed(MouseButton::Left) {
//                         let i = proj.world_to_tile(cam_transform, p).unwrap();
//                         if let Some(entity) = grid.0.get(&i) {
//                             if let Ok(mut sprite) = q_sprite.get_mut(entity.clone()) {
//                                 sprite.color = Color::rgb_u8(255, 0, 255);
//                             }
//                         }
//                     }

//                     return;
//                 }
//             }
//         }
//     }

//     let (_, mut v) = q_cursor.single_mut();
//     v.is_visible = false;
// }v
