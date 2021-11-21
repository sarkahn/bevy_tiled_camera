# Bevy Tiled Camera

A simple camera for properly displaying low resolution pixel perfect 2D games in bevy. The camera will adjust the viewport to scale up your target resolution as much as possible without causing visual artifacts.

---
![](images/demo.gif)

  *From the "interactive" example*

---

## Usage
Just spawn a `TiledCameraBundle` with your desired settings. You should specify your desired pixels per tile and tile count, then the viewport will be adjusted any time the window is resized. 

```rs
use bevy_tiled_camera::TiledCameraBundle;
use bevy::prelude::Commands;

fn setup(mut commands:Commands) {
  // Sets up a camera to display 80 x 25 tiles. The viewport will be scaled up
  // as much as possible to fit the window size and maintain the appearance of
  // 8 pixels per tile.
  let camera_bundle = TiledCameraBundle::new()
      .with_pixels_per_tile(8)
      .with_tile_count((80,25).into()
  );

  commands.spawn_bundle(camera_bundle);
}
```

You can change the camera settings at any time by adjusting the `TiledProjection` component on the camera entity.

## World Space
Note that this projection assumes the size of one tile is equal to one world unit. This is different than Bevy's default 2D orthographic camera which assumes one *pixel* is equal to one world unit.