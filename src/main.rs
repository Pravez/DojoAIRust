extern crate opencv;

use opencv::core::*;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait, CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT};
use opencv::highgui::{named_window, imshow, wait_key};
use opencv::imgproc::{rectangle, LINE_4, cvt_color, COLOR_BGR2RGB, circle, put_text, FONT_HERSHEY_COMPLEX_SMALL};
use crate::detector::Detector;
use face_recognition::face_encoding::{FaceEncodings, FaceEncoding};
use std::fs;
use opencv::imgcodecs::{imread, ImreadModes, IMREAD_ANYCOLOR};

pub mod detector;
pub mod detection_rectangle;

fn load_database(path: &str, detector: &Detector) -> (Vec<String>, Vec<FaceEncoding>) {
    let mut names = Vec::default();
    let mut faces = Vec::default();

    for _file in fs::read_dir(path).unwrap() {
        let file = _file.unwrap();
        let name = String::from(file.file_name().to_str().unwrap());
        let image = imread(file.path().to_str().unwrap(), IMREAD_ANYCOLOR);
        let face = detector.get_vectorized_faces(&image.unwrap());

        if face.1.len() > 0 {
            names.push(name);
            faces.push(face.1[0].get(0).unwrap().clone());
        } else {
            println!("No face found for file {}", file.path().to_str().unwrap());
        }
    }

    (names, faces)
}

fn main() {
    let mut capture = VideoCapture::new(0, CAP_ANY).unwrap();
    named_window("test", 0);
    let mut image = Mat::default().unwrap();

    let color = Scalar_([255.0, 0.0, 0.0, 0.0]);
    let mut rgb_image = Mat::default().unwrap();

    let detector = Detector::new();
    let (names, faces) = load_database("./data", &detector);

    loop {
        capture.read(&mut image);
        cvt_color(&image, &mut rgb_image, COLOR_BGR2RGB, 0);
        let (rectangles, encodings) = detector.get_vectorized_faces(&image);

        for (n, r) in rectangles.iter().enumerate() {
            rectangle(&mut image, *r, color, 1, LINE_4, 0);
            for (i, name) in names.iter().enumerate() {
                let distance = faces[i].distance(encodings[n].get(0).unwrap());
                if distance < 0.6 {
                    put_text(&mut image, name.as_str(), Point2i { x: r.x, y: r.y }, FONT_HERSHEY_COMPLEX_SMALL, 2.0, color, 3, LINE_4, false);
                }
            }
        }

        imshow("test", &image);
        if wait_key(10).unwrap() == 27 {
            break;
        }
    }
    capture.release().unwrap()
}
