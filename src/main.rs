extern crate opencv;

use opencv::core::*;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
use opencv::highgui::{named_window, imshow, wait_key};
use opencv::imgproc::{rectangle, LINE_4, cvt_color, COLOR_BGR2RGB, put_text, FONT_HERSHEY_COMPLEX_SMALL};
use crate::detector::Detector;
use crate::recognizer::Recognizer;

pub mod detector;
pub mod detection_rectangle;
pub mod recognizer;



fn main() {
    let mut capture = VideoCapture::new(0, CAP_ANY).unwrap();

    let mut image = Mat::default().unwrap();
    let color = Scalar_([255.0, 0.0, 0.0, 0.0]);
    let mut rgb_image = Mat::default().unwrap();

    let detector = Detector::new();
    let recognizer = Recognizer::new("./data", &detector);

    named_window("test", 0).expect("Unable to create a window");

    loop {
        capture.read(&mut image).expect("Unable to read video stream");
        cvt_color(&image, &mut rgb_image, COLOR_BGR2RGB, 0).expect("Unable to convert BGR to RGB");
        let face_encodings = detector.get_vectorized_faces(&image);

        for (face, r) in face_encodings {
            rectangle(&mut image, r, color, 1, LINE_4, 0).unwrap();
            let name = recognizer.find_corresponding_face(&face.get(0).unwrap());
            put_text(&mut image, name.as_str(), Point2i { x: r.x, y: r.y }, FONT_HERSHEY_COMPLEX_SMALL, 2.0, color, 3, LINE_4, false).unwrap();
        }

        imshow("test", &image).expect("Unable to show image");
        if wait_key(10).unwrap() == 27 {
            break;
        }
    }
    capture.release().unwrap()
}
