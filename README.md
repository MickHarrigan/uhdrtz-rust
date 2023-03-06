# uhdrtz-rust
Rust implementation of the UHDRTZ

## Tooling used
The primary method for recreating and porting the UHDRTZ and its functionality are through the [bevy](https://github.com/bevyengine/bevy) ecosystem for the viewing and manipulation of the images.
In addition to this the use of the [nokhwa](https://github.com/l1npengtul/nokhwa) library for capture of the images shall be used.


## Things to do
- Camera control GUI
- Update the max interval resource to be setup in the gui
- Use bevy_audio to play music at the same time
- Break up the Zoetrope plugin into smaller subplugins
- QOL updates (select camera/Arduino?/ Other hardware) on startup
- Fix camera dropping issues
- slow down camera capture rate
- stability and speed updates

## Things Completed
- Get video from the nokhwa package
- Convert said video frames to bevy texture
- Create a function to rotate the camera (or the plane that the texture is located)
  - Rotate automatically
- Rotate camera based on keyboard input
- Read bluetooth data from Arduino and rotate based on that input
- Display images (masks) over the actual rotating image
- Add max interval and number of slices to gui
- GUI for controlling the location of the different images

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
