[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/bevy_tiled_camera)](https://crates.io/crates/bevy_tiled_camera)
[![docs](https://docs.rs/bevy_tiled_camera/badge.svg)](https://docs.rs/bevy_tiled_camera/)

# `Bevy Tiled Camera`

A camera for properly displaying low resolution pixel perfect 2D games in 
bevy. It works by adjusting the viewport to match a target resolution, which
is defined by a tile count and the number of pixels per tile.

The camera will scale the viewport as much as possible while maintaining 
your target resolution and avoiding pixel artifacts.

**Note**: Due to how resources are initialized, `TiledCameraPlugin` *must* 
be added before `DefaultPlugins` during app initialization. This ensures the
default image filtering is set properly.
https://github.com/bevyengine/bevy/issues/1255

## Example
```rust
use bevy_tiled_camera::*;
use bevy::prelude::*;

fn setup(mut commands:Commands) {
  // Sets up a camera to display 80 x 35 tiles. The viewport will be scaled
  // up as much as possible to fit the window size and maintain the 
  // appearance of 8 pixels per tile.
  let camera_bundle = TiledCameraBundle::unit_cam()
      .with_pixels_per_tile([8,8])
      .with_tile_count([80,35]);
  // or
  let camera_bundle = TiledCameraBundle::unit_cam()
      .with_pixels_per_tile([8,8])
      .with_target_resolution([640,280]);

  commands.spawn_bundle(camera_bundle);
}

fn main() {
    App::new()
    .add_plugin(TiledCameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}
```

## Rendering Pixel Art
There's few important things to consider when displaying low resolution 
pixel art. Depending on how your camera is set up and what your image 
filtering is, you might end up with extreme rendering artifacts. Your 
pixels may deform in shape when moving, or you might see blinking lines 
between your tiles depending on the position and 'orthographic size' of your 
camera.

This camera attempts to alleviate some of that, but you must decide how your
world space will be defined, as it informs how all your sprites must
be sized and positioned in the game. Regardless of which you use, the 
camera will adjust the projection and viewport to match your target 
resolution.

### `WorldSpace::Units`
With this method you decide on a set number of "pixels per unit". This 
defines how large a single world unit (or "tile") is in pixels.

When your world space is defined by world units you can define your 
transforms and movement in your game in terms of world units, the same as 
you would in 3d.

This is the default for TiledCamera.

### `WorldSpace::Pixels`
One pixel == one world unit. If your world space is defined by pixels, all 
motion and positioning will also be defined in terms of pixels. The "pixels 
per tile" setting of the camera determines how many world units are 
considered a "tile" from the camera's perspective.

This is the default for bevy's built in orthographic camera.

## Customization

BevyTiledCamera supports either world space, but defaults to 
`WorldSpace::Units`. You can change this during construction:

```rust
use bevy_tiled_camera::*;
let cam = TiledCameraBundle::pixel_cam([5,5], [8,8]);
let cam = TiledCameraBundle::unit_cam([5,5], [8,8]);
// Or
let cam = TiledCameraBundle::new()
    .with_world_space(WorldSpace::Pixels);
```
## Versions
| bevy | bevy_tiled_camera |
| --- | --- |
| 0.8 | 0.4.0 |
| 0.6 | 0.3.0 |
| 0.5 | 0.2.4 |
| 0.5 | 0.2.3 |