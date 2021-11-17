# Bevy Tiled Camera

A simple camera for properly displaying low resolution pixel perfect 2D games in bevy. The camera will adjust the viewport to scale up your target resolution as much as possible without causing visual artifacts.

![](images/demo.gif)

Note that this projection assumes the size of one tile is equal to one world unit. This is different than Bevy's default 2D orthographic camera which assumes one *pixel* is equal to one world unit.

## Usage
You can use `TiledCameraBuilder` to easily set up your camera with your desired settings. This will create a camera entity with all the required components for rendering. You just have to specify your desired pixels per tile and tile count, then the viewport will be adjusted any time the window is resized. 

```rs
let camera_bundle = TiledCameraBuilder::new()
  .with_pixels_per_tile(8)
  .with_tile_count((80,25).into())
  .camera_bundle;

commands.spawn(camera_bundle);
```

You can change the camera settings at any time by adjusting the `TiledProjection` component on the camera entity.

## Bevy Version

This crate is currently built on the pipelined-rendering branch of bevy.