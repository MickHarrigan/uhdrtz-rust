# uhdrtz-rust
Rust implementation of the UHDRTZ

## Tooling used
The primary method for recreating and porting the UHDRTZ and its functionality are through the [bevy][] ecosystem for the viewing and manipulation of the images.
In adiition to this is the use of the [opencv][] rust bindings for getting the image from the camera.

## Things to do
- Create a way to convert between opencv::prelude::Mat and a bevy image
- Create a component of the VideoCapture device used to get the images from
