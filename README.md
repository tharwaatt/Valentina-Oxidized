# Valentina-Oxidized 🦀📐

**Learning Rust by re-engineering the Valentina CAD engine.**

---

## 🌟 Overview

**Valentina-Oxidized** is an ambitious project to migrate and reimagine the core engine of [Valentina](https://gitlab.com/smart-pattern/valentina), an open-source pattern-making software, from C++ to Rust.

This is not a mere line-by-line translation. It is an **architectural migration** focused on safety, performance, and modern software design. By rebuilding a complex CAD system, I am applying the **Feynman Technique**: teaching through documentation to master the intricacies of both systems engineering and the Rust programming language.

---

## 🏗️ The Architectural Approach: "The Peer-Architect"

1. **Deconstruction:** Analyzing the original C++/Qt implementation (Memory layout, Object lifespans, and Design patterns).
2. **Mapping:** Finding the "Idiomatic Rust" equivalent (e.g., transforming Inheritance into Traits/Composition).
3. **Refactoring:** Implementing the logic while leveraging Rust’s "Fearless Concurrency" and "Memory Safety."

### Technical Mapping Table

| C++ Concept (Valentina) | Rust Equivalent (Oxidized) | Architectural Reasoning |
| --- | --- | --- |
| **Pimpl / Implicit Sharing** | **Explicit Ownership / Arc / Box** | Moving from hidden magic to explicit, compile-time safety. |
| **Class Inheritance** | **Traits & Composition** | Decoupling behavior from state for better maintainability. |
| **Qt Framework (QString/QPointF)** | **Standard Rust / Specialized Crates** | Reducing heavy dependencies in favor of lightweight, native types. |

---

## 📅 The Learning Log

Each stage of development is documented as a "lesson." You can find detailed architectural breakdowns in the [`/docs/analysis`] folder.

* **Day 1:** [Analyzing `VPointF] - Understanding Pimpl and memory sharing in C++.

---

## 🗺️ Roadmap

* [ ] **Phase 1: Geometry Foundation (`vgeometry`)**
* [ ] Points, Vectors, and Basic Math Utils.
* [ ] Geometric Transformations (Rotate, Flip, Move).


* [ ] **Phase 2: The Core Identity (`vcore`)**
* [ ] Implementing the `VGObject` trait.
* [ ] Object ID and Metadata management.


* [ ] **Phase 3: The Parametric Heart**
* [ ] Porting the calculation engine and formula parsing.


* [ ] **Phase 4: Data Interoperability**
* [ ] XML parsing for `.val` and `.vit` files using `Serde`.



---

## 🛠️ Development Setup

```bash
# Clone the repository
git clone https://github.com/tharwaatt/valentina-oxidized

# Build the project
cd valentina-oxidized
cargo build

# Run tests
cargo test

```

---

## 🤝 Contribution & Peer Review

This project is as much about **community learning** as it is about code. If you are a Rustacean, a C++ veteran, or a CAD enthusiast, your "Peer Review" is highly welcome! Open an issue or a PR to discuss the architecture.

---
