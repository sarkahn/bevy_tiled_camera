[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/bevy_tiled_camera)](https://crates.io/crates/bevy_tiled_camera)
[![docs](https://docs.rs/bevy_tiled_camera/badge.svg)](https://docs.rs/bevy_tiled_camera/)

# `Bevy Tiled Camera`

A simple camera for properly displaying low resolution pixel perfect 2D games in bevy.

---
![](images/demo.gif)

---

This camera will scale up the viewport as much as possible while mainting your target
resolution and avoiding pixel artifacts.

## Example
```rs
use bevy_tiled_camera::*;
use bevy::prelude::*;

fn setup(mut commands:Commands) {
  // Sets up a camera to display 80 x 25 tiles. The viewport will be scaled up
  // as much as possible to fit the window size and maintain the appearance of
  // 8 pixels per tile.
  let camera_bundle = TiledCameraBundle::new()
      .with_pixels_per_tile(8)
      .with_tile_count((80,25));

  commands.spawn_bundle(camera_bundle);
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(TiledCameraPlugin)
    .add_startup_system(setup.system())
    .run();
}
```

Note this is only half the work needed to avoid artifacts with low resolution pixel art.
You also need to ensure the camera position and your sprite edges are aligned to the 
pixel grid. 


You can change the camera settings at any time by adjusting the [TiledProjection](src/projection.rs) component on the camera entity.

## World Space
Note that this projection assumes the size of one tile is equal to one world unit. This is different than Bevy's default 2D orthographic camera which assumes one *pixel* is equal to one world unit.

## Versions
| bevy | bevy_tiled_camera |
| --- | --- |
| 0.5 | 0.2.4 |
| 0.5 | 0.2.3 |

*There is currently a bug in bevy version 0.5 where the camera projection
does not update unless you actively resize the window.*