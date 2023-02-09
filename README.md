# uhdrtz-rust
Rust implementation of the UHDRTZ

## Tooling used
The primary method for recreating and porting the UHDRTZ and its functionality are through the [bevy](https://github.com/bevyengine/bevy) ecosystem for the viewing and manipulation of the images.
In addition to this the use of the [nokhwa](https://github.com/l1npengtul/nokhwa) library for capture of the images shall be used.


## Things to do
- Get video from the nokhwa package
- Convert said video frames to bevy texture (either through wgpu itself or some other methods)
- Create a function to rotate the camera (or the plane that the texture is located)
  - Rotate automatically
  - Rotate based on keyboard input
  - Plugin for reading input from the bluetooth part to rotate based on that value
- Setup a GUI for the user to tweak and adjust the values of the camera inputs (bevy egui and other controls)
- Use bevy_audio to play music at the same time