use bevy::{prelude::*, text::Text2dSize, render::camera::{ScalingMode, WindowOrigin}, sprite::Anchor};
use bevy_tiled_camera::*;

#[derive(Default)]
pub struct CameraState {
    zoom: f32,
}

fn main() {
    App::new()
    .add_plugin(TiledCameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_startup_system(text)
    .run();
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    // commands.spawn_bundle(Camera2dBundle {
    //     projection: OrthographicProjection {
    //         scaling_mode: ScalingMode::None,
    //         left: 0.0,
    //         right: 4.0,
    //         top: 8.0,
    //         bottom: 0.0,
    //         ..default()
    //     },
    //     ..default()
    // });

    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(1.0),
            window_origin: WindowOrigin::BottomLeft,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: server.load("4x8.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            custom_size: Some(Vec2::new(0.5, 1.0)),
            ..default()
        },
        ..default()
    });
}

fn input(
    key: Res<Input<KeyCode>>,
    mut state: ResMut<CameraState>,
) {
    if key.just_pressed(KeyCode::W) {
        state.zoom += 0.05;
    }
}

fn text(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    let style = TextStyle {
        font: server.load("RobotoMono-Regular.ttf"),
        font_size: 26.0,
        ..default()
    };
    let text = Text2dBundle {
        text: Text {
            sections: [TextSection {
                value: "Hello hello".to_string(),
                style: style,
            }].to_vec(),
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
            ..default()
        },
        ..default()
    };

    commands.spawn_bundle(text);
}