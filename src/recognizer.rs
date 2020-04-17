use crate::detector::Detector;
use face_recognition::face_encoding::FaceEncoding;
use std::fs;
use opencv::imgcodecs::{imread, IMREAD_ANYCOLOR};

pub struct Recognizer {
    pub faces: Vec<(FaceEncoding, String)>
}

fn load_database(path: &str, detector: &Detector) -> Vec<(FaceEncoding, String)> {
    let mut data = Vec::default();

    for _file in fs::read_dir(path).unwrap() {
        let file = _file.unwrap();
        let name = String::from(file.file_name().to_str().unwrap());

        if !name.contains("jpg") {
            continue
        }

        let image = imread(file.path().to_str().unwrap(), IMREAD_ANYCOLOR);
        let face = detector.get_vectorized_faces(&image.unwrap());

        if face.len() > 0 {
            data.push((face[0].0.get(0).unwrap().clone(), name));
        } else {
            println!("No face found for file {}", file.path().to_str().unwrap());
        }
    }

    data
}

impl Recognizer {
    pub fn new(database_path: &str, detector: &Detector) -> Self {
        let faces = load_database(database_path, detector);
        Self {
            faces
        }
    }
}