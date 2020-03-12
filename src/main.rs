extern crate opencv;

use opencv::core::*;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait, CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT};
use opencv::highgui::{named_window, imshow, wait_key};
use opencv::imgproc::{rectangle, LINE_4, cvt_color, COLOR_BGR2RGB, circle};
use crate::detector::Detector;

pub mod detector;

fn main() {
    let mut capture = VideoCapture::new(0, CAP_ANY).unwrap();
    named_window("test", 0);
    let mut image = Mat::default().unwrap();

    let color = Scalar_([255.0, 0.0, 0.0, 0.0]);
    let mut rgb_image = Mat::default().unwrap();

    let detector = Detector::new();


    loop {
        capture.read(&mut image);
        cvt_color(&image, &mut rgb_image, COLOR_BGR2RGB, 0);
        let (rectangles, encodings) = detector.get_vectorized_faces(&mut image);

        for r in rectangles.iter() {
            rectangle(&mut image, Rect2i { x: r.left as i32, y: r.top as i32, width: (r.right - r.left) as i32, height: (r.bottom - r.top) as i32 }, color, 1, LINE_4, 0);
        }


        imshow("test", &image);
        if wait_key(10).unwrap() == 27 {
            break;
        }
    }
    capture.release().unwrap()
}
