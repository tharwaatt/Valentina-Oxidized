# Valentina-Oxidized: Project Context (Stage 3 - Relationships)

## 🌟 Vision
A modern, Rust-based CAD tool for fashion pattern making. Built for performance and safety using the Dioxus framework and SVG rendering.

## 🛠 Tech Stack (Established)
- **Rust Stable**: Core logic (edition 2024).
- **Dioxus 0.7.3**: Desktop GUI with SVG support.
- **Serde & serde_json**: Serialization for project persistence.
- **RFD (Rusty File Dialogs)**: Native OS file dialog integration.
- **SVG**: For vector-perfect geometric drawing.
- **Target OS**: Linux (Ubuntu environment).

## 📍 Roadmap Evolution: Stages
*Detailed track available in [ROADMAP.md](../ROADMAP.md)*

- **Stage 1 (Foundation)**: Mathematical primitives (Point2D). [Status: DONE]
- **Stage 2 (Canvas)**: Dioxus Integration & Interactive Point creation. [Status: DONE]
- **Stage 3 (Relationships)**: Connecting entities via Lines and Curves. [Status: IN_PROGRESS]
  - Task 3.1 - 3.9: Completed (Lines, Splines, Selection, Labels, Delete, Save/Load, Drag, Bisector, Paths).
  - Task 3.10: Dynamic Measurement Overlays. (Current)
  - Milestone 3 Goal: Stable Geometric Graph Editor.

## 🛑 Lessons Learned (Memory for Agent)
- **Entity Referencing**: To group objects without duplication, use an `EntityRef` enum. This maintains a single source of truth for geometry.
- **Visual Grouping**: In CAD, increasing stroke-width and changing color is an effective way to show that entities are part of a larger "Path" or "Contour".
- **Reactive Drag & Drop**: Using ID-based linking allows for seamless dragging; updating a point's coordinates automatically re-renders all dependent lines/splines/paths.

## 🎯 Next Objective: Live Measurements & UI Polish
1. Dynamic Overlays: Real-time SVG labels showing lengths and angles next to active lines and segments.
2. Finalizing Stage 3: Ensuring the geometric graph is robust enough for pattern piece generation.
