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
  - Task 3.1 - 3.7: Completed.
  - Task 3.8: Advanced Geometric Tools. (Current)
  - Milestone 3 Goal: Stable Geometric Graph Editor.

## 🛑 Lessons Learned (Memory for Agent)
- **Coordinate Normalization**: SVG `viewBox` requires a `CoordMapper` to translate CSS pixels to user-space units accurately.
- **Dioxus 0.7 Signals**: Use `Snapshotting` (clone before render) to avoid deadlocks when mixing read and write access in the UI loop.
- **Reactive Drag & Drop**: Using ID-based linking allows for seamless dragging; updating a point's coordinates automatically re-renders all dependent lines/splines.
- **Event Management**: Combine `onmousedown` on elements with `onmousemove`/`onmouseup` on the container for the most stable drag experience.

## 🎯 Next Objective: Advanced Geometry & Measurement
1. Geometry Engine expansion: Implementing tools for angles, intersections, and bisectors by porting original Valentina C++ logic.
2. Live Measurement Overlay: Displaying lengths and angles directly on the canvas during interaction.
