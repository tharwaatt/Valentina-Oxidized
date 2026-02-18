use crate::types::{GOType, DrawMode};
use crate::geometry::Point2D;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SelectedItem {
    None,
    Point(u32),
    Line(u32),
    Spline(u32),
    Bisector(u32),
    Arc(u32),
    Contour(u32),
}

/// يمثل نوع ومعرف أي كيان هندسي يمكن أن يكون جزءاً من مسار
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntityRef {
    Line(u32),
    Spline(u32),
    Bisector(u32),
    Arc(u32),
}

/// البيانات المشتركة لكل كائنات Valentina
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VGObject {
    pub id: u32,
    pub name: String,
    pub obj_type: GOType,
    pub mode: DrawMode,
}

impl VGObject {
    pub fn new(id: u32, name: &str, obj_type: GOType) -> Self {
        Self {
            id,
            name: name.to_string(),
            obj_type,
            mode: DrawMode::Modeling,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPoint {
    pub metadata: VGObject,
    pub coords: Point2D,
}

impl VPoint {
    pub fn new(id: u32, name: &str, x: f64, y: f64) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Point),
            coords: Point2D::new(x, y),
        }
    }
    pub fn x(&self) -> f64 { self.coords.x }
    pub fn y(&self) -> f64 { self.coords.y }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VLine {
    pub metadata: VGObject,
    pub start_point_id: u32,
    pub end_point_id: u32,
}

impl VLine {
    pub fn new(id: u32, name: &str, start_id: u32, end_id: u32) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Line),
            start_point_id: start_id,
            end_point_id: end_id,
        }
    }
    pub fn length(&self, start_p: &VPoint, end_p: &VPoint) -> f64 {
        start_p.coords.distance_to(&end_p.coords)
    }
    pub fn angle(&self, start_p: &VPoint, end_p: &VPoint) -> f64 {
        start_p.coords.angle_to(&end_p.coords)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VCubicBezier {
    pub metadata: VGObject,
    pub p1_id: u32, 
    pub p2_id: u32, 
    pub p3_id: u32, 
    pub p4_id: u32, 
}

impl VCubicBezier {
    pub fn new(id: u32, name: &str, p1: u32, p2: u32, p3: u32, p4: u32) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Spline),
            p1_id: p1,
            p2_id: p2,
            p3_id: p3,
            p4_id: p4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VBisector {
    pub metadata: VGObject,
    pub p1_id: u32,
    pub vertex_id: u32,
    pub p3_id: u32,
    pub length: f64,
}

impl VBisector {
    pub fn new(id: u32, name: &str, p1: u32, vertex: u32, p3: u32, length: f64) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Line),
            p1_id: p1,
            vertex_id: vertex,
            p3_id: p3,
            length,
        }
    }
    pub fn calculate_end_point(&self, p1: &VPoint, vertex: &VPoint, p3: &VPoint) -> Point2D {
        let ang1 = vertex.coords.angle_to(&p1.coords);
        let ang2 = vertex.coords.angle_to(&p3.coords);
        let mut diff = ang2 - ang1;
        while diff < 0.0 { diff += 360.0; }
        while diff >= 360.0 { diff -= 360.0; }
        let bisector_angle = if diff > 180.0 {
            let actual_angle = 360.0 - diff;
            ang1 - actual_angle / 2.0
        } else {
            ang1 + diff / 2.0
        };
        vertex.coords.point_at(self.length, bisector_angle)
    }
}

/// كونتور (مسار) يجمع عدة خطوط ومنحنيات ليشكل قطعة واحدة
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VContour {
    pub metadata: VGObject,
    pub entities: Vec<EntityRef>,
}

impl VContour {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Spline), // نوع افتراضي للمسارات المعقدة
            entities: Vec::new(),
        }
    }
}

/// قوس دائري - مرتبط بنقطة المركز بالمعرف
/// VArc: Circular Arc referencing a center point by ID
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VArc {
    pub metadata: VGObject,
    pub center_id: u32,        // معرف نقطة المركز
    pub radius: f64,           // نصف القطر
    pub start_angle: f64,      // زاوية البداية بالدرجات
    pub end_angle: f64,        // زاوية النهاية بالدرجات
}

impl VArc {
    pub fn new(id: u32, name: &str, center_id: u32, radius: f64, start_angle: f64, end_angle: f64) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Arc),
            center_id,
            radius,
            start_angle,
            end_angle,
        }
    }

    /// حساب نقطة البداية على القوس
    /// Get start point on arc based on start_angle
    pub fn get_start_point(&self, center: &VPoint) -> Point2D {
        center.coords.point_at(self.radius, self.start_angle)
    }

    /// حساب نقطة النهاية على القوس
    /// Get end point on arc based on end_angle
    pub fn get_end_point(&self, center: &VPoint) -> Point2D {
        center.coords.point_at(self.radius, self.end_angle)
    }

    /// حساب زاوية القوس (الفرق بين البداية والنهاية)
    /// مع الحفاظ على الاتجاه (موجب = عكس عقارب الساعة، سالب = مع عقارب الساعة)
    /// Calculate arc angle span, preserving direction
    pub fn arc_angle(&self) -> f64 {
        let mut angle = self.end_angle - self.start_angle;
        // تطبيع الزاوية لتكون في النطاق (-180°, 180°]
        while angle > 180.0 { angle -= 360.0; }
        while angle <= -180.0 { angle += 360.0; }
        angle
    }

    /// طول القوس (موجب دائماً)
    /// Arc length (always positive)
    pub fn length(&self) -> f64 {
        self.radius * self.arc_angle().abs().to_radians()
    }

    /// توليد مسار SVG للأقواس باستخدام أمر A
    /// Generate SVG path data using Arc command
    /// SVG Arc: A rx ry x-axis-rotation large-arc-flag sweep-flag x y
    pub fn to_svg_path(&self, center: &VPoint) -> String {
        let start = self.get_start_point(center);
        let end = self.get_end_point(center);
        
        // BUG FIX: Use the raw angle difference to determine flags, not the normalized one.
        let angle_span = self.end_angle - self.start_angle;
        
        // large-arc-flag: 1 if the absolute span > 180°
        let large_arc_flag = if angle_span.abs() > 180.0 { 1 } else { 0 };
        
        // sweep-flag: 1 for counter-clockwise (positive angle span), 0 for clockwise (negative)
        let sweep_flag = if angle_span >= 0.0 { 1 } else { 0 };
        
        format!(
            "M {} {} A {} {} 0 {} {} {} {}",
            start.x, start.y,
            self.radius, self.radius,
            large_arc_flag, sweep_flag,
            end.x, end.y
        )
    }
}
