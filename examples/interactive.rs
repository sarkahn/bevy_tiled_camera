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
    sprite::{Sprite, SpriteBundle},
    utils::HashMap,
    DefaultPlugins,
};

use bevy_tiled_camera::{TiledCameraBundle, TiledCameraPlugin, TiledProjection};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledCameraPlugin)
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(spawn_sprites)
        .add_system(cursor_system)
        .add_system(update_text)
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

struct GridEntities(HashMap<IVec2, Entity>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_count = [2, 2];
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

    let grid = GridEntities(HashMap::default());
    commands.insert_resource(grid);

    let col = Color::rgba(1.0, 1.0, 1.0, 0.35);
    let cursor = SpriteBundle {
        sprite: Sprite {
            color: col,
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..Default::default()
    };
    commands.spawn_bundle(cursor).insert(Cursor);

    make_ui(&mut commands, asset_server);
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut q_cam: Query<&mut TiledProjection>,
    mut q_tile_count: Query<&mut TileCount>,
    mut sprite_textures: ResMut<SpriteTextures>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut cam = q_cam.single_mut();
        let centered = cam.centered();
        cam.set_centered(!centered);
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
    mut grid: ResMut<GridEntities>,
    q_sprite_count_changed: Query<&TileCount, Changed<TileCount>>,
    q_camera_changed: Query<&TiledProjection, Changed<TiledProjection>>,
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

        let (cam_transform, proj) = q_camera.single();
        grid.0.clear();

        for p in proj.tile_center_iter(cam_transform) {
            let sprite = Sprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            };
            let texture = match sprite_textures.current {
                1 => sprite_textures.tex_16x16.clone(),
                2 => sprite_textures.tex_32x32.clone(),
                _ => sprite_textures.tex_8x8.clone(),
            };
            let bundle = SpriteBundle {
                sprite,
                texture,
                transform: Transform::from_translation(p),
                ..Default::default()
            };
            let entity = commands.spawn_bundle(bundle).insert(GridSprite).id();

            let tile_index = proj.world_to_tile(&cam_transform, p).unwrap();
            grid.0.insert(tile_index, entity);
        }
    }
}

fn make_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let font_size = 23.0;
    let font = asset_server.load("RobotoMono-Regular.ttf");
    let color = Color::YELLOW;
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text {
            sections: vec![
                TextSection {
                    value: "Controls:\n  -Resize the window to see auto-scaling.".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                TextSection {
                    value: "\n  -Spacebar to toggle camera center.\n  -Arrow keys to adjust number of tiles.\n  -Tab to change textures.".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                // Tile count/ppu
                TextSection {
                    value: String::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                // Window resolution
                TextSection {
                    value: String::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                // Camera zoom
                TextSection {
                    value: String::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                // Centered
                TextSection {
                    value: String::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                // Cursor world pos
                TextSection {
                    value: String::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                },
                // Cursor tile pos
                TextSection {
                    value: String::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                }
            ],
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_text(
    mut q_text: Query<&mut Text>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, &TiledProjection)>,
) {
    let mut text = q_text.single_mut();

    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        for (cam, t, proj) in q_camera.iter() {
            if let Some(cursor_world) = proj.screen_to_world(cam, &windows, t, pos) {
                let zoom = proj.zoom();
                let tile_count = proj.tile_count();
                let ppu = proj.pixels_per_tile();
                let target_res = tile_count * ppu;
                let cursor_x = format!("{:.2}", cursor_world.x);
                let cursor_y = format!("{:.2}", cursor_world.y);
                let window_res = Vec2::new(window.width(), window.height()).as_uvec2();
                let centered = proj.centered();

                text.sections[2].value = format!(
                    "\nProjection tiles: {}. Pixels Per Tile: {}",
                    tile_count, ppu
                );

                text.sections[3].value = format!(
                    "\nTarget resolution: {}.\nWindow resolution: {}. ",
                    target_res, window_res
                );

                text.sections[4].value = format!("\nProjection zoom: {}", zoom);

                text.sections[5].value = format!("\nProjection centered: {}", centered);

                text.sections[6].value = format!("\nCursor world pos: [{},{}]", cursor_x, cursor_y);

                if let Some(tile_pos) = proj.world_to_tile(t, cursor_world) {
                    text.sections[7].value = format!("\nCursor tile pos: {}", tile_pos);
                } else {
                    text.sections[7].value = String::default();
                }
            }
        }
    }
}

#[derive(Component)]
struct Cursor;

fn cursor_system(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, &TiledProjection)>,
    mut q_cursor: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
    mut q_sprite: Query<&mut Sprite>,
    grid: Res<GridEntities>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        for (cam, cam_transform, proj) in q_camera.iter() {
            if let Some(p) = proj.screen_to_world(cam, &windows, cam_transform, pos) {
                if let Some(mut p) = proj.world_to_tile_center(cam_transform, p) {
                    p.z = 2.0;

                    let (mut cursor_transform, mut v) = q_cursor.single_mut();
                    v.is_visible = true;

                    cursor_transform.translation = p;

                    if input.just_pressed(MouseButton::Left) {
                        let i = proj.world_to_tile(cam_transform, p).unwrap();
                        if let Some(entity) = grid.0.get(&i) {
                            if let Ok(mut sprite) = q_sprite.get_mut(entity.clone()) {
                                sprite.color = Color::rgb_u8(255, 0, 255);
                            }
                        }
                    }

                    return;
                }
            }
        }
    }

    let (_, mut v) = q_cursor.single_mut();
    v.is_visible = false;
}
