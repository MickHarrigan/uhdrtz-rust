use std::collections::BTreeMap;

// This is for the handling of the physical camera and its implementation of the overall controlling within the other modules.
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::HashMap;
use flume::{bounded, unbounded};
use image::{ImageBuffer, Rgba};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::query;
use nokhwa::utils::{
    ApiBackend, CameraControl, CameraIndex, ControlValueSetter, KnownCameraControl, RequestedFormat,
};
use nokhwa::Camera;

use crate::gui::CameraControlEvent;

#[derive(Resource, Clone)]
pub struct VideoFrame(pub Handle<Image>);

#[derive(Component)]
pub struct VideoStream {
    pub image_rx: flume::Receiver<Image>,
    pub op_tx: flume::Sender<CameraSetting>,
    pub known_controls: BTreeMap<KnownCameraControl, CameraControl>,
    pub controls: BTreeMap<KnownCameraControl, ControlValueSetter>,
}

#[derive(Resource)]
pub struct ColorSettings {
    pub brightness: i8,
    pub contrast: u8,
    pub saturation: u8,
    pub gamma: u16,
    pub gain: u8,
    pub white_balance: u32,
    pub sharpness: u8,
    pub auto_exposure: bool,
    pub zoom: u16,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            brightness: 0,
            contrast: 15,
            saturation: 32,
            gamma: 220,
            gain: 0,
            white_balance: 5000,
            sharpness: 16,
            auto_exposure: true,
            zoom: 100,
        }
    }
}

pub struct CameraSetting {
    pub id: KnownCameraControl,
    pub control: ControlValueSetter,
}

impl VideoStream {
    pub fn new(index: CameraIndex, format: RequestedFormat) -> Result<Self> {
        // lots of this is *heavily* taken from https://github.com/foxzool/bevy_nokhwa/blob/main/src/camera.rs
        let (sender, receiver) = bounded(1);
        let (op_tx, op_rx) = unbounded::<CameraSetting>();

        let mut cam = Camera::new(index, format).unwrap();

        cam.open_stream().expect("Could not open the camera stream");
        let known_controls = cam.camera_controls_known_camera_controls().unwrap();

        std::thread::spawn(move || {
            #[allow(clippy::empty_loop)]
            loop {
                match op_rx.try_recv() {
                    Ok(op) => {
                        if let Err(why) = cam.set_camera_control(op.id, op.control) {
                            eprintln!("Couldn't set the control: {}", why);
                        }
                    }
                    // Err(why) => eprintln!("couldn't receive: {}", why),
                    Err(_why) => (),
                }
                let buffer = cam.frame().expect("Couldn't receive the camera frame");
                let mut buf = buffer.decode_image::<RgbAFormat>().unwrap();
                let wh = (2160, 2160);
                let image = Self::make_image(wh, &mut buf);
                let _ = sender.send(image);
            }
        });

        let known_controls: BTreeMap<KnownCameraControl, CameraControl> = known_controls
            .into_iter()
            .map(|(k, cont)| (k, cont))
            .collect();

        let controls = known_controls
            .iter()
            .map(|(k, cont)| {
                let value = cont.value();
                (*k, value)
            })
            .collect();
        Ok(Self {
            image_rx: receiver,
            op_tx,
            known_controls,
            controls,
        })
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

// helper function that sets a setting and prints the errors
pub fn send_camera_setting(cam: &VideoStream, id: KnownCameraControl, value: i64) {
    if let Err(why) = cam.op_tx.send(CameraSetting {
        id,
        control: ControlValueSetter::Integer(value),
    }) {
        eprintln!("{}", why);
    }
}

pub fn reset_camera_controls(mut color_settings: ResMut<ColorSettings>, cam: &VideoStream) {
    *color_settings = ColorSettings::default();

    send_camera_setting(
        cam,
        KnownCameraControl::Brightness,
        color_settings.brightness.into(),
    );

    send_camera_setting(
        cam,
        KnownCameraControl::Contrast,
        color_settings.contrast.into(),
    );
    send_camera_setting(
        cam,
        KnownCameraControl::Saturation,
        color_settings.saturation.into(),
    );
    send_camera_setting(cam, KnownCameraControl::Gamma, color_settings.gamma.into());
    send_camera_setting(
        cam,
        KnownCameraControl::Sharpness,
        color_settings.sharpness.into(),
    );
}
