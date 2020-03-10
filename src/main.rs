extern crate face_recognition;

use face_recognition::*;
use face_recognition::face_detection::*;
use face_recognition::landmark_prediction::*;
use std::time::Instant;

extern crate opencv;

use opencv::core::*;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
use opencv::highgui::{named_window, imshow, wait_key};
use std::convert::TryFrom;
use opencv::imgproc::{rectangle, LINE_4};

fn mat_to_image_matrix(mat: &mut Mat) -> ImageMatrix {
    unsafe {
        ImageMatrix::new(mat.rows() as usize, mat.cols() as usize, mat.data_mut())
    }
}

fn main() {
    let mut capture = VideoCapture::new(0, CAP_ANY).unwrap();
    named_window("test", 0);
    let mut image = Mat::default().unwrap();
    let detector = FaceDetector::default();
    let mut matrix;
    let color = Scalar_([255.0, 0.0,0.0,0.0]);
    let mut face_locations;

    loop {
        capture.read(&mut image);

        matrix = mat_to_image_matrix(&mut image);
        face_locations = detector.face_locations(&matrix);

        println!("{}", face_locations.len());

        for r in face_locations.iter() {
            println!("{} {} {} {}", r.top, r.bottom, r.left, r.right);
            rectangle(&mut image, Rect2i { x: r.left as i32, y: r.top as i32, width: (r.right - r.left) as i32, height: (r.bottom - r.top) as i32 }, color, 1, LINE_4, 0);
        }

        imshow("test", &image);
        if wait_key(10).unwrap() == 27 {
            break;
        }
    }
    capture.release().unwrap()
}
