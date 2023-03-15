# 0.4.0

Update to `bevy` `0.10`.
- Fix issues with stretching.
- Add `Fill::stretch`.

# 0.3.0

Update to `bevy` `0.9`.

# 0.2.0

- `PixelBuffer` is no longer a marker component, but holds the size and fill behaviour.
- Add `create_image` function.
- Add `PixelBufferBundle` and `PixelBufferSpriteBundle` as an alternative to the builder API.
- Add configurable sprite bundle to `RenderConfig`.
- If a pixel buffer has a `Sprite` component, it will resize when the pixel buffer size changes.
- `egui` feature is now disabled by default.
- `Fill::Window` now has a `WindowId`. This allows to fill a window that is not the primary window.
- Add `PixelBufferPlugins` plugin group.
- Rewrite `query` module.
  - Add `PixelBuffers` world query.
  - `QueryPixelBuffers` now works different but the API is almos the same.
- Removed `init_frame` from `PixelBufferCommands`.
- Add `GetFrame`, `GetFrameFromImages`, `GetFrameFromImageHandle` and `FrameEditExtension` traits that improve them ergonomics when getting a frame.
