/// GOType: Geometric Object Type
/// ده اللي بيعرف الـ AI أو البرنامج إحنا بنرسم إيه
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GOType {
    Point,
    Line,
    Spline,
    Arc,
    Circle,
}

/// Draw Mode: هل الكائن ده للحسابات فقط أم للرسم النهائي؟
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Calculation,  // للحسابات فقط (مش هيظهر للمستخدم)
    Modeling,     // للرسم النهائي (اللي المستخدم هيشوفه)
}