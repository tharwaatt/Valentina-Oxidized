mod types;
mod geometry;

use types::{GOType, DrawMode};
use geometry::Point2D;

fn main() {
    let origin = Point2D::new(0.0, 0.0);
    let my_point = Point2D::new(10.0, 0.0);
    
    // تجربة الدوران 90 درجة
    let rotated = my_point.rotate(&origin, 90.0);
    
    println!("Point rotated: {:?}", rotated);
    println!("Type: {:?}", GOType::Point);
}