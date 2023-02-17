# uhdrtz-rust
Rust implementation of the UHDRTZ

## Tooling used
The primary method for recreating and porting the UHDRTZ and its functionality are through the [bevy](https://github.com/bevyengine/bevy) ecosystem for the viewing and manipulation of the images.
In addition to this the use of the [nokhwa](https://github.com/l1npengtul/nokhwa) library for capture of the images shall be used.


## Things to do
- Rotate camera based on keyboard input
- Plugin for reading input from the bluetooth part to rotate based on that value
- Setup a GUI for the user to tweak and adjust the values of the camera inputs (bevy egui and other controls)
- Use bevy_audio to play music at the same time

### Nokhwa camera controls:
- brightness
- contrast
- saturation
- sharpness
- gamma
- white balance
- gain

6 others:
- exposure time
- zoom
- pan
- tilt
- auto exposure
- white balance, automatic (boolean)

## Things Completed
- Get video from the nokhwa package
- Convert said video frames to bevy texture
- Create a function to rotate the camera (or the plane that the texture is located)
  - Rotate automatically

