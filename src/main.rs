#![allow(non_snake_case)]
use dioxus::prelude::*;
mod types;
mod geometry;
mod object;
mod canvas_coords;

use object::{VPoint, VLine, VCubicBezier};
use canvas_coords::{CoordMapper, SvgViewBox, AspectRatioMode};
use serde_json::Value;

#[derive(Clone, PartialEq, Debug)]
pub enum CanvasMode {
    PlacePoint,
    AwaitingLineStart,
    AwaitingLineEnd { first_point_id: u32 },
    // مراحل منحنى بيزيه
    BezierStart,
    BezierControl1 { p1: u32 },
    BezierControl2 { p1: u32, p2: u32 },
    BezierEnd { p1: u32, p2: u32, p3: u32 },
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut points = use_signal(|| Vec::<VPoint>::new());
    let mut lines = use_signal(|| Vec::<VLine>::new());
    let mut splines = use_signal(|| Vec::<VCubicBezier>::new());
    let mut mode = use_signal(|| CanvasMode::PlacePoint);
    let mut next_id = use_signal(|| 1u32);
    let mut svg_elem_size = use_signal(|| (1000.0, 1000.0)); // حجم افتراضي

    // استخدام eval للحصول على حجم الـ SVG الفعلي لضبط الإحداثيات
    use_effect(move || {
        let mut eval_instance = document::eval(r#"
            const el = document.getElementById('main-canvas');
            if (el) {
                const rect = el.getBoundingClientRect();
                dioxus.send([rect.width, rect.height]);
            }
        "#);
        
        spawn(async move {
            if let Ok(val) = eval_instance.recv().await {
                let val: Value = val;
                if let Some(arr) = val.as_array() {
                    let w = arr.get(0).and_then(|v: &Value| v.as_f64()).unwrap_or(1000.0);
                    let h = arr.get(1).and_then(|v: &Value| v.as_f64()).unwrap_or(1000.0);
                    svg_elem_size.set((w, h));
                }
            }
        });
    });

    // سحب البيانات من الـ Signals قبل الرسم لتجنب Deadlock
    let pts_snapshot = points.read().clone();
    let lns_snapshot = lines.read().clone();
    let spl_snapshot = splines.read().clone();
    let current_mode = mode.read().clone();

    let mode_text = match current_mode {
        CanvasMode::PlacePoint => "Click background to add Points",
        CanvasMode::AwaitingLineStart => "Click Start Point for Line",
        CanvasMode::AwaitingLineEnd { .. } => "Click End Point for Line",
        CanvasMode::BezierStart => "Bezier: Select Start Point",
        CanvasMode::BezierControl1 { .. } => "Bezier: Select Control Point 1",
        CanvasMode::BezierControl2 { .. } => "Bezier: Select Control Point 2",
        CanvasMode::BezierEnd { .. } => "Bezier: Select End Point",
    };

    rsx! {
        style { {include_str!("../assets/main.css")} }
        div { id: "container",
            div { id: "sidebar",
                h2 { "Valentina-Oxidized 🦀" }
                
                div { class: "toolbar",
                    button {
                        class: if matches!(current_mode, CanvasMode::PlacePoint) { "active" } else { "" },
                        onclick: move |_| mode.set(CanvasMode::PlacePoint),
                        "📍 Pt"
                    }
                    button {
                        class: if matches!(current_mode, CanvasMode::AwaitingLineStart | CanvasMode::AwaitingLineEnd { .. }) { "active" } else { "" },
                        onclick: move |_| mode.set(CanvasMode::AwaitingLineStart),
                        "📏 Line"
                    }
                    button {
                        class: if matches!(current_mode, CanvasMode::BezierStart | CanvasMode::BezierControl1 { .. } | CanvasMode::BezierControl2 { .. } | CanvasMode::BezierEnd { .. }) { "active" } else { "" },
                        onclick: move |_| mode.set(CanvasMode::BezierStart),
                        "➰ Spline"
                    }
                }

                p { class: "mode-hint", "{mode_text}" }
                
                h3 { "Entities" }
                ul {
                    li { "Points: {pts_snapshot.len()}" }
                    li { "Lines: {lns_snapshot.len()}" }
                    li { "Splines: {spl_snapshot.len()}" }
                }
            }

            div { id: "viewport",
                svg {
                    id: "main-canvas",
                    width: "100%", height: "100%", view_box: "0 0 1000 1000",
                    preserve_aspect_ratio: "xMidYMid meet", // تغيير لـ meet لسهولة الحساب
                    
                    defs {
                        pattern { id: "grid", width: "50", height: "50", pattern_units: "userSpaceOnUse",
                            path { d: "M 50 0 L 0 0 0 50", fill: "none", stroke: "#333", stroke_width: "0.5" }
                        }
                    }

                    // خلفية للنقر
                    rect {
                        width: "100%", height: "100%", fill: "url(#grid)",
                        onclick: move |evt| {
                            if *mode.read() == CanvasMode::PlacePoint {
                                let coords = evt.element_coordinates();
                                let (elem_w, elem_h) = *svg_elem_size.read();
                                let mapper = CoordMapper {
                                    viewbox: SvgViewBox { min_x: 0.0, min_y: 0.0, width: 1000.0, height: 1000.0 },
                                    preserve_aspect_ratio: AspectRatioMode::Meet,
                                };
                                let (svg_x, svg_y) = mapper.to_svg_space(coords.x, coords.y, elem_w, elem_h);
                                
                                let id = *next_id.read();
                                points.write().push(VPoint::new(id, &format!("P{}", id), svg_x, svg_y));
                                next_id.set(id + 1);
                            }
                        }
                    }

                    // رسم المنحنيات (Splines)
                    for spline in spl_snapshot.iter() {
                        {
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p1_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p2_id);
                            let p3 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p3_id);
                            let p4 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p4_id);
                            
                            if let (Some(s), Some(c1), Some(c2), Some(e)) = (p1, p2, p3, p4) {
                                let d_path = format!("M {} {} C {} {}, {} {}, {} {}", 
                                    s.x(), s.y(), c1.x(), c1.y(), c2.x(), c2.y(), e.x(), e.y());
                                rsx! {
                                    path { 
                                        key: "{spline.metadata.id}",
                                        d: "{d_path}",
                                        stroke: "#2ecc71", 
                                        stroke_width: "3",
                                        fill: "none"
                                    }
                                }
                            } else { rsx! { "" } }
                        }
                    }

                    // رسم الخطوط
                    for line in lns_snapshot.iter() {
                        {
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == line.start_point_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == line.end_point_id);
                            
                            if let (Some(start), Some(end)) = (p1, p2) {
                                rsx! {
                                    line { 
                                        key: "{line.metadata.id}",
                                        x1: "{start.x()}", y1: "{start.y()}", 
                                        x2: "{end.x()}", y2: "{end.y()}", 
                                        stroke: "#3498db", stroke_width: "2" 
                                    }
                                }
                            } else { rsx! { "" } }
                        }
                    }

                    // رسم النقاط
                    for p in pts_snapshot.iter() {
                        {
                            let pid = p.metadata.id;
                            let px = p.x();
                            let py = p.y();
                            
                            let is_bezier_active = match &current_mode {
                                CanvasMode::BezierControl1 { p1 } => *p1 == pid,
                                CanvasMode::BezierControl2 { p1, p2 } => *p1 == pid || *p2 == pid,
                                CanvasMode::BezierEnd { p1, p2, p3 } => *p1 == pid || *p2 == pid || *p3 == pid,
                                _ => false,
                            };

                            let is_line_active = matches!(
                                &current_mode,
                                CanvasMode::AwaitingLineEnd { first_point_id } if *first_point_id == pid
                            );

                            let is_active = is_line_active || is_bezier_active;
                            
                            let fill_color = if is_active { "#f1c40f" } 
                                             else if !matches!(current_mode, CanvasMode::PlacePoint) { "#e67e22" }
                                             else { "#e74c3c" };

                            rsx! {
                                circle { 
                                    key: "{pid}",
                                    cx: "{px}", cy: "{py}", r: "10", 
                                    fill: "{fill_color}",
                                    stroke: if is_active { "white" } else { "none" },
                                    stroke_width: "2",
                                    style: "cursor: pointer; pointer-events: all;",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        let mode_val = mode.read().clone();
                                        match mode_val {
                                            CanvasMode::AwaitingLineStart => {
                                                mode.set(CanvasMode::AwaitingLineEnd { first_point_id: pid });
                                            }
                                            CanvasMode::AwaitingLineEnd { first_point_id } => {
                                                if first_point_id != pid {
                                                    let lid = *next_id.read();
                                                    let line_name = format!("L{}", lid);
                                                    lines.write().push(VLine::new(lid, &line_name, first_point_id, pid));
                                                    next_id.set(lid + 1);
                                                }
                                                mode.set(CanvasMode::AwaitingLineStart);
                                            }
                                            CanvasMode::BezierStart => {
                                                mode.set(CanvasMode::BezierControl1 { p1: pid });
                                            }
                                            CanvasMode::BezierControl1 { p1 } => {
                                                mode.set(CanvasMode::BezierControl2 { p1, p2: pid });
                                            }
                                            CanvasMode::BezierControl2 { p1, p2 } => {
                                                mode.set(CanvasMode::BezierEnd { p1, p2, p3: pid });
                                            }
                                            CanvasMode::BezierEnd { p1, p2, p3 } => {
                                                if pid != p1 && pid != p2 && pid != p3 {
                                                    let sid = *next_id.read();
                                                    let spline_name = format!("S{}", sid);
                                                    splines.write().push(VCubicBezier::new(sid, &spline_name, p1, p2, p3, pid));
                                                    next_id.set(sid + 1);
                                                }
                                                mode.set(CanvasMode::BezierStart);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
