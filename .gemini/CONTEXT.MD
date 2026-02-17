# Valentina-Oxidized: Project Context (Ready for Day 5)

## 🌟 Vision
A modern, Rust-based CAD tool for fashion pattern making. Built for performance and safety using the Dioxus framework and SVG rendering.

## 🛠 Tech Stack (Established)
- **Rust Stable**: Core logic.
- **Dioxus 0.6+**: Desktop GUI with SVG support.
- **SVG**: For vector-perfect geometric drawing.
- **Target OS**: Linux (Ubuntu/Debian environment).

## 📅 Roadmap: Completed Milestones

### [Day 1] Foundation
- Project scaffolding and dependency setup.

### [Day 2] Geometry Engine (`src/geometry.rs`)
- Implemented `Point2D` with mathematical methods (e.g., `rotate`).
- Ensured coordinate logic is separate from UI.

### [Day 3] Identity & Objects (`src/object.rs`)
- Created `VGObject` for metadata (ID, String Name).
- Created `VPoint` (Composition of Identity + Geometry).
- Established the "Ownership" principle (using `String` for metadata).

### [Day 4] Interactive Canvas (CURRENT STATE)
- **GUI Boot**: Successfully launched Dioxus Desktop.
- **System Fixes**: Resolved Linux linker issues (`libxdo-dev`, `libwebkit2gtk-4.1-dev`).
- **Interactive SVG**:
    - Implemented a coordinate grid (50px pattern).
    - Mouse Click Logic: Adding `VPoint` objects to a central `Vec<VPoint>` signal on click.
    - Dynamic Rendering: SVG circles and text labels update automatically via Dioxus Signals.

## 🛑 Lessons Learned (Memory for Agent)
- **No Raw References**: We use IDs (u32) to link objects to avoid Borrow Checker hell.
- **Snake Case**: Dioxus SVG attributes must be `snake_case` (e.g., `stroke_width`).
- **Event Handling**: Use `stop_propagation()` to prevent event bubbling from points to the background canvas.

## 🎯 Next Objective: Day 5 - Relationships (Lines)
The project is now a "point cloud" generator. Day 5 will focus on:
1. Defining the `VLine` struct in `object.rs`.
2. Implementing logic to select two points and connect them.
3. Calculating line properties (length, angle) from the mathematical model.
