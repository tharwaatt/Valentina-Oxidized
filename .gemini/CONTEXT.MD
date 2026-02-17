# Valentina-Oxidized: Project Context (Stage 3 - Relationships)

## 🌟 Vision
A modern, Rust-based CAD tool for fashion pattern making. Built for performance and safety using the Dioxus framework and SVG rendering.

## 🛠 Tech Stack (Established)
- **Rust Stable**: Core logic (edition 2024).
- **Dioxus 0.7.3**: Desktop GUI with SVG support and `document` features.
- **Serde & serde_json**: Serialization for project persistence.
- **RFD (Rusty File Dialogs)**: Native OS file dialog integration.
- **SVG**: For vector-perfect geometric drawing.
- **Target OS**: Linux (Ubuntu environment).

## 📍 Roadmap Evolution: Stages
*Detailed track available in [ROADMAP.md](../ROADMAP.md)*

- **Stage 1 (Foundation)**: Mathematical primitives (Point2D). [Status: DONE]
- **Stage 2 (Canvas)**: Dioxus Integration & Interactive Point creation. [Status: DONE]
- **Stage 3 (Relationships)**: Connecting entities via Lines and Curves. [Status: IN_PROGRESS]
  - Task 3.1 - 3.8: Completed (Lines, Splines, Selection, Labels, Delete, Save/Load, Drag, Bisector).
  - Task 3.9: Path Consolidation. (Current)
  - Milestone 3 Goal: Stable Geometric Graph Editor.

## 🛑 Lessons Learned (Memory for Agent)
- **Coordinate Normalization**: SVG `viewBox` requires a `CoordMapper` to translate CSS pixels to user-space units accurately.
- **Dioxus 0.7 Signals**: Use `Snapshotting` (clone before render) to avoid deadlocks when mixing read and write access in the UI loop.
- **Reactive Drag & Drop**: Using ID-based linking allows for seamless dragging; updating a point's coordinates automatically re-renders all dependent lines/splines.
- **Advanced Geometry**: Porting original C++ logic (like BisectorAngle) requires careful handling of polar coordinate systems and angle normalization (0-360).

## 🎯 Next Objective: Shape Logic & Measurements
1. Path Consolidation: Logic to group lines and splines into a single "Path" entity for pattern pieces.
2. Dynamic Overlays: Real-time SVG labels showing lengths and angles next to active lines.
