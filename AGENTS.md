# Reglas y Directrices para Agentes de IA (`AGENTS.md`)

Este archivo contiene las directrices operativas, restricciones y pautas de arquitectura que cualquier asistente o agente de IA (incluyendo Antigravity) debe seguir rigurosamente al trabajar en este repositorio.

---

## 1. Principio Fundamental: Paridad 1:1 (Python ↔ Rust)

- **Simetría de implementaciones:** Cada concepto, algoritmo, estimador causal o modelo de Machine Learning que se desarrolle en `python/` debe tener su contraparte equivalente en `rust/`, y viceversa.
- **Estructura especular:** La organización de submódulos, nombres de funciones y pruebas debe ser análoga en la medida de lo idiomático de cada lenguaje.
- **Validación numérica:** Cuando sea factible, los resultados numéricos de ambos entornos deben compararse con conjuntos de datos sintéticos o de referencia idénticos para garantizar coherencia en estimaciones causales (ATE, CATE, ITE, propensity scores, etc.).

---

## 2. Política Estricta de Dependencias

- **CERO dependencias anticipadas:** **NUNCA** agregues librerías, crates o paquetes a menos que el usuario lo solicite de manera explícita y directa.
- **Entorno Python actual:**
  - Solo `numpy` está permitido por ahora.
  - Para agregar una dependencia (previa aprobación): `uv add <paquete>`.
- **Entorno Rust actual:**
  - Solo la biblioteca estándar (`std`) está permitida por ahora.
  - Para agregar un crate (previa aprobación): `cargo add <crate>`.

---

## 3. Manejo de Entornos de Ejecución

### Python
- El único gestor autorizado es **`uv`**. No utilices comandos directos de `pip`, `conda`, `virtualenv` o `poetry`.
- Para ejecutar cualquier script o herramienta en Python:
  ```bash
  uv run python <ruta-al-script>
  ```
- Mantener `uv.lock` actualizado y bajo control de versiones.

### Rust
- Todo el código debe compilar con la toolchain **estable más reciente** de Rust.
- Utilizar los comandos estándar de Cargo:
  - `cargo check`: Verificación rápida.
  - `cargo run`: Ejecución de binarios / ejemplos.
  - `cargo test`: Ejecución de pruebas.
  - `cargo fmt --check`: Verificación de formato.
  - `cargo clippy`: Linter estático.

---

## 4. Sugerencias de Reglas, Hooks y Mejoras Futuras

Conforme el repositorio crezca, se recomienda considerar las siguientes configuraciones:

### A. Git Pre-commit Hooks (Vía `.git/hooks/pre-commit` o `pre-commit`)
Un hook para verificar antes de cada commit que ambos ambientes se mantienen sanos y formateados:
1. **Python:** Formateo y linting rápido (ej. `uv run ruff check` y `uv run ruff format --check` cuando se autorice ruff).
2. **Rust:** `cargo fmt --check` y `cargo clippy -- -D warnings`.
3. **No git commit si alguno de los dos ambientes falla la compilación o tests.**

### B. CI / GitHub Actions
Un flujo en `.github/workflows/ci.yml` con dos matrices de jobs paralelas:
- **Job Python:** `uv sync` + `uv run pytest`.
- **Job Rust:** `cargo test` + `cargo clippy`.

### C. Paridad Numérica en Pruebas
Generar archivos de datos canónicos (en formato CSV o JSON) para que los tests de Rust y Python consuman exactamente los mismos datos de entrada y validen que las estimaciones causales coincidan con una tolerancia $\epsilon < 10^{-6}$.
