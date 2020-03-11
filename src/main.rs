extern crate face_recognition;
extern crate image;
extern crate opencv;

use face_recognition::*;
use face_recognition::face_detection::*;
use face_recognition::landmark_prediction::*;
use std::time::Instant;

use image::*;

use opencv::core::*;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait, CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT};
use opencv::highgui::{named_window, imshow, wait_key};
use std::convert::TryFrom;
use opencv::imgproc::{rectangle, LINE_4, cvt_color, COLOR_BGR2RGB, circle};
use std::any::Any;
use std::fmt::Debug;
use std::io::stdout;
use face_recognition::face_encoding::{FaceEncoding, FaceEncodingNetwork, FaceEncodings};
use opencv::sys::cv_getRotationMatrix2D__Point2f_double_double;

fn mat_to_image_matrix(mat: &mut Mat) -> ImageMatrix {
    unsafe {
        ImageMatrix::new(mat.cols() as usize, mat.rows() as usize, mat.data_mut())
    }
}

fn main() {
    let mut capture = VideoCapture::new(0, CAP_ANY).unwrap();
    named_window("test", 0);
    let mut image = Mat::default().unwrap();
    let detector = FaceDetector::default();
    let mut matrix;
    let color = Scalar_([255.0, 0.0, 0.0, 0.0]);
    let mut face_locations;
    let mut rgb_image = unsafe { Mat::new_rows_cols(capture.get(CAP_PROP_FRAME_HEIGHT).unwrap() as i32, capture.get(CAP_PROP_FRAME_WIDTH).unwrap() as i32, CV_8UC3) }.unwrap();

    let recognizer_nn = FaceEncodingNetwork::default();
    let landmarks_predictor = LandmarkPredictor::default();

    let mut landmarks;
    //let mut faces;

    loop {
        capture.read(&mut image);
        cvt_color(&image, &mut rgb_image, COLOR_BGR2RGB, 0);
        matrix = mat_to_image_matrix(&mut rgb_image);

        face_locations = detector.face_locations(&matrix);
        landmarks = face_locations.iter().map(|r | landmarks_predictor.face_landmarks(&matrix, r)).collect::<Vec<FaceLandmarks>>();

        for l in landmarks.iter() {
            for p in l.iter() {
                circle(&mut image, Point2i {x: p.x as i32, y: p.y as i32}, 1, color, 1, LINE_4, 0);
            }
        }

        //faces = landmarks.iter().map(|l | recognizer_nn.get_face_encodings(&matrix, &[l], 0)).collect::<Vec<FaceEncodings>>();

        for r in face_locations.iter() {
            rectangle(&mut image, Rect2i { x: r.left as i32, y: r.top as i32, width: (r.right - r.left) as i32, height: (r.bottom - r.top) as i32 }, color, 1, LINE_4, 0);
        }


        imshow("test", &image);
        if wait_key(10).unwrap() == 27 {
            break;
        }
    }
    capture.release().unwrap()
}
