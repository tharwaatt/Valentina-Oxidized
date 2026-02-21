#![allow(non_snake_case)]
use dioxus::prelude::*;
mod types;
mod geometry;
mod object;
mod canvas_coords;

use object::{VPoint, VLine, VCubicBezier, VBisector, VPointAlongLine, VContour, VMirror, SelectedItem, EntityRef};
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
    // مراحل الكونتور
    ContourCreation { active_contour_id: u32 },
    // مراحل الانعكاس
    MirrorSelection { selected_entities: Vec<EntityRef> },
    MirrorAxisStart { selected_entities: Vec<EntityRef> },
    MirrorAxisEnd { selected_entities: Vec<EntityRef>, axis_p1: u32 },
    AlongLineStart,
    AlongLineEnd { p1: u32 },
    ArcCenter,
    ArcRadius { center_id: u32 },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub points: Vec<VPoint>,
    pub lines: Vec<VLine>,
    pub splines: Vec<VCubicBezier>,
    pub bisectors: Vec<VBisector>,
    pub along_lines: Vec<VPointAlongLine>,
    pub arcs: Vec<object::VArc>,
    pub contours: Vec<VContour>,
    pub mirrors: Vec<VMirror>,
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
    let mut along_lines = use_signal(|| Vec::<VPointAlongLine>::new());
    let mut arcs = use_signal(|| Vec::<object::VArc>::new());
    let mut contours = use_signal(|| Vec::<VContour>::new());
    let mut mirrors = use_signal(|| Vec::<VMirror>::new());
    let mut mode = use_signal(|| CanvasMode::PlacePoint);
    let mut selected_item = use_signal(|| SelectedItem::None);
    let mut dragging_point_id = use_signal(|| None::<u32>);
    let mut next_id = use_signal(|| 1u32);

    // سحب البيانات من الـ Signals للرسم (Snapshots) لتجنب Deadlock
    let pts_snapshot = points.read().clone();
    let lns_snapshot = lines.read().clone();
    let spl_snapshot = splines.read().clone();
    let mir_snapshot = mirrors.read().clone();
    let bis_snapshot = bisectors.read().clone();
    let alg_snapshot = along_lines.read().clone();
    let arc_snapshot = arcs.read().clone();
    let current_mode = mode.read().clone();
    let current_selection = selected_item.read().clone();

    let mode_text = match current_mode {
        CanvasMode::PlacePoint => "📍 Pt Mode: Click to add / Drag to move",
        CanvasMode::AwaitingLineStart => "📏 Line: Select start",
        CanvasMode::AwaitingLineEnd { .. } => "📏 Line: Select end",
        CanvasMode::BezierStart => "➰ Bezier: Select P1",
        CanvasMode::BezierControl1 { .. } => "➰ Bezier: Select C1",
        CanvasMode::BezierControl2 { .. } => "➰ Bezier: Select C2",
        CanvasMode::BezierEnd { .. } => "➰ Bezier: Select P2",
        CanvasMode::BisectorStart => "📐 Bisector: Select P1",
        CanvasMode::BisectorVertex { .. } => "📐 Bisector: Select vertex",
        CanvasMode::BisectorEnd { .. } => "📐 Bisector: Select P3",
        CanvasMode::AlongLineStart => "📏 Along Line: Select start",
        CanvasMode::AlongLineEnd { .. } => "📏 Along Line: Select direction",
        CanvasMode::ArcCenter => "🌈 Arc: Select center point",
        CanvasMode::ArcRadius { .. } => "🌈 Arc: Select point for radius/angle",
        _ => "Action in progress...",
    };

    rsx! {
        style { {include_str!("../assets/main.css")} }
        div { 
            id: "container",
            style: if dragging_point_id().is_some() { "user-select: none; cursor: grabbing;" } else { "" },
            
            div { id: "sidebar",
                h2 { "Valentina-Oxidized 🦀" }
                
                div { class: "toolbar",
                    button { class: if matches!(current_mode, CanvasMode::PlacePoint) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::PlacePoint), "📍 Pt" }
                    button { class: if matches!(current_mode, CanvasMode::AwaitingLineStart | CanvasMode::AwaitingLineEnd { .. }) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::AwaitingLineStart), "📏 Ln" }
                    button { class: if matches!(current_mode, CanvasMode::BezierStart | CanvasMode::BezierControl1 { .. } | CanvasMode::BezierControl2 { .. } | CanvasMode::BezierEnd { .. }) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::BezierStart), "➰ Spl" }
                    button { class: if matches!(current_mode, CanvasMode::ArcCenter | CanvasMode::ArcRadius { .. }) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::ArcCenter), "🌈 Arc" }
                    button { class: if matches!(current_mode, CanvasMode::BisectorStart | CanvasMode::BisectorVertex { .. } | CanvasMode::BisectorEnd { .. }) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::BisectorStart), "📐 Bis" }
                    button { class: if matches!(current_mode, CanvasMode::AlongLineStart | CanvasMode::AlongLineEnd { .. }) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::AlongLineStart), "📏+ Pt" }
                    button { class: if matches!(current_mode, CanvasMode::MirrorSelection { .. } | CanvasMode::MirrorAxisStart { .. } | CanvasMode::MirrorAxisEnd { .. }) { "active" } else { "" }, onclick: move |_| mode.set(CanvasMode::MirrorSelection { selected_entities: Vec::new() }), "آ Mir" }
                }

                p { class: "mode-hint", "{mode_text}" }
                
                if let CanvasMode::MirrorSelection { selected_entities } = current_mode.clone() {
                    button { class: "action-btn", style: "width: 100%; background: #f39c12;", onclick: move |_| mode.set(CanvasMode::MirrorAxisStart { selected_entities: selected_entities.clone() }), "Done Selecting" }
                }

                div { class: "info-box",
                    h3 { "Selection" }
                    match current_selection {
                        SelectedItem::None => rsx! { p { "Nothing selected" } },
                        SelectedItem::Point(id) => rsx! { 
                            div {
                                p { "Selected Point: P{id}" }
                                button { class: "delete-btn", onclick: move |_| { points.write().retain(|p| p.metadata.id != id); lines.write().retain(|l| l.start_point_id != id && l.end_point_id != id); splines.write().retain(|s| s.p1_id != id && s.p2_id != id && s.p3_id != id && s.p4_id != id); along_lines.write().retain(|a| a.p1_id != id && a.p2_id != id); arcs.write().retain(|a| a.center_id != id); selected_item.set(SelectedItem::None); }, "🗑 Delete" }
                            }
                        },
                        _ => rsx! { p { "Entity selected" } }
                    }
                }

                div { class: "control-box",
                    h3 { "Project" }
                    button { class: "action-btn", onclick: move |_| { 
                        let pts = points.read().clone(); let lns = lines.read().clone(); let spls = splines.read().clone(); let biss = bisectors.read().clone(); let algs = along_lines.read().clone(); let arcs_data = arcs.read().clone(); let mirs = mirrors.read().clone(); let nid = *next_id.read();
                        let cnts = contours.read().clone();
                        spawn(async move {
                            if let Some(path) = rfd::AsyncFileDialog::new().set_file_name("project.json").save_file().await {
                                let data = ProjectData { points: pts, lines: lns, splines: spls, bisectors: biss, along_lines: algs, arcs: arcs_data, contours: cnts, mirrors: mirs, next_id: nid };
                                if let Ok(json) = serde_json::to_string_pretty(&data) { let _ = fs::write(path.path(), json); }
                            }
                        });
                    }, "💾 Save" }
                    button { class: "action-btn", onclick: move |_| {
                        spawn(async move {
                            if let Some(path) = rfd::AsyncFileDialog::new().pick_file().await {
                                if let Ok(json) = fs::read_to_string(path.path()) {
                                    if let Ok(data) = serde_json::from_str::<ProjectData>(&json) {
                                        points.set(data.points); lines.set(data.lines); splines.set(data.splines); bisectors.set(data.bisectors); along_lines.set(data.along_lines); arcs.set(data.arcs); mirrors.set(data.mirrors); next_id.set(data.next_id);
                                    }
                                }
                            }
                        });
                    }, "📂 Load" }
                }
            }

            div { 
                id: "viewport",
                onmousemove: move |evt| {
                    if let Some(pid) = *dragging_point_id.read() {
                        let client_x = evt.client_coordinates().x;
                        let client_y = evt.client_coordinates().y;
                        let mut p_sig = points;
                        let mut eval_instance = document::eval(&format!(r#"
                            const svg = document.getElementById('main-canvas');
                            if (svg) {{
                                const pt = svg.createSVGPoint();
                                pt.x = {}; pt.y = {};
                                const transformed = pt.matrixTransform(svg.getScreenCTM().inverse());
                                dioxus.send([transformed.x, transformed.y]);
                            }}
                        "#, client_x, client_y));
                        spawn(async move {
                            if let Ok(val) = eval_instance.recv().await {
                                let val: Value = val;
                                if let Some(arr) = val.as_array() {
                                    let x = arr.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                    let y = arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                    let mut pts = p_sig.write();
                                    if let Some(p) = pts.iter_mut().find(|p| p.metadata.id == pid) {
                                        p.coords.x = x; p.coords.y = y;
                                    }
                                }
                            }
                        });
                    }
                },
                onmouseup: move |_| dragging_point_id.set(None),
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
                            let client_x = evt.client_coordinates().x;
                            let client_y = evt.client_coordinates().y;
                            let mut p_sig = points;
                            let mut id_sig = next_id;
                            let m_sig = mode;
                            let mut sel_sig = selected_item;
                            
                            let mut eval_instance = document::eval(&format!(r#"
                                const svg = document.getElementById('main-canvas');
                                if (svg) {{
                                    const pt = svg.createSVGPoint();
                                    pt.x = {}; pt.y = {};
                                    const transformed = pt.matrixTransform(svg.getScreenCTM().inverse());
                                    dioxus.send([transformed.x, transformed.y]);
                                }}
                            "#, client_x, client_y));

                            spawn(async move {
                                if let Ok(val) = eval_instance.recv().await {
                                    let val: Value = val;
                                    if let Some(arr) = val.as_array() {
                                        let x = arr.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let y = arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        if *m_sig.read() == CanvasMode::PlacePoint {
                                            let id = *id_sig.read();
                                            p_sig.write().push(VPoint::new(id, &format!("P{}", id), x, y));
                                            id_sig.set(id + 1);
                                        }
                                        sel_sig.set(SelectedItem::None);
                                    }
                                }
                            });
                        }
                    }

                    // محرك الرسم للانعكاس
                    for mir in mir_snapshot.iter() {
                        {
                            let ap1 = pts_snapshot.iter().find(|p| p.metadata.id == mir.axis_p1_id);
                            let ap2 = pts_snapshot.iter().find(|p| p.metadata.id == mir.axis_p2_id);
                            if let (Some(axis1), Some(axis2)) = (ap1, ap2) {
                                rsx! {
                                    for entity in mir.source_entities.iter() {
                                        {
                                            match entity {
                                                EntityRef::Line(lid) => {
                                                    if let Some(l) = lns_snapshot.iter().find(|l| l.metadata.id == *lid) {
                                                        let p1 = pts_snapshot.iter().find(|p| p.metadata.id == l.start_point_id);
                                                        let p2 = pts_snapshot.iter().find(|p| p.metadata.id == l.end_point_id);
                                                        if let (Some(s), Some(e)) = (p1, p2) {
                                                            let ms = s.coords.mirror_over_line(&axis1.coords, &axis2.coords);
                                                            let me = e.coords.mirror_over_line(&axis1.coords, &axis2.coords);
                                                            rsx! { line { x1: "{ms.x}", y1: "{ms.y}", x2: "{me.x}", y2: "{me.y}", stroke: "rgba(52, 152, 219, 0.5)", stroke_width: "2", stroke_dasharray: "4" } }
                                                        } else { rsx! { "" } }
                                                    } else { rsx! { "" } }
                                                }
                                                EntityRef::Point(pid) => {
                                                    if let Some(p) = pts_snapshot.iter().find(|p| p.metadata.id == *pid) {
                                                        let mp = p.coords.mirror_over_line(&axis1.coords, &axis2.coords);
                                                        rsx! { circle { cx: "{mp.x}", cy: "{mp.y}", r: "6", fill: "rgba(231, 76, 60, 0.5)" } }
                                                    } else { rsx! { "" } }
                                                }
                                                _ => rsx! { "" }
                                            }
                                        }
                                    }
                                }
                            } else { rsx! { "" } }
                        }
                    }

                    // رسم الأقواس
                    for arc in arc_snapshot.iter() {
                        {
                            if let Some(center) = pts_snapshot.iter().find(|p| p.metadata.id == arc.center_id) {
                                rsx! { path { d: "{arc.to_svg_path(center)}", stroke: "#e67e22", stroke_width: "3", fill: "none" } }
                            } else { rsx! { "" } }
                        }
                    }

                    // رسم Along Line
                    for alg in alg_snapshot.iter() {
                        {
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == alg.p1_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == alg.p2_id);
                            if let (Some(p1), Some(p2)) = (p1, p2) {
                                let res = alg.calculate_point(p1, p2);
                                rsx! { g { line { x1: "{p1.x()}", y1: "{p1.y()}", x2: "{res.x}", y2: "{res.y}", stroke: "#95a5a6", stroke_width: "1", stroke_dasharray: "2,2" } circle { cx: "{res.x}", cy: "{res.y}", r: "6", fill: "#8e44ad" } } }
                            } else { rsx! { "" } }
                        }
                    }

                    for spline in spl_snapshot.iter() {
                        {
                            let sid = spline.metadata.id;
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p1_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p2_id);
                            let p3 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p3_id);
                            let p4 = pts_snapshot.iter().find(|p| p.metadata.id == spline.p4_id);
                            if let (Some(s), Some(c1), Some(c2), Some(e)) = (p1, p2, p3, p4) {
                                let d_path = format!("M {} {} C {} {}, {} {}, {} {}", s.x(), s.y(), c1.x(), c1.y(), c2.x(), c2.y(), e.x(), e.y());
                                rsx! { path { d: "{d_path}", stroke: "#2ecc71", stroke_width: "3", fill: "none", 
                                    onmousedown: move |evt| {
                                        evt.stop_propagation();
                                        let mut m_sig = mode;
                                        let current_m = m_sig.cloned();
                                        if let CanvasMode::MirrorSelection { mut selected_entities } = current_m {
                                            selected_entities.push(EntityRef::Spline(sid));
                                            m_sig.set(CanvasMode::MirrorSelection { selected_entities });
                                        }
                                    }
                                } }
                            } else { rsx! { "" } }
                        }
                    }

                    for line in lns_snapshot.iter() {
                        {
                            let lid = line.metadata.id;
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == line.start_point_id);
                            let p2 = pts_snapshot.iter().find(|p| p.metadata.id == line.end_point_id);
                            if let (Some(start), Some(end)) = (p1, p2) {
                                rsx! { line { x1: "{start.x()}", y1: "{start.y()}", x2: "{end.x()}", y2: "{end.y()}", stroke: "#3498db", stroke_width: "3",
                                    onmousedown: move |evt| {
                                        evt.stop_propagation();
                                        let mut m_sig = mode;
                                        let current_m = m_sig.cloned();
                                        if let CanvasMode::MirrorSelection { mut selected_entities } = current_m {
                                            selected_entities.push(EntityRef::Line(lid));
                                            m_sig.set(CanvasMode::MirrorSelection { selected_entities });
                                        }
                                    }
                                } }
                            } else { rsx! { "" } }
                        }
                    }

                    for bis in bis_snapshot.iter() {
                        {
                            let p1 = pts_snapshot.iter().find(|p| p.metadata.id == bis.p1_id);
                            let vertex = pts_snapshot.iter().find(|p| p.metadata.id == bis.vertex_id);
                            let p3 = pts_snapshot.iter().find(|p| p.metadata.id == bis.p3_id);
                            if let (Some(p1), Some(v), Some(p3)) = (p1, vertex, p3) {
                                let end = bis.calculate_end_point(p1, v, p3);
                                rsx! { line { x1: "{v.x()}", y1: "{v.y()}", x2: "{end.x}", y2: "{end.y}", stroke: "#9b59b6", stroke_width: "2", stroke_dasharray: "5,5" } }
                            } else { rsx! { "" } }
                        }
                    }

                    for p in points.read().iter() {
                        {
                            let pid = p.metadata.id;
                            let px = p.x();
                            let py = p.y();
                            let p_name = p.metadata.name.clone();
                            let is_active = match &current_mode {
                                CanvasMode::AwaitingLineEnd { first_point_id } => *first_point_id == pid,
                                CanvasMode::BezierControl1 { p1 } => *p1 == pid,
                                CanvasMode::BezierControl2 { p1, p2 } => *p1 == pid || *p2 == pid,
                                CanvasMode::BezierEnd { p1, p2, p3 } => *p1 == pid || *p2 == pid || *p3 == pid,
                                CanvasMode::BisectorVertex { p1 } => *p1 == pid,
                                CanvasMode::BisectorEnd { p1, vertex } => *p1 == pid || *vertex == pid,
                                CanvasMode::MirrorAxisEnd { axis_p1, .. } => *axis_p1 == pid,
                                CanvasMode::AlongLineEnd { p1 } => *p1 == pid,
                                CanvasMode::ArcRadius { center_id } => *center_id == pid,
                                _ => false
                            } || matches!(current_selection, SelectedItem::Point(id) if id == pid);
                            
                            let fill_color = if is_active { "#f1c40f" } 
                                             else if !matches!(current_mode, CanvasMode::PlacePoint) { "#e67e22" }
                                             else { "#e74c3c" };

                            rsx! {
                                g {
                                    key: "pt-group-{pid}",
                                    circle { 
                                        cx: "{px}", cy: "{py}", r: "10", fill: "{fill_color}",
                                        stroke: if is_active { "white" } else { "none" },
                                        stroke_width: "2", cursor: "pointer",
                                        onmousedown: move |evt| {
                                            evt.stop_propagation();
                                            let mut m_sig = mode;
                                            let mut sel_sig = selected_item;
                                            let mut drg_sig = dragging_point_id;
                                            let mut p_sig = points;
                                            let mut l_sig = lines;
                                            let mut s_sig = splines;
                                            let mut b_sig = bisectors;
                                            let mut a_sig = arcs;
                                            let mut al_sig = along_lines;
                                            let mut id_sig = next_id;
                                            let mut mir_sig = mirrors;

                                            let m = m_sig.cloned();
                                            match m {
                                                CanvasMode::PlacePoint => { drg_sig.set(Some(pid)); sel_sig.set(SelectedItem::Point(pid)); }
                                                CanvasMode::AwaitingLineStart => { m_sig.set(CanvasMode::AwaitingLineEnd { first_point_id: pid }); }
                                                CanvasMode::AwaitingLineEnd { first_point_id } => {
                                                    if first_point_id != pid {
                                                        let lid = *id_sig.read();
                                                        l_sig.write().push(VLine::new(lid, &format!("L{}", lid), first_point_id, pid));
                                                        id_sig.set(lid + 1);
                                                    }
                                                    m_sig.set(CanvasMode::AwaitingLineStart);
                                                }
                                                CanvasMode::BezierStart => { m_sig.set(CanvasMode::BezierControl1 { p1: pid }); }
                                                CanvasMode::BezierControl1 { p1 } => { m_sig.set(CanvasMode::BezierControl2 { p1, p2: pid }); }
                                                CanvasMode::BezierControl2 { p1, p2 } => { m_sig.set(CanvasMode::BezierEnd { p1, p2, p3: pid }); }
                                                CanvasMode::BezierEnd { p1, p2, p3 } => {
                                                    if pid != p1 && pid != p2 && pid != p3 {
                                                        let sid = *id_sig.read();
                                                        s_sig.write().push(VCubicBezier::new(sid, &format!("S{}", sid), p1, p2, p3, pid));
                                                        id_sig.set(sid + 1);
                                                    }
                                                    m_sig.set(CanvasMode::BezierStart);
                                                }
                                                CanvasMode::BisectorStart => { m_sig.set(CanvasMode::BisectorVertex { p1: pid }); }
                                                CanvasMode::BisectorVertex { p1 } => { m_sig.set(CanvasMode::BisectorEnd { p1, vertex: pid }); }
                                                CanvasMode::BisectorEnd { p1, vertex } => {
                                                    if pid != p1 && pid != vertex {
                                                        let bid = *id_sig.read();
                                                        b_sig.write().push(VBisector::new(bid, &format!("B{}", bid), p1, vertex, pid, 150.0));
                                                        id_sig.set(bid + 1);
                                                    }
                                                    m_sig.set(CanvasMode::BisectorStart);
                                                }
                                                CanvasMode::AlongLineStart => { m_sig.set(CanvasMode::AlongLineEnd { p1: pid }); }
                                                CanvasMode::AlongLineEnd { p1 } => {
                                                    if p1 != pid {
                                                        let aid = *id_sig.read();
                                                        al_sig.write().push(VPointAlongLine::new(aid, &format!("AL{}", aid), p1, pid, 100.0));
                                                        id_sig.set(aid + 1);
                                                    }
                                                    m_sig.set(CanvasMode::AlongLineStart);
                                                }
                                                CanvasMode::ArcCenter => { m_sig.set(CanvasMode::ArcRadius { center_id: pid }); }
                                                CanvasMode::ArcRadius { center_id } => {
                                                    if center_id != pid {
                                                        let aid = *id_sig.read();
                                                        let center_p = p_sig.read().iter().find(|p| p.metadata.id == center_id).cloned().unwrap();
                                                        let radius = center_p.coords.distance_to(&p_sig.read().iter().find(|pt| pt.metadata.id == pid).unwrap().coords);
                                                        let angle = center_p.coords.angle_to(&p_sig.read().iter().find(|pt| pt.metadata.id == pid).unwrap().coords);
                                                        a_sig.write().push(object::VArc::new(aid, &format!("A{}", aid), center_id, radius, angle, angle + 90.0));
                                                        id_sig.set(aid + 1);
                                                    }
                                                    m_sig.set(CanvasMode::ArcCenter);
                                                }
                                                CanvasMode::MirrorSelection { mut selected_entities } => { selected_entities.push(EntityRef::Point(pid)); m_sig.set(CanvasMode::MirrorSelection { selected_entities }); }
                                                CanvasMode::MirrorAxisStart { selected_entities } => { m_sig.set(CanvasMode::MirrorAxisEnd { selected_entities, axis_p1: pid }); }
                                                CanvasMode::MirrorAxisEnd { selected_entities, axis_p1 } => {
                                                    if axis_p1 != pid {
                                                        let mid = *id_sig.read();
                                                        mir_sig.write().push(VMirror::new(mid, "M1", selected_entities, axis_p1, pid));
                                                        id_sig.set(mid + 1);
                                                    }
                                                    m_sig.set(CanvasMode::PlacePoint);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    text { x: "{px + 15.0}", y: "{py - 15.0}", fill: "#2c3e50", font_size: "18", pointer_events: "none", "{p_name}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
