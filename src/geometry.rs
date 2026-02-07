#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // تطبيق أول دالة رياضية شفناها في الـ CPP: الدوران
    pub fn rotate(&self, origin: &Point2D, degrees: f64) -> Self {
        let rad = degrees.to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();

        let dx = self.x - origin.x;
        let dy = self.y - origin.y;

        Point2D {
            x: origin.x + (dx * cos_a - dy * sin_a),
            y: origin.y + (dx * sin_a + dy * cos_a),
        }
    }
}