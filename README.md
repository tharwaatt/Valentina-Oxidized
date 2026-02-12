# Valentina-Oxidized 🦀📐

**Learning Rust by re-engineering the Valentina CAD engine.**

---

## 🌟 Overview
**Valentina-Oxidized** is an ambitious project to migrate and reimagine the core engine of [Valentina](https://gitlab.com/smart-pattern/valentina) from C++ to Rust. Focused on safety, performance, and modern software design.

---

## 🏗️ The Architectural Approach
1. **Deconstruction:** Analyzing C++/Qt memory layout.
2. **Mapping:** Finding Idiomatic Rust equivalents (Traits/Composition).
3. **Refactoring:** Leveraging Rust’s Memory Safety.

| C++ Concept | Rust Equivalent | Architectural Reasoning |
| --- | --- | --- |
| **Class Inheritance** | **Composition** | Decoupling behavior from state for better maintainability. |
| **Qt Framework** | **Dioxus + WGPU** | Modern, lightweight, and cross-platform (Web/Desktop). |

---

## 📅 The Learning Log
* **[Day 1](./lessons/01-deconstruction.md):** Analyzing `VPointF` - Pimpl and memory sharing.
* **[Day 2](./lessons/02-foundation.md):** Foundation - Cargo and `Point2D` math engine.
* **[Day 3](./lessons/03-identity.md):** The Core Identity - Implementing `VGObject` and Composition.

---

## 🗺️ Roadmap
* [x] **Phase 1: Geometry Foundation (`vgeometry`)**
    * [x] Point2D Struct & Rotation Logic.
* [ ] **Phase 2: The Core Identity (`vcore`)**
    * [x] Object ID and Metadata management.
    * [ ] Implementing the `VGObject` trait for shared behavior.
* [ ] **Phase 4: Modern UI (Dioxus Integration)**
    * [ ] Setup Dioxus Desktop with WGPU Canvas.

---

## 🛠️ Development Setup

```bash
# Clone the repository
git clone [https://github.com/tharwaatt/valentina-oxidized](https://github.com/tharwaatt/valentina-oxidized)

# Build the project
cd valentina-oxidized
cargo build

# Run the project
cargo run

# Run tests
cargo test
```