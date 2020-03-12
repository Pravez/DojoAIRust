extern crate face_recognition;

use face_recognition::face_detection::FaceDetector;
use face_recognition::landmark_prediction::LandmarkPredictor;
use face_recognition::face_encoding::{FaceEncodings, FaceEncodingNetwork};
use opencv::core::{Mat, MatTrait};
use face_recognition::Rectangle;
use face_recognition::ImageMatrix;

pub struct Detector {
    face_detector: FaceDetector,
    face_encoder: FaceEncodingNetwork,
    landmarks_predictor: LandmarkPredictor
}

impl Detector {
    pub fn new() -> Self {
        Self {
            face_detector: FaceDetector::default(),
            face_encoder: FaceEncodingNetwork::default(),
            landmarks_predictor: LandmarkPredictor::default()
        }
    }

    pub fn cv_image_to_matrix(mat: &mut Mat) -> ImageMatrix {
        unsafe {
            ImageMatrix::new(mat.cols() as usize, mat.rows() as usize, mat.data_mut())
        }
    }

    pub fn get_vectorized_faces(&self, image: &mut Mat) -> (Vec<Rectangle>, Vec<FaceEncodings>) {
        let matrix = Detector::cv_image_to_matrix(image);

        let locations = self.face_detector.face_locations(&matrix);
        let rectangles = locations.iter().map(|location | location.clone()).collect::<Vec<Rectangle>>();
        let encodings = locations
            .iter()
            .map(|location |self.landmarks_predictor.face_landmarks(&matrix, location))
            .map(|landmarks |self.face_encoder.get_face_encodings(&matrix, &[landmarks], 0))
            .collect::<Vec<FaceEncodings>>();

        (rectangles, encodings)
    }
}