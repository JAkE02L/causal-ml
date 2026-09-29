# Agent Rules & Guidelines (`AGENTS.md`)

This file defines the operational guidelines, architecture constraints, and rules that any AI assistant or agent (including Antigravity) must strictly follow when working in this repository.

---

## 1. Core Principle: 1:1 Parity (Python ↔ Rust)

- **Implementation Symmetry:** Every concept, algorithm, causal estimator, or Machine Learning model developed in `python/` must have an equivalent counterpart in `rust/`, and vice versa.
- **Mirrored Structure:** The organization of submodules, function names, and test suites must be analogous, adhering to the idiomatic conventions of each language.
- **Numerical Validation:** Whenever feasible, numerical results from both environments should be validated against identical synthetic or benchmark datasets to ensure consistent causal estimates (ATE, CATE, ITE, propensity scores, etc.).

---

## 2. Language Policy

- **Strict English in Repository:** All files, code comments, documentation (READMEs, design docs), docstrings, type annotations, and Git commit messages within this repository must be written in **English**, regardless of the conversation language used with the user.

---

## 3. Strict Dependency Policy

- **ZERO Anticipated Dependencies:** **NEVER** install or add libraries, crates, or packages unless the user explicitly and directly requests it.
- **Current Python Environment:**
  - Only `numpy` is permitted for now.
  - Adding a dependency (prior approval required): `uv add <package>`.
- **Current Rust Environment:**
  - Only the standard library (`std`) is permitted for now.
  - Adding a crate (prior approval required): `cargo add <crate>`.

---

## 4. Execution Environment Rules

### Python
- The only authorized package and environment manager is **`uv`**. Never execute raw `pip`, `conda`, `virtualenv`, or `poetry` commands.
- Run any Python script or tool using:
  ```bash
  uv run python <path-to-script>
  ```
- Keep `uv.lock` up-to-date and tracked under version control.

### Rust
- All code must compile against the **latest stable** Rust toolchain.
- Use standard Cargo commands:
  - `cargo check`: Fast syntax/type check.
  - `cargo run`: Run binaries / examples.
  - `cargo test`: Run tests.
  - `cargo fmt --check`: Check formatting.
  - `cargo clippy`: Static linter.

---

## 5. Suggested Rules, Hooks, and Future Workflows

As the repository matures, consider the following configurations:

### A. Git Pre-commit Hooks (Via `.git/hooks/pre-commit` or `pre-commit`)
A hook to ensure both environments remain healthy, linted, and formatted before committing:
1. **Python:** Fast linting and format checking (e.g., `uv run ruff check` and `uv run ruff format --check` once approved).
2. **Rust:** `cargo fmt --check` and `cargo clippy -- -D warnings`.
3. **Reject commits if either environment fails compilation or tests.**

### B. CI / GitHub Actions
A workflow in `.github/workflows/ci.yml` running parallel matrix jobs:
- **Python Job:** `uv sync` + `uv run pytest`.
- **Rust Job:** `cargo test` + `cargo clippy`.

### C. Numerical Parity Testing
Generate canonical benchmark datasets (in CSV or JSON format) consumed by both Rust and Python tests to verify that causal estimates match within a numerical tolerance of $\epsilon < 10^{-6}$.
