use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{
    CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
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
    let buf = buffer.decode_image::<RgbFormat>().unwrap();
    let (wt, ht) = (buf.width(), buf.height());
    println!("Buffer size of {} x {}", wt, ht);
    buf.save("image.png").expect("Image was unable to be saved");
}
