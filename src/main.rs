extern crate opencv;

use opencv::core::*;
use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
use opencv::highgui::{named_window, imshow, wait_key};
use opencv::imgproc::{rectangle, LINE_4, cvt_color, COLOR_BGR2RGB, put_text, FONT_HERSHEY_COMPLEX_SMALL};
use crate::detector::Detector;
use crate::recognizer::Recognizer;
use face_recognition::face_encoding::FaceEncoding;

pub mod detector;
pub mod detection_rectangle;
pub mod recognizer;

pub fn find_corresponding_face(recognizer: &Recognizer, face: &FaceEncoding) -> String {
    for (db_face, name) in recognizer.faces.iter() {
        // Pour chaque visage, on va le comparer à la base de visages (fonction `distance()` de FaceEncoding)
        // Si la distance est inférieure à un certain seuil, alors c'est qu'on est sur le bon visage !
    }

    return String::from("unknown");
}

fn main() {
    let mut capture = VideoCapture::new(0, CAP_ANY).unwrap();
    let window_name = "Dojo FaceRecognition::Rust";
    let database_path = "./data";

    let mut image = Mat::default().unwrap();
    let color = Scalar_([255.0, 0.0, 0.0, 0.0]);
    let mut rgb_image = Mat::default().unwrap();

    let detector = Detector::new();
    let recognizer = Recognizer::new(database_path, &detector);

    named_window(window_name, 0).expect("Unable to create a window");

    loop {
        // OpenCV : on lit l'image depuis le Stream Video (camera)
        capture.read(&mut image).expect("Unable to read video stream");
        // Premier piège ! OpenCV lit les images en BGR et notre réseau de neurones les lit en RGB !
        // utilisez la fonction `cvt_color()` pour changer le format de l'image.



        // Maintenant, récupérez à partir de l'image les visages vectorisés (fonction `get_vectorized_faces()`
        // De la structure Detector. Elle vous renvoie un vecteur de paires "Encodage du visage - Rectangle de positions dans l'image"



        // Il faut maintenant pour chaque visage que vous avez pu trouver, le comparer à la base d'images !
        // Utilisez pour cela la fonction `find_corresponding_face()` dans une boucle pour chercher la meilleure adéquation
        // par rapport à notre base de visages ! La fonction vous renvoie le nom de la personne, il ne vous
        // reste plus qu'à l'afficher.



        // Pour dessiner un carré dans OpenCV : `rectangle(image: &mut Image, rectangle: Rect2i, couleur: Scalar4, 1, LINE_4, 0)`
        // Pour écrire du texte : `put_text(image: &mut Image, text: &str, point: Point2i, FONT_HERSHEY_COMPLEX_SMALL, 2.0, couleur: Scalar4, 3, LINE_4, false)`

        imshow(window_name, &image).expect("Unable to show image");
        if wait_key(10).unwrap() == 27 {
            break;
        }
    }
    capture.release().unwrap()
}
