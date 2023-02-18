use anyhow::Result;
use bevy::asset::Handle;
use bevy::ecs::{component::Component, system::Resource};
use bevy::render::texture::Image;
use flume::unbounded;
use image::RgbaImage;
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat};
use nokhwa::CallbackCamera;

#[derive(Resource, Clone)]
pub struct VideoFrame(pub Handle<Image>);

#[derive(Component)]
pub struct VideoStream {
    pub image_rx: flume::Receiver<RgbaImage>,
}

impl VideoStream {
    pub fn new(index: u32, format: RequestedFormat) -> Result<Self> {
        // lots of this is *heavily* taken from https://github.com/foxzool/bevy_nokhwa/blob/main/src/camera.rs
        let (sender, receiver) = unbounded();

        let callback_fn = move |buffer: nokhwa::Buffer| {
            let image = buffer.decode_image::<RgbAFormat>().unwrap();
            let _ = sender.send(image);
        };

        let mut threaded_camera =
            CallbackCamera::new(CameraIndex::Index(index), format, callback_fn)
                .expect("Could not create a CallbackCamera");

        threaded_camera
            .open_stream()
            .expect("Could not open the camera stream");

        std::thread::spawn(move || {
            #[allow(clippy::empty_loop)]
            loop {
                threaded_camera
                    .last_frame()
                    .expect("Couldn't receive the latest frame");
            }
        });

        Ok(Self { image_rx: receiver })
    }
}

impl Drop for VideoStream {
    fn drop(&mut self) {
        println!("VideoStream Dropped!");
    }
}
