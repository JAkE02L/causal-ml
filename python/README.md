# Python Environment — Causal Inference & Machine Learning

This subfolder contains the **Python** implementations of causal inference and machine learning algorithms and experiments, managed exclusively with `uv` for dependency management and environment reproducibility.

---

## 1. Prerequisites & Tooling

- **Package and Environment Manager:** `uv` (exclusive).
- **Python Version:** Python >= 3.14 (automatically downloaded and managed by `uv`).
- **Lockfile:** `uv.lock` guarantees 100% reproducible and deterministic installs across machines.

If you do not have `uv` installed on another machine:
```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```
Or on Linux/macOS:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

---

## 2. Setup & Reproducibility

The virtual environment is synced directly from `uv.lock`.

### Sync the exact virtual environment:
```bash
uv sync
```
This creates or updates `.venv` matching the locked dependencies without altering resolved versions.

---

## 3. Workflow

Execute all commands inside the `python/` directory (or specify `--directory python`):

- **Run a script inside the virtual environment:**
  ```bash
  uv run python src/causal_ml/__init__.py
  ```
- **Open an interactive Python REPL:**
  ```bash
  uv run python
  ```
- **Add a new dependency (strictly upon explicit request):**
  ```bash
  uv add <package-name>
  ```
- **Remove a dependency:**
  ```bash
  uv remove <package-name>
  ```

---

## 4. Current Dependencies

- `numpy` (>= 2.5.3)
