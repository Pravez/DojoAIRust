use face_recognition::Rectangle;
use opencv::core::Rect2i;

pub struct DetectionRectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl From<&Rectangle> for DetectionRectangle {
    fn from(r: &Rectangle) -> Self {
        Self {
            x: r.left as i32,
            y: r.top as i32,
            width: (r.right - r.left) as i32,
            height: (r.bottom - r.top) as i32
        }
    }
}

impl Into<Rect2i> for DetectionRectangle {
    fn into(self) -> Rect2i {
        Rect2i {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height
        }
    }
}
