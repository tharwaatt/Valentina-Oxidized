# Valentina-Oxidized: Project Context (Ready for Day 5)

## 🌟 Vision
A modern, Rust-based CAD tool for fashion pattern making. Built for performance and safety using the Dioxus framework and SVG rendering.

## 🛠 Tech Stack (Established)
- **Rust Stable**: Core logic.
- **Dioxus 0.7+**: Desktop GUI with SVG support.
- **SVG**: For vector-perfect geometric drawing.
- **Target OS**: Linux (Ubuntu environment).

## 📍 Roadmap Evolution: Stages
- **Stage 1 (Foundation)**: Mathematical primitives (Point2D). [Status: DONE]
- **Stage 2 (Canvas)**: Dioxus Integration & Interactive Point creation. [Status: DONE]
- **Stage 3 (Relationships)**: Connecting entities via Lines and Angles. [Status: IN_PROGRESS]
  - Task 3.1: Define VLine & ID-based linking. (Completed)
  - Task 3.2: Implement Length/Angle calculations. (Completed)
  - Task 3.3: Visual feedback for selected points. (Completed)

## 🛑 Lessons Learned (Memory for Agent)
- **ID-Based Ownership**: In CAD tools, decoupling geometry from topology via IDs is essential for the Rust borrow checker.
- **Math on the Fly**: Calculating properties (length, angle) in the UI loop should pass point references to the line methods.
- **Dioxus Interaction**: Selection modes are best managed via signals representing "optional IDs".

## 🎯 Next Objective: Day 6 - Interaction & Deletion
With lines in place, the next step is:
1. Enabling "Delete" for points and lines (and handling the fallout).
2. SVG text labels to identify points (P1, P2) in the canvas.
3. Adding CSS transitions for a smoother CAD experience.
