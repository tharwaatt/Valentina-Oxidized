use crate::types::{GOType, DrawMode};
use crate::geometry::Point2D;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug)]
pub enum SelectedItem {
    None,
    Point(u32),
    Line(u32),
    Spline(u32),
}

/// البيانات المشتركة لكل كائنات Valentina
/// بدلاً من الوراثة (Inheritance)، هنستخدم التركيب (Composition)
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
            mode: DrawMode::Modeling, // الوضع الافتراضي
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPoint {
    pub metadata: VGObject, // الهوية
    pub coords: Point2D,    // الحسابات
}

impl VPoint {
    pub fn new(id: u32, name: &str, x: f64, y: f64) -> Self {
        Self {
            metadata: VGObject::new(id, name, crate::types::GOType::Point),
            coords: Point2D::new(x, y),
        }
    }
    // دالة لتسهيل الوصول للحسابات
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

    /// حساب طول الخط بناءً على إحداثيات النقاط المرتبطة به.
    /// ملاحظة: بما أن الخط يخزن المعرفات فقط، يجب تمرير مراجع للنقاط من الخارج.
    pub fn length(&self, start_p: &VPoint, end_p: &VPoint) -> f64 {
        start_p.coords.distance_to(&end_p.coords)
    }

    /// حساب زاوية الخط بالدرجات (Degrees).
    pub fn angle(&self, start_p: &VPoint, end_p: &VPoint) -> f64 {
        start_p.coords.angle_to(&end_p.coords)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VCubicBezier {
    pub metadata: VGObject,
    pub p1_id: u32, // Start
    pub p2_id: u32, // Control 1
    pub p3_id: u32, // Control 2
    pub p4_id: u32, // End
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

    /// حساب إحداثيات النقطة النهائية للمنصف
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
