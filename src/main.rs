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
// ... (نفس الـ imports القديمة)

// ... نفس الـ imports السابقة

fn App() -> Element {
    // 1. تغيير الحالة إلى قائمة (Vector) من النقاط
    let mut points = use_signal(|| Vec::<VPoint>::new());
    let mut next_id = use_signal(|| 1u32);

    rsx! {
        style { {include_str!("../assets/main.css")} }
        
        div { id: "container",
            div { id: "sidebar",
                h2 { "Valentina-Oxidized 🦀" }
                p { "Click on the grid to add points" }
                hr {}
                div { class: "info-box",
                    h3 { "Points List" }
                    ul {
                        for p in points().iter() {
                            li { "{p.metadata.name}: ({p.x():.1}, {p.y():.1})" }
                        }
                    }
                }
            }

            div { id: "viewport",
                svg {
                    width: "100%",
                    height: "100%",
                    view_box: "0 0 1000 1000",
                    
                    // 2. التقاط حدث الضغط على الـ SVG
                    onclick: move |evt| {
                        let coords = evt.element_coordinates();
                        let name = format!("P{}", next_id());
                        
                        // إنشاء النقطة الجديدة وإضافتها للمخزن
                        let new_point = VPoint::new(next_id(), &name, coords.x, coords.y);
                        points.with_mut(|p_vec| p_vec.push(new_point));
                        
                        // زيادة الـ ID للمرة القادمة
                        next_id += 1;
                    },

                    // رسم الشبكة
                    defs {
                        pattern { id: "grid", width: "50", height: "50", pattern_units: "userSpaceOnUse",
                            path { d: "M 50 0 L 0 0 0 50", fill: "none", stroke: "#ccc", stroke_width: "0.5" }
                        }
                    }
                    rect { width: "100%", height: "100%", fill: "url(#grid)" }

                    // 3. رسم كل النقاط الموجودة في القائمة
                    for p in points().iter() {
                        circle { 
                            cx: "{p.x()}", 
                            cy: "{p.y()}", 
                            r: "6", 
                            fill: "red",
                            style: "filter: drop-shadow(0px 0px 3px rgba(0,0,0,0.5));"
                        }
                        text { 
                            x: "{p.x() + 8.0}", 
                            y: "{p.y() - 8.0}", 
                            font_size: "12",
                            "{p.metadata.name}" 
                        }
                    }
                }
            }
        }
    }
}