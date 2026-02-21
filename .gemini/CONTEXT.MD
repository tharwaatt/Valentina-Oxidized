# Valentina-Oxidized: Project Context (Stage 4 - Advanced CAD)

## 🌟 Vision
Building a constraint-based CAD engine. Every element is reactive and mathematically linked to its dependencies.

## 📍 Roadmap Evolution: Stages
*Detailed track available in [ROADMAP.md](../docs/ROADMAP.md)*

- **Stage 1-3**: Core primitives, relationships, and basic interaction. [Status: DONE]
- **Stage 4 (Advanced Operations)**: Constraints and complex tools. [Status: IN_PROGRESS]
  - Task 4.1: Linked Mirroring. (Completed)
  - Task 4.2: Point Along Line. (Completed)
  - Task 4.3: Precision Overhaul (JS Transformation). (Completed)
  - Task 4.4: Intersection Tools. (Next)

## 🛑 Lessons Learned (Memory for Agent)
- **Coordinate Precision**: Never trust CSS pixel calculations for CAD. Always use `svg.getScreenCTM().inverse()` via JavaScript to map mouse to SVG user-space.
- **Dioxus 0.7.3 Eval**: Use `document::eval` and capture signals by value within `spawn` to avoid lifetime and borrow-checker hell in event handlers.
- **DoF (Degrees of Freedom)**: Mirrored points and calculated points have zero DoF; they are derived from their parents. UI should prevent dragging them directly.

## 🎯 Next Objective: Geometric Logic Expansion
1. Point of Intersection: Porting Valentina's logic to find the crossing point of two lines.
2. interactive Arcs: Enhancing the Arc tool to support dynamic radius and angle editing.
3. Command Pattern: Design a way to create entities via text-based instructions.
