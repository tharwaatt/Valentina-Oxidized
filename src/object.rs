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


#[derive(Debug, Clone)]


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


        let dx = end_p.x() - start_p.x();


        let dy = end_p.y() - start_p.y();


        (dx * dx + dy * dy).sqrt()


    }





        /// حساب زاوية الخط بالدرجات (Degrees).





        pub fn angle(&self, start_p: &VPoint, end_p: &VPoint) -> f64 {





            let dx = end_p.x() - start_p.x();





            let dy = end_p.y() - start_p.y();





            dy.atan2(dx).to_degrees()





        }





    }





    

