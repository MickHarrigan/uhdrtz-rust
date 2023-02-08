use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{
    CameraFormat, CameraIndex, FrameFormat, KnownCameraControl, RequestedFormat,
    RequestedFormatType, Resolution,
};
use nokhwa::{Buffer, Camera};

fn main() {
    println!("Hello World!");
    let mut cam = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(CameraFormat::new(
            Resolution::new(3840, 2160),
            FrameFormat::MJPEG,
            30,
        ))),
    )
    .unwrap();
    // this opens the camera to stream the images in
    cam.open_stream().unwrap();

    let frame = cam.frame().unwrap();
    println!("Captured an image of {}", frame.buffer().len());

    let buffer = Buffer::new(
        Resolution::new(3840, 2160),
        &frame.buffer(),
        FrameFormat::MJPEG,
    );
    let val = buffer
        .decode_image::<RgbFormat>()
        .unwrap()
        .save("image.png");

    if let Ok(_) = val {
        println!("Image was created, I think");
        // at this point in the testing this did take an image of me lol
    } else {
        println!("Image was not able to be created");
    }
}
