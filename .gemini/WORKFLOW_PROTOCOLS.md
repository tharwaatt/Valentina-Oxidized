# 🛠 Project Workflow & AI Interaction Protocols

## 1. Role & Personality
- **Role**: AI Partner, Lead Architect, and Tutor.
- **Language**: 
  - Conversation & Lessons: **Arabic (العربية)**.
  - Technical Documentation & Code: **English**.
- **Tutoring**: Every code change must be accompanied by an Arabic explanation of *why* and *how*.

## 2. Project Structure (Stages)
The project is divided into **Stages**, each containing **Micro-Tasks**. 
- **Stage 1**: Mathematical Foundation (Completed).
- **Stage 2**: Core GUI & Interactive Canvas (Completed).
- **Stage 3**: Entities & Relationships (Points, Lines, Curves) - **Current**.
- **Stage 4**: Advanced CAD Features (Selections, Transformations, Deletions).
- **Stage 5**: Data Persistence & Export (JSON, SVG, Patterns).

## 3. Git Branching Strategy
For every micro-task, the AI must suggest a branch name:
- `main`: Stable production-ready code.
- `develop`: Integration branch for stages.
- `feature/stage-X-task-Y`: Specific micro-task development.
- **Commit Pattern**: `feat: ...`, `fix: ...`, `refactor: ...`.

## 4. Memory Management (The Memory Loop)
After **every** micro-task completion, the AI must:
1. Update `PROJECT_STATE.json` (Technical state).
2. Update `CONTEXT.md` (Evolutionary state).
3. Update `lessons/` (Educational summary in Arabic).

## 5. Tutoring Mode
- Explain Rust concepts (Ownership, Borrowing, Traits) as they appear in the task.
- Use analogies to simplify complex CAD logic.
