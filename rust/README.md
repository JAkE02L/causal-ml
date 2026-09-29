# Ambiente de Rust — Inferencia Causal & Machine Learning

Este subdirectorio contiene las implementaciones en **Rust** de los conceptos, algoritmos y experimentos de inferencia causal y aprendizaje automático, diseñados como espejo exacto de las implementaciones en Python.

---

## 1. Requisitos e Instalación de Rust

Rust se gestiona oficialmente a través de `rustup`, el instalador y gestor de toolchains de Rust.

### En Windows (Recomendado)

1. **Requisito previo (C++ Build Tools):**
   Rust en Windows utiliza por defecto la ABI MSVC, la cual requiere las herramientas de compilación de Microsoft C++.
   - Descarga e instala **Visual Studio Build Tools**: [https://visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - Durante la instalación, marca la carga de trabajo: **"Desarrollo para el escritorio con C++"** (*Desktop development with C++*).

2. **Instalador de Rust (`rustup`):**
   - Descarga `rustup-init.exe` desde [https://rustup.rs/](https://rustup.rs/).
   - Ejecuta el instalador y selecciona la opción por defecto `1) Proceed with installation (default)`.
   - Reinicia tu terminal (PowerShell o CMD) para que las variables de entorno se actualicen.

3. **Alternativa vía PowerShell (Winget):**
   ```powershell
   winget install --id Rustlang.Rustup
   ```

### En Linux y macOS

Ejecuta en tu terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Sigue las instrucciones en pantalla y recarga tu shell (`source $HOME/.cargo/env`).

---

## 2. Verificación de la Instalación

Comprueba que el compilador y el gestor de paquetes estén disponibles:

```bash
rustc --version
cargo --version
```

### Actualización a la versión estable más reciente

Para mantener Rust actualizado con la última versión estable:

```bash
rustup update stable
rustup default stable
```

---

## 3. Uso y Flujo de Trabajo con Cargo

Todos los comandos deben ejecutarse dentro de la carpeta `rust/`:

- **Compilar el proyecto:**
  ```bash
  cargo build
  ```
- **Ejecutar el programa principal:**
  ```bash
  cargo run
  ```
- **Verificación rápida de sintaxis y tipos (sin generar binario):**
  ```bash
  cargo check
  ```
- **Ejecutar pruebas unitarias / de integración:**
  ```bash
  cargo test
  ```
- **Linter y análisis estático:**
  ```bash
  cargo clippy
  ```
- **Formatear el código:**
  ```bash
  cargo fmt
  ```

---

## 4. Política de Dependencias

Para mantener el entorno reproducible y minimalista, **no se deben añadir crates adicionales a `Cargo.toml`** a menos que sea explícitamente solicitado.
Cuando se requiera un nuevo crate, agrégalo usando:
```bash
cargo add <nombre-del-crate>
```
