# Ambiente de Python — Inferencia Causal & Machine Learning

Este subdirectorio contiene las implementaciones en **Python** de los conceptos, algoritmos y experimentos de inferencia causal y aprendizaje automático, usando exclusivamente `uv` para la gestión de dependencias y entornos virtuales.

---

## 1. Requisitos y Herramientas

- **Gestor de entorno y paquetes:** `uv` (exclusivo).
- **Versión de Python:** Python >= 3.14 (gestionado y descargado automáticamente por `uv`).
- **Archivo de bloqueo:** `uv.lock` asegura que cualquier instalación sea 100% reproducible y exacta.

Si no tienes `uv` instalado en otro equipo:
```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```
O en Linux/macOS:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

---

## 2. Configuración y Reproducibilidad

El entorno virtual se sincroniza directamente desde el archivo `uv.lock`.

### Sincronizar el entorno virtual exacto:
```bash
uv sync
```
Esto creará o actualizará `.venv` con las versiones fijadas en el lockfile sin alterar las versiones resueltas.

---

## 3. Flujo de Trabajo

Todos los comandos deben ejecutarse dentro de la carpeta `python/` (o especificando `--directory python`):

- **Ejecutar un script dentro del entorno virtual:**
  ```bash
  uv run python src/causal_ml/__init__.py
  ```
- **Abrir una sesión interactiva (REPL):**
  ```bash
  uv run python
  ```
- **Añadir una nueva dependencia (sólo bajo petición explícita):**
  ```bash
  uv add <nombre-paquete>
  ```
- **Eliminar una dependencia:**
  ```bash
  uv remove <nombre-paquete>
  ```

---

## 4. Dependencias Actuales

- `numpy` (>= 2.5.3)

*Nota: No se deben añadir dependencias adicionales sin solicitud explícita.*
