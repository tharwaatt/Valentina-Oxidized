use crate::types::{GOType, DrawMode};
use crate::geometry::Point2D;

/// البيانات المشتركة لكل كائنات Valentina
/// بدلاً من الوراثة (Inheritance)، هنستخدم التركيب (Composition)
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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