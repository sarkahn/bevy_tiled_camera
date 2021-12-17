use bevy::prelude::*;
use bevy_tiled_camera::*;

fn setup(
    mut commands: Commands
) {
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TiledCameraPlugin)
    .run();
}