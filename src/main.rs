#![allow(non_snake_case)]
use dioxus::prelude::*;
mod types;
mod geometry;
mod object;

use object::VPoint;

fn main() {
    // في الإصدارات الحديثة نستخدم LaunchBuilder أو مباشرة Launch
    launch(App);
}

#[component]
fn App() -> Element {
    // استخدام Signal لإدارة الحالة (State)
    let mut point = use_signal(|| VPoint::new(1, "Point A", 150.0, 150.0));
    let mut angle = use_signal(|| 0.0);

    rsx! {
        // إذا لم يكن لديك ملف CSS بعد، يمكنك كتابة التنسيق هنا مباشرة لتجنب أخطاء المسارات
        style { {include_str!("../assets/main.css")} }
        
        div { id: "container",
            div { id: "sidebar",
                h2 { "Valentina-Oxidized 🦀" }
                hr {}
                div { class: "info-box",
                    h3 { "Object Info" }
                    p { "Name: {point.read().metadata.name}" }
                    p { "ID: {point.read().metadata.id}" }
                }
                
                div { class: "control-box",
                    h3 { "Rotation" }
                    label { "Angle: {angle}°" }
                    input { 
                        r#type: "range", min: "0", max: "360", value: "{angle}",
                        oninput: move |evt| {
                            let new_angle: f64 = evt.value().parse().unwrap_or(0.0);
                            angle.set(new_angle);
                            
                            let origin = geometry::Point2D::new(200.0, 200.0);
                            let mut new_p = VPoint::new(1, "Point A", 150.0, 150.0);
                            new_p.coords = new_p.coords.rotate(&origin, new_angle);
                            point.set(new_p);
                        }
                    }
                }
            }

            div { id: "viewport",
                div {
                    class: "virtual-point",
                    style: "left: {point.read().x()}px; top: {point.read().y()}px;",
                    "•"
                }
            }
        }
    }
}