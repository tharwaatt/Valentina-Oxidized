mod types;
mod geometry;
mod object; // الملف الجديد

use object::VPoint;

fn main() {
    // إنشاء أول نقطة فالنتينا حقيقية
    let p1 = VPoint::new(1, "A", 100.0, 50.0);

    println!("--- Valentina Object Created ---");
    println!("Name: {}", p1.metadata.name);
    println!("ID: {}", p1.metadata.id);
    println!("Coordinates: ({}, {})", p1.x(), p1.y());

    // تخيل إننا بنلفف النقطة دي
    let origin = geometry::Point2D::new(0.0, 0.0);
    let rotated_coords = p1.coords.rotate(&origin, 90.0);
    
    println!("After 90 deg rotation: ({:.2}, {:.2})", rotated_coords.x, rotated_coords.y);
}