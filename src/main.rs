#![allow(non_snake_case)]
use dioxus::prelude::*;
mod types;
mod geometry;
mod object;
mod canvas_coords;

use object::{VPoint, VLine, VCubicBezier, VBisector, VContour, SelectedItem, EntityRef};
use canvas_coords::{CoordMapper, SvgViewBox, AspectRatioMode};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::fs;

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
    // مراحل المنصف
    BisectorStart,
    BisectorVertex { p1: u32 },
    BisectorEnd { p1: u32, vertex: u32 },
    // مرحلة إنشاء الكونتور (المسار)
    ContourCreation { active_contour_id: u32 },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub points: Vec<VPoint>,
    pub lines: Vec<VLine>,
    pub splines: Vec<VCubicBezier>,
    pub bisectors: Vec<VBisector>,
    pub contours: Vec<VContour>,
    pub next_id: u32,
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut points = use_signal(|| Vec::<VPoint>::new());
    let mut lines = use_signal(|| Vec::<VLine>::new());
    let mut splines = use_signal(|| Vec::<VCubicBezier>::new());
    let mut bisectors = use_signal(|| Vec::<VBisector>::new());
    let mut contours = use_signal(|| Vec::<VContour>::new());
    let mut mode = use_signal(|| CanvasMode::PlacePoint);
    let mut selected_item = use_signal(|| SelectedItem::None);
    let mut dragging_point_id = use_signal(|| None::<u32>);
    let mut next_id = use_signal(|| 1u32);
    let mut svg_elem_size = use_signal(|| (1000.0, 1000.0));

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
    let bis_snapshot = bisectors.read().clone();
    let cnt_snapshot = contours.read().clone();
    let current_mode = mode.read().clone();
    let current_selection = selected_item.read().clone();

    let mode_text = match current_mode {
        CanvasMode::PlacePoint => "Click background to add Points / Drag to move",
        CanvasMode::AwaitingLineStart => "Line: Select Start Point",
        CanvasMode::AwaitingLineEnd { .. } => "Line: Select End Point",
        CanvasMode::BezierStart => "Bezier: Select Start Point",
        CanvasMode::BezierControl1 { .. } => "Bezier: Select Control Point 1",
        CanvasMode::BezierControl2 { .. } => "Bezier: Select Control Point 2",
        CanvasMode::BezierEnd { .. } => "Bezier: Select End Point",
        CanvasMode::BisectorStart => "Bisector: Select first point",
        CanvasMode::BisectorVertex { .. } => "Bisector: Select vertex (corner)",
        CanvasMode::BisectorEnd { .. } => "Bisector: Select third point",
        CanvasMode::ContourCreation { .. } => "Contour: Select lines/splines to group",
    };

    rsx! {
        style { {include_str!("../assets/main.css")} }
        div { 
            id: "container",
            style: if dragging_point_id().is_some() { "user-select: none; cursor: grabbing;" } else { "" },
            
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
                        "📏 Ln"
                    }
                    button {
                        class: if matches!(current_mode, CanvasMode::BezierStart | CanvasMode::BezierControl1 { .. } | CanvasMode::BezierControl2 { .. } | CanvasMode::BezierEnd { .. }) { "active" } else { "" },
                        onclick: move |_| mode.set(CanvasMode::BezierStart),
                        "➰ Spl"
                    }
                    button {
                        class: if matches!(current_mode, CanvasMode::BisectorStart | CanvasMode::BisectorVertex { .. } | CanvasMode::BisectorEnd { .. }) { "active" } else { "" },
                        onclick: move |_| mode.set(CanvasMode::BisectorStart),
                        "📐 Bis"
                    }
                    button {
                        class: if matches!(current_mode, CanvasMode::ContourCreation { .. }) { "active" } else { "" },
                        onclick: move |_| {
                            let new_cid = *next_id.read();
                            let c_name = format!("Path{}", new_cid);
                            contours.write().push(VContour::new(new_cid, &c_name));
                            next_id.set(new_cid + 1);
                            mode.set(CanvasMode::ContourCreation { active_contour_id: new_cid });
                        },
                        "🧩 Path"
                    }
                }

                p { class: "mode-hint", "{mode_text}" }
                
                div { class: "info-box",
                    h3 { "Selection" }
                    match current_selection {
                        SelectedItem::None => rsx! { p { "Nothing selected" } },
                        SelectedItem::Point(id) => rsx! { 
                            div {
                                p { "Selected Point: P{id}" }
                                button { 
                                    class: "delete-btn",
                                    onclick: move |_| {
                                        points.write().retain(|p| p.metadata.id != id);
                                        lines.write().retain(|l| l.start_point_id != id && l.end_point_id != id);
                                        splines.write().retain(|s| s.p1_id != id && s.p2_id != id && s.p3_id != id && s.p4_id != id);
                                        bisectors.write().retain(|b| b.p1_id != id && b.vertex_id != id && b.p3_id != id);
                                        selected_item.set(SelectedItem::None);
                                    },
                                    "🗑 Delete"
                                }
                            }
                        },
                        SelectedItem::Line(id) => rsx! { 
                            div {
                                p { "Selected Line: L{id}" }
                                button { 
                                    class: "delete-btn",
                                    onclick: move |_| {
                                        lines.write().retain(|l| l.metadata.id != id);
                                        selected_item.set(SelectedItem::None);
                                    },
                                    "🗑 Delete"
                                }
                            }
                        },
                        SelectedItem::Spline(id) => rsx! { 
                            div {
                                p { "Selected Spline: S{id}" }
                                button { 
                                    class: "delete-btn",
                                    onclick: move |_| {
                                        splines.write().retain(|s| s.metadata.id != id);
                                        selected_item.set(SelectedItem::None);
                                    },
                                    "🗑 Delete"
                                }
                            }
                        },
                        SelectedItem::Bisector(id) => rsx! { 
                            div {
                                p { "Selected Bisector: B{id}" }
                                button { 
                                    class: "delete-btn",
                                    onclick: move |_| {
                                        bisectors.write().retain(|b| b.metadata.id != id);
                                        selected_item.set(SelectedItem::None);
                                    },
                                    "🗑 Delete"
                                }
                            }
                        },
                        SelectedItem::Contour(id) => rsx! { 
                            div {
                                p { "Selected Path: {id}" }
                                if let Some(c) = cnt_snapshot.iter().find(|c| c.metadata.id == id) {
                                    p { class: "stats", "Entities: {c.entities.len()}" }
                                }
                                button { 
                                    class: "delete-btn",
                                    onclick: move |_| {
                                        contours.write().retain(|c| c.metadata.id != id);
                                        selected_item.set(SelectedItem::None);
                                    },
                                    "🗑 Delete Path"
                                }
                            }
                        },
                    }
                }

                div { class: "control-box",
                    h3 { "Project" }
                    button { 
                        class: "action-btn",
                        onclick: move |_| {
                            let pts = points.read().clone();
                            let lns = lines.read().clone();
                            let spls = splines.read().clone();
                            let bis = bisectors.read().clone();
                            let cnts = contours.read().clone();
                            let nid = *next_id.read();
                            
                            spawn(async move {
                                if let Some(path) = rfd::AsyncFileDialog::new()
                                    .set_file_name("project.json")
                                    .add_filter("JSON", &["json"])
                                    .save_file()
                                    .await {
                                        let data = ProjectData {
                                            points: pts,
                                            lines: lns,
                                            splines: spls,
                                            bisectors: bis,
                                            contours: cnts,
                                            next_id: nid,
                                        };
                                        if let Ok(json) = serde_json::to_string_pretty(&data) {
                                            let _ = fs::write(path.path(), json);
                                        }
                                }
                            });
                        },
                        "💾 Save"
                    }
                    button { 
                        class: "action-btn",
                        onclick: move |_| {
                            spawn(async move {
                                if let Some(path) = rfd::AsyncFileDialog::new()
                                    .add_filter("JSON", &["json"])
                                    .pick_file()
                                    .await {
                                        if let Ok(json) = fs::read_to_string(path.path()) {
                                            if let Ok(data) = serde_json::from_str::<ProjectData>(&json) {
                                                points.set(data.points);
                                                lines.set(data.lines);
                                                splines.set(data.splines);
                                                bisectors.set(data.bisectors);
                                                contours.set(data.contours);
                                                next_id.set(data.next_id);
                                                selected_item.set(SelectedItem::None);
                                            }
                                        }
                                }
                            });
                        },
                        "📂 Load"
                    }
                }

                h3 { "Entities" }
                ul {
                    li { "Points: {pts_snapshot.len()}" }
                    li { "Lines: {lns_snapshot.len() + bis_snapshot.len()}" }
                    li { "Splines: {spl_snapshot.len()}" }
                    li { "Paths: {cnt_snapshot.len()}" }
                }
            }

            div { 
                id: "viewport",
                onmousemove: move |evt| {
                    if let Some(pid) = *dragging_point_id.read() {
                        let coords = evt.element_coordinates();
                        let (elem_w, elem_h) = *svg_elem_size.read();
                        let mapper = CoordMapper {
                            viewbox: SvgViewBox { min_x: 0.0, min_y: 0.0, width: 1000.0, height: 1000.0 },
                            preserve_aspect_ratio: AspectRatioMode::Meet,
                        };
                        let (svg_x, svg_y) = mapper.to_svg_space(coords.x, coords.y, elem_w, elem_h);
                        
                        let mut points_lock = points.write();
                        if let Some(p) = points_lock.iter_mut().find(|p| p.metadata.id == pid) {
                            p.coords.x = svg_x;
                            p.coords.y = svg_y;
                        }
                    }
                },
                onmouseup: move |_| {
                    dragging_point_id.set(None);
                },
                svg {
                    id: "main-canvas",
                    width: "100%", height: "100%", view_box: "0 0 1000 1000",
                    preserve_aspect_ratio: "xMidYMid meet",
                    
                    defs {
                        pattern { id: "grid", width: "50", height: "50", pattern_units: "userSpaceOnUse",
                            path { d: "M 50 0 L 0 0 0 50", fill: "none", stroke: "#333", stroke_width: "0.5" }
                        }
                    }

                    rect {
                        width: "100%", height: "100%", fill: "url(#grid)",
                        onmousedown: move |evt| {
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
                            selected_item.set(SelectedItem::None);
                        }
                    }

                    // رسم المنحنيات (Splines)
                    for spline in spl_snapshot.iter() {
                        {
                            let sid = spline.metadata.id;
                            let is_selected = matches!(current_selection, SelectedItem::Spline(id) if id == sid);
                            // تمييز إذا كان جزء من كونتور
                            let is_in_contour = cnt_snapshot.iter().any(|c| c.entities.contains(&EntityRef::Spline(sid)));
                            
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p1_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p2_id);
                            let p3 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p3_id);
                            let p4 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p4_id);
                            
                            if let (Some(s), Some(c1), Some(c2), Some(e)) = (p1, p2, p3, p4) {
                                let d_path = format!("M {} {} C {} {}, {} {}, {} {}", 
                                    s.x(), s.y(), c1.x(), c1.y(), c2.x(), c2.y(), e.x(), e.y());
                                rsx! {
                                    path { 
                                        key: "spl-{sid}",
                                        class: if is_selected { "selected" } else { "" },
                                        d: "{d_path}",
                                        stroke: if is_in_contour { "#f39c12" } else { "#2ecc71" }, 
                                        stroke_width: if is_in_contour { "5" } else { "3" },
                                        fill: "none",
                                        onmousedown: move |evt| {
                                            evt.stop_propagation();
                                            if let CanvasMode::ContourCreation { active_contour_id } = *mode.read() {
                                                if let Some(c) = contours.write().iter_mut().find(|c| c.metadata.id == active_contour_id) {
                                                    c.entities.push(EntityRef::Spline(sid));
                                                }
                                            } else {
                                                selected_item.set(SelectedItem::Spline(sid));
                                            }
                                        }
                                    }
                                }
                            } else { rsx! { "" } }
                        }
                    }

                    // رسم الخطوط العادية
                    for line in lns_snapshot.iter() {
                        {
                            let lid = line.metadata.id;
                            let is_selected = matches!(current_selection, SelectedItem::Line(id) if id == lid);
                            let is_in_contour = cnt_snapshot.iter().any(|c| c.entities.contains(&EntityRef::Line(lid)));

                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == line.start_point_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == line.end_point_id);
                            
                            if let (Some(start), Some(end)) = (p1, p2) {
                                rsx! {
                                    line { 
                                        key: "ln-{lid}",
                                        class: if is_selected { "selected" } else { "" },
                                        x1: "{start.x()}", y1: "{start.y()}", 
                                        x2: "{end.x()}", y2: "{end.y()}", 
                                        stroke: if is_in_contour { "#f39c12" } else { "#3498db" }, 
                                        stroke_width: if is_in_contour { "5" } else { "3" },
                                        onmousedown: move |evt| {
                                            evt.stop_propagation();
                                            if let CanvasMode::ContourCreation { active_contour_id } = *mode.read() {
                                                if let Some(c) = contours.write().iter_mut().find(|c| c.metadata.id == active_contour_id) {
                                                    c.entities.push(EntityRef::Line(lid));
                                                }
                                            } else {
                                                selected_item.set(SelectedItem::Line(lid));
                                            }
                                        }
                                    }
                                }
                            } else { rsx! { "" } }
                        }
                    }

                    // رسم المنصفات (Bisectors)
                    for bis in bis_snapshot.iter() {
                        {
                            let bid = bis.metadata.id;
                            let is_selected = matches!(current_selection, SelectedItem::Bisector(id) if id == bid);
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == bis.p1_id);
                            let vertex = pts_snapshot.iter().find(|p| p.metadata.id == bis.vertex_id);
                            let p3 = pts_snapshot.iter().find(|p| p.metadata.id == bis.p3_id);
                            
                            if let (Some(p1), Some(v), Some(p3)) = (p1, vertex, p3) {
                                let end_coords = bis.calculate_end_point(p1, v, p3);
                                rsx! {
                                    line { 
                                        key: "bis-{bid}",
                                        class: if is_selected { "selected" } else { "" },
                                        x1: "{v.x()}", y1: "{v.y()}", 
                                        x2: "{end_coords.x}", y2: "{end_coords.y}", 
                                        stroke: "#9b59b6", stroke_width: "2",
                                        stroke_dasharray: "5,5",
                                        onmousedown: move |evt| {
                                            evt.stop_propagation();
                                            if let CanvasMode::ContourCreation { active_contour_id } = *mode.read() {
                                                if let Some(c) = contours.write().iter_mut().find(|c| c.metadata.id == active_contour_id) {
                                                    c.entities.push(EntityRef::Bisector(bid));
                                                }
                                            } else {
                                                selected_item.set(SelectedItem::Bisector(bid));
                                            }
                                        }
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
                            let is_selected = matches!(current_selection, SelectedItem::Point(id) if id == pid);
                            
                            let is_active = match &current_mode {
                                CanvasMode::AwaitingLineEnd { first_point_id } => *first_point_id == pid,
                                CanvasMode::BezierControl1 { p1 } => *p1 == pid,
                                CanvasMode::BezierControl2 { p1, p2 } => *p1 == pid || *p2 == pid,
                                CanvasMode::BezierEnd { p1, p2, p3 } => *p1 == pid || *p2 == pid || *p3 == pid,
                                CanvasMode::BisectorVertex { p1 } => *p1 == pid,
                                CanvasMode::BisectorEnd { p1, vertex } => *p1 == pid || *vertex == pid,
                                _ => false,
                            } || is_selected;
                            
                            let fill_color = if is_active { "#f1c40f" } 
                                             else if is_selected { "#f1c40f" }
                                             else if !matches!(current_mode, CanvasMode::PlacePoint) { "#e67e22" }
                                             else { "#e74c3c" };

                            rsx! {
                                g {
                                    key: "pt-group-{pid}",
                                    circle { 
                                        key: "pt-{pid}",
                                        class: if is_selected { "selected" } else { "" },
                                        cx: "{px}", cy: "{py}", r: "10", 
                                        fill: "{fill_color}",
                                        stroke: if is_active || is_selected { "white" } else { "none" },
                                        stroke_width: "2",
                                        style: "cursor: grab; pointer-events: all;",
                                        onmousedown: move |evt| {
                                            evt.stop_propagation();
                                            let current_m = mode.read().clone();
                                            match current_m {
                                                CanvasMode::PlacePoint => {
                                                    dragging_point_id.set(Some(pid));
                                                    selected_item.set(SelectedItem::Point(pid));
                                                }
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
                                                CanvasMode::BisectorStart => {
                                                    mode.set(CanvasMode::BisectorVertex { p1: pid });
                                                }
                                                CanvasMode::BisectorVertex { p1 } => {
                                                    mode.set(CanvasMode::BisectorEnd { p1, vertex: pid });
                                                }
                                                CanvasMode::BisectorEnd { p1, vertex } => {
                                                    if pid != p1 && pid != vertex {
                                                        let bid = *next_id.read();
                                                        let bis_name = format!("B{}", bid);
                                                        bisectors.write().push(VBisector::new(bid, &bis_name, p1, vertex, pid, 150.0));
                                                        next_id.set(bid + 1);
                                                    }
                                                    mode.set(CanvasMode::BisectorStart);
                                                }
                                                CanvasMode::ContourCreation { .. } => {
                                                    selected_item.set(SelectedItem::Point(pid));
                                                }
                                            }
                                        }
                                    }
                                    text {
                                        key: "lbl-{pid}",
                                        x: "{px + 15.0}",
                                        y: "{py - 15.0}",
                                        fill: "#2c3e50",
                                        font_size: "18",
                                        font_weight: "bold",
                                        style: "pointer-events: none; user-select: none;",
                                        "{p.metadata.name}"
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
