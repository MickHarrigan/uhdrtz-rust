// This is for the handling of the physical camera and its implementation of the overall controlling within the other modules.
use anyhow::Result;
use bevy::asset::Handle;
use bevy::ecs::{component::Component, system::Resource};
use bevy::render::texture::Image;
use bevy::utils::HashMap;
use flume::unbounded;
use image::RgbaImage;
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::query;
use nokhwa::utils::{ApiBackend, CameraIndex, RequestedFormat};
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
        // this should soon be updated to hopefully try and remove the lock poisoning
        // as I believe that the incorrect dropping of the VideoStream "object" is at play here.
        println!("VideoStream Dropped!");
    }
}

pub fn hash_available_cameras(// mut cams: ResMut<CaptureDevices>,
    // mut selected: ResMut<SelectedCamera>,
) -> (Option<(String, u32)>, HashMap<String, u32>) {
    // TODO: make this a function that returns a CaptureDevices or the hash itself to save on some resource complexity

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
