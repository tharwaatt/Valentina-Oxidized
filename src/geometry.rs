use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// حساب المسافة بين نقطتين
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// حساب زاوية الخط المتجه من هذه النقطة إلى نقطة أخرى (بالدرجات)
    pub fn angle_to(&self, other: &Point2D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dy.atan2(dx).to_degrees()
    }

    /// حساب نقطة جديدة تبعد مسافة معينة وبزاوية معينة
    pub fn point_at(&self, distance: f64, angle_degrees: f64) -> Self {
        let rad = angle_degrees.to_radians();
        Self {
            x: self.x + distance * rad.cos(),
            y: self.y + distance * rad.sin(),
        }
    }

    /// حساب انعكاس النقطة حول خط معرف بنقطتين (axis_p1, axis_p2)
    pub fn mirror_over_line(&self, axis_p1: &Point2D, axis_p2: &Point2D) -> Self {
        let dx = axis_p2.x - axis_p1.x;
        let dy = axis_p2.y - axis_p1.y;
        
        let d = dx * dx + dy * dy;
        if d.abs() < f64::EPSILON { return *self; }

        let a = (dx * dx - dy * dy) / d;
        let b = 2.0 * dx * dy / d;

        let x = self.x - axis_p1.x;
        let y = self.y - axis_p1.y;

        Self {
            x: (a * x + b * y) + axis_p1.x,
            y: (b * x - a * y) + axis_p1.y,
        }
    }

    // تطبيق أول دالة رياضية شفناها في الـ CPP: الدوران
    pub fn rotate(&self, origin: &Point2D, degrees: f64) -> Self {
        let rad = degrees.to_radians();
        let (sin_a, cos_a) = rad.sin_cos();

        let dx = self.x - origin.x;
        let dy = self.y - origin.y;

        Point2D {
            x: origin.x + (dx * cos_a - dy * sin_a),
            y: origin.y + (dx * sin_a + dy * cos_a),
        }
    }
}
