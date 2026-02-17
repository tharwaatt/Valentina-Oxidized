# Valentina-Oxidized: Project Context (Stage 3 - Relationships)

## 🌟 Vision
A modern, Rust-based CAD tool for fashion pattern making. Built for performance and safety using the Dioxus framework and SVG rendering.

## 🛠 Tech Stack (Established)
- **Rust Stable**: Core logic (edition 2024).
- **Dioxus 0.7.3**: Desktop GUI with SVG support and `document` features.
- **SVG**: For vector-perfect geometric drawing.
- **Target OS**: Linux (Ubuntu environment).

## 📍 Roadmap Evolution: Stages
- **Stage 1 (Foundation)**: Mathematical primitives (Point2D). [Status: DONE]
- **Stage 2 (Canvas)**: Dioxus Integration & Interactive Point creation. [Status: DONE]
- **Stage 3 (Relationships)**: Connecting entities via Lines and Curves. [Status: IN_PROGRESS]
  - Task 3.1: Define VLine & ID-based linking. (Completed)
  - Task 3.2: Implement Cubic Bezier Splines. (Completed)
  - Task 3.3: Visual selection highlight & Global Selection System. (Completed)
  - Task 3.4: SVG Text Labels for points. (Completed)
  - Task 3.5: Cascading Deletion logic. (Current)

## 🛑 Lessons Learned (Memory for Agent)
- **Coordinate Normalization**: SVG `viewBox` requires a `CoordMapper` to translate CSS pixels to user-space units accurately.
- **Dioxus 0.7 Signals**: Use `Snapshotting` (clone before render) to avoid deadlocks when mixing read and write access in the UI loop.
- **Event Delegation**: `stop_propagation()` is not always reliable in Dioxus Desktop; a state-machine (`CanvasMode`) is the robust alternative.

## 🎯 Next Objective: Selection & Visual Feedback
With lines and splines functional, we must improve the UI:
1. CSS classes for "Selected" and "Hover" states on SVG elements.
2. SVG text labels (P1, P2, etc.) that float near points.
3. Deletion logic: what happens to a line if its start point is deleted? (Dependency tracking).
