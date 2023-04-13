// This is for the handling of the physical camera and its implementation of the overall controlling within the other modules.
use anyhow::Result;
use bevy::asset::Handle;
use bevy::ecs::{component::Component, system::Resource};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::Image;
use bevy::utils::HashMap;
use flume::bounded;
use image::{ImageBuffer, Rgba};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::query;
use nokhwa::utils::{ApiBackend, CameraIndex, RequestedFormat};
use nokhwa::CallbackCamera;

#[derive(Resource, Clone)]
pub struct VideoFrame(pub Handle<Image>);

#[derive(Component)]
pub struct VideoStream {
    pub image_rx: flume::Receiver<Image>,
}

impl VideoStream {
    pub fn new(index: CameraIndex, format: RequestedFormat) -> Result<Self> {
        // lots of this is *heavily* taken from https://github.com/foxzool/bevy_nokhwa/blob/main/src/camera.rs
        let (sender, receiver) = bounded(1);

        let callback_fn = move |buffer: nokhwa::Buffer| {
            let mut buf = buffer.decode_image::<RgbAFormat>().unwrap();
            let wh = (2160, 2160);
            let image = Self::make_image(wh, &mut buf);
            let _ = sender.send(image);
        };

        let mut threaded_camera = CallbackCamera::new(index, format, callback_fn)
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

    #[inline]
    fn make_image(wh: (u32, u32), buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Image {
        Image::new(
            Extent3d {
                width: wh.0,
                height: wh.1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            image::imageops::crop(buffer, 360, 0, 2160, 2160)
                .to_image()
                .to_vec(),
            TextureFormat::Rgba8UnormSrgb,
        )
    }
}

impl Drop for VideoStream {
    fn drop(&mut self) {
        // this should soon be updated to hopefully try and remove the lock poisoning
        // as I believe that the incorrect dropping of the VideoStream "object" is at play here.
        println!("VideoStream Dropped!");
    }
}

pub fn hash_available_cameras(// mut cams: ResMut<CaptureDevices>,
    // mut selected: ResMut<SelectedCamera>,
) -> (Option<(String, u32)>, HashMap<String, u32>) {
    // this is where the query for cameras should occur and then filter out any repeats
    let cameras = query(ApiBackend::Auto).unwrap();
    let mut hash: HashMap<String, _> = HashMap::new();
    for camera in cameras.iter() {
        if hash.contains_key(&camera.human_name()) {
            // if the old value in the hash is greater than the new one
            // replace it with the smaller value
            if hash.get(&camera.human_name()).unwrap() > &camera.index().as_index().unwrap() {
                hash.insert(camera.human_name(), camera.index().as_index().unwrap());
            }
        } else {
            hash.insert(camera.human_name(), camera.index().as_index().unwrap());
        }
    }

    // this sets the default selected camera to that of the first thing obtained from the hash
    let selected: Option<(String, u32)>;
    if hash.len() > 0 {
        let (name, ind) = hash.iter().nth(0).unwrap();
        selected = Some((name.clone().to_string(), *ind));
    } else {
        selected = None;
    }
    // this sets the "list" of cameras to that of the hash (un-ordered list in effect)
    // cams.0 = hash;
    (selected, hash)
}
