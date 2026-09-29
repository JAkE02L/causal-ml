# Rust Environment — Causal Inference & Machine Learning

This subfolder contains the **Rust** implementations of causal inference and machine learning algorithms and experiments, designed as an exact mirror of the Python implementations.

---

## 1. Prerequisites & Toolchain Installation

Rust is officially managed via `rustup`, the installer and toolchain manager for Rust.

### On Windows (Recommended)

1. **Prerequisite (C++ Build Tools):**
   Rust on Windows uses the MSVC ABI by default, which requires Microsoft C++ build tools.
   - Download and install **Visual Studio Build Tools**: [https://visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - During installation, check the workload: **"Desktop development with C++"**.

2. **Rust Installer (`rustup`):**
   - Download `rustup-init.exe` from [https://rustup.rs/](https://rustup.rs/).
   - Run the installer and choose the default option `1) Proceed with installation (default)`.
   - Restart your terminal (PowerShell or CMD) for environment variables to take effect.

3. **Alternative via PowerShell (Winget):**
   ```powershell
   winget install --id Rustlang.Rustup
   ```

### On Linux & macOS

Run in your terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Follow the on-screen prompts and reload your shell (`source $HOME/.cargo/env`).

---

## 2. Verifying the Installation

Verify that the compiler and package manager are available:

```bash
rustc --version
cargo --version
```

### Updating to the Latest Stable Version

To keep Rust updated to the latest stable release:

```bash
rustup update stable
rustup default stable
```

---

## 3. Workflow with Cargo

Execute all commands inside the `rust/` directory:

- **Build the project:**
  ```bash
  cargo build
  ```
- **Run the main binary:**
  ```bash
  cargo run
  ```
- **Fast type and syntax checking (without binary generation):**
  ```bash
  cargo check
  ```
- **Run unit / integration tests:**
  ```bash
  cargo test
  ```
- **Linter and static analysis:**
  ```bash
  cargo clippy
  ```
- **Format code:**
  ```bash
  cargo fmt
  ```

---

## 4. Dependency Policy

To keep the project clean, lightweight, and reproducible, **no external crates should be added to `Cargo.toml`** unless explicitly requested.
When a new crate is needed and authorized, add it via:
```bash
cargo add <crate-name>
```
