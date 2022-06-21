use bevy::prelude::*;
use bevy_tiled_camera::*;

fn main() {
    App::new()
    .add_plugin(TiledCameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(change_font_size)
    .run();
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    //commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(TiledCameraBundle::unit_cam([15,15], 26));

    let font_size = 26.0;
    let font = asset_server.load("RobotoMono-Regular.ttf");
    let color = Color::YELLOW;
    let alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text { 
            sections: [
                TextSection {
                    value: "Test".to_string(),
                    style: TextStyle { 
                        font, 
                        font_size, 
                        color, 
                    }
                },
            ].to_vec(),
            alignment, 
            ..default()
        },
        transform: Transform::from_xyz(-5.0, -5.0,0.0),
        ..default()
    });
}

fn change_font_size(
    input: Res<Input<KeyCode>>,
    mut q_text: Query<&mut Text>,
) {
    let mut diff = 0;

    if input.just_pressed(KeyCode::Up) {
        diff = 1;
    }
    if input.just_pressed(KeyCode::Down) {
        diff = -1;
    }

    if diff == 0 {
        return;
    }

    for mut t in q_text.iter_mut() {
        for mut section in &mut t.sections {
            let diff = diff as f32 * 0.25;
            section.style.font_size += diff;
            println!("Setting font size to {}", section.style.font_size);
        }
    }
}