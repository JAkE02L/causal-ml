# Causal Inference & Machine Learning: Python & Rust Lab

Personal study and research repository dedicated to **Causal Inference** and **Machine Learning**.

The primary objective of this repository is to explore, implement, and benchmark causal inference and machine learning algorithms while maintaining **exact 1:1 parity** across two high-level ecosystems: **Python** and **Rust**.

---

## Repository Structure

```text
causal-ml/
├── .gitignore          # Ignore rules for Python (.venv, cache) and Rust (target/)
├── README.md           # Repository overview and quick start
├── AGENTS.md           # Operational guidelines and architecture rules for AI agents
├── python/             # Python environment (managed exclusively via uv)
│   ├── pyproject.toml  # Project configuration and dependencies
│   ├── uv.lock         # Fully reproducible lockfile
│   ├── README.md       # Python setup and workflow guide
│   └── src/            # Python source code and implementations
└── rust/               # Rust environment (managed via Cargo)
    ├── Cargo.toml      # Cargo package manifest and dependencies
    ├── Cargo.lock      # Fully reproducible lockfile
    ├── README.md       # Rust setup, toolchain installation, and workflow guide
    └── src/            # Rust source code and implementations
```

---

## Design Principle: 1:1 Parity

Every conceptual module, algorithm, or experiment in this repository is mirrored across both languages:

1. **Python (`python/`):** Focused on rapid prototyping, scientific flexibility, and statistical benchmarking, using `uv` as the sole package and environment manager.
2. **Rust (`rust/`):** Focused on low-level performance, memory safety, concurrency, and strongly-typed guarantees, using the latest stable Rust toolchain.

---

## Quick Start & Environment Setup

- For instructions on setting up and reproducing the **Python** environment, see [`python/README.md`](python/README.md).
- For step-by-step instructions on installing and configuring the **Rust** toolchain on Windows, Linux, or macOS, see [`rust/README.md`](rust/README.md).
