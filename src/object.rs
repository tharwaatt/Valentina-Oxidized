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
    AlongLine(u32),
    Contour(u32),
}

/// يمثل نوع ومعرف أي كيان هندسي يمكن أن يكون جزءاً من مسار أو عملية
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntityRef {
    Point(u32),
    Line(u32),
    Spline(u32),
    Bisector(u32),
    Arc(u32),
    AlongLine(u32),
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
            metadata: VGObject::new(id, name, crate::types::GOType::Line), // تظهر كخط
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

/// قوس دائري - مرتبط بنقطة المركز بالمعرف
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VArc {
    pub metadata: VGObject,
    pub center_id: u32,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
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
    pub fn get_start_point(&self, center: &VPoint) -> Point2D {
        center.coords.point_at(self.radius, self.start_angle)
    }
    pub fn get_end_point(&self, center: &VPoint) -> Point2D {
        center.coords.point_at(self.radius, self.end_angle)
    }
    pub fn arc_angle(&self) -> f64 {
        let mut angle = self.end_angle - self.start_angle;
        while angle > 180.0 { angle -= 360.0; }
        while angle <= -180.0 { angle += 360.0; }
        angle
    }
    pub fn to_svg_path(&self, center: &VPoint) -> String {
        let start = self.get_start_point(center);
        let end = self.get_end_point(center);
        let angle_span = self.end_angle - self.start_angle;
        let large_arc_flag = if angle_span.abs() > 180.0 { 1 } else { 0 };
        let sweep_flag = if angle_span >= 0.0 { 1 } else { 0 };
        format!("M {} {} A {} {} 0 {} {} {} {}", start.x, start.y, self.radius, self.radius, large_arc_flag, sweep_flag, end.x, end.y)
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
            metadata: VGObject::new(id, name, crate::types::GOType::Spline),
            entities: Vec::new(),
        }
    }
}

/// أداة "نقطة على خط" (Point Along Line)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VPointAlongLine {
    pub metadata: VGObject,
    pub p1_id: u32, // بداية الخط
    pub p2_id: u32, // نهاية الخط (لتحديد الاتجاه)
    pub distance: f64, // المسافة من p1 باتجاه p2
}

impl VPointAlongLine {
    pub fn new(id: u32, name: &str, p1: u32, p2: u32, distance: f64) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Point),
            p1_id: p1,
            p2_id: p2,
            distance,
        }
    }

    /// حساب إحداثيات النقطة التابعة
    pub fn calculate_point(&self, p1: &VPoint, p2: &VPoint) -> Point2D {
        let angle = p1.coords.angle_to(&p2.coords);
        p1.coords.point_at(self.distance, angle)
    }
}

/// يمثل قيد الانعكاس (Mirror Constraint)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VMirror {
    pub metadata: VGObject,
    pub source_entities: Vec<EntityRef>,
    pub axis_p1_id: u32,
    pub axis_p2_id: u32,
}

impl VMirror {
    pub fn new(id: u32, name: &str, sources: Vec<EntityRef>, axis_p1: u32, axis_p2: u32) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Line),
            source_entities: sources,
            axis_p1_id: axis_p1,
            axis_p2_id: axis_p2,
        }
    }
}
