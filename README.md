# Causal Inference & Machine Learning: Python & Rust Lab

Repositorio de estudio e investigación personal sobre **Inferencia Causal** y **Machine Learning**. 

El objetivo principal de este repositorio es explorar, implementar y contrastar algoritmos y modelos de inferencia causal y aprendizaje automático manteniendo **paridad exacta 1:1** en dos ecosistemas de alto nivel: **Python** y **Rust**.

---

## Estructura del Repositorio

```text
causal-ml/
├── .gitignore          # Reglas de exclusión para Python (.venv, cache) y Rust (target/)
├── README.md           # Visión general del repositorio
├── AGENTS.md           # Reglas operativas, directrices y pautas para asistentes IA
├── python/             # Ambiente de Python (gestionado exclusivamente con uv)
│   ├── pyproject.toml  # Definición del proyecto y dependencias
│   ├── uv.lock         # Archivo de bloqueo reproducible
│   ├── README.md       # Guía de uso y configuración de Python
│   └── src/            # Código fuente e implementaciones en Python
└── rust/               # Ambiente de Rust (gestionado con Cargo)
    ├── Cargo.toml      # Manifiesto de paquetes y dependencias de Cargo
    ├── Cargo.lock      # Archivo de bloqueo reproducible
    ├── README.md       # Guía de uso e instalación de Rust
    └── src/            # Código fuente e implementaciones en Rust
```

---

## Principio de Diseño: Paridad de Implementación

Cada módulo conceptual o experimento abordado en este repositorio se implementa de manera idéntica en ambos lenguajes:

1. **Python (`python/`):** Enfocado en prototipado rápido, flexibilidad científica y benchmarking estadístico, usando `uv` como gestor único.
2. **Rust (`rust/`):** Enfocado en rendimiento de bajo nivel, seguridad de memoria, concurrencia y robustez tipada, usando la toolchain estable de Rust.

---

## Ambientes y Requisitos Rápidos

- Para detalles sobre la configuración, instalación y reproducción del ambiente de **Python**, consulta [`python/README.md`](python/README.md).
- Para instrucciones detalladas de instalación de la toolchain de **Rust** en Windows, Linux o macOS, consulta [`rust/README.md`](rust/README.md).
