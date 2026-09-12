# Sistema de Debugging Interactivo para FCC

## Overview del Sistema

Sistema de debugging de dos componentes para el compilador fcc:

1. **Snapshot Generator** (en fcc) - Genera JSON con snapshots de cada transformación
2. **TUI Debugger** (binario separado) - Herramienta interactiva para navegar snapshots

**Comunicación:** JSON único (`program.snapshots.json`)

```
┌──────────────────┐                                        ┌──────────────┐
│   FCC COMPILER   │                                        │ TUI Debugger │
│   + Snapshots    │──────► program.snapshots.json ──────►  │ (fcc-debug)  │
│   Generator      │                                        │              │
└──────────────────┘                                        └──────────────┘
~/Projects/fcc/                                             ~/Projects/fcc-debug/
```

---

## Fases de Implementación

### V1 - Mínimo Viable
- 7 snapshots básicos (sin source IDs ni source_map)
- Timing info (timestamp_ms, duration_ms)
- Diff simple line-by-line (sin summary inteligente)
- TUI con navegación básica (←/→, ↑/↓, q)
- 3 vistas: Single, Split, Diff
- **Sin trace view**

### V2 - Source IDs (futuro)
- Agregar SourceIdGenerator
- Modificar `tacky/from.rs` para comentarios `# source: expr_N`
- Modificar `codegen/x64/from.rs` para propagar source IDs
- Habilitar source_map en snapshots
- Diff con summary inteligente

### V3 - Trace View (futuro)
- Implementar trace view modal en TUI
- Búsqueda de source IDs a través de snapshots
- Jump to snapshot desde trace
- Goto (`g`) y Search (`s`) commands

---

# SPEC 1: Snapshot Generator (para agente implementador en fcc)

## Responsabilidad

Implementar sistema de captura NO INVASIVO en el compilador fcc que genera un archivo JSON con snapshots de cada transformación del pipeline.

## Requisitos

### 1. Flag CLI: `--save-snapshots`

Agregar a `src/driver.rs`:
```rust
#[arg(long, help = "Save compilation snapshots to JSON for debugging")]
save_snapshots: bool,
```

### 2. Estructura del JSON Output

**Archivo generado:** `<program>.snapshots.json`

**Schema V1:**

```json
{
  "version": "1.0.0",
  "compilation_metadata": {
    "source_file": "program.c",
    "timestamp": "2026-01-11T10:30:45Z",
    "total_duration_ms": 1234,
    "compiler_version": "0.1.0"
  },
  "snapshots": [
    {
      "id": 0,
      "stage": "parse",
      "pass": "initial_ast",
      "ir_type": "C_AST",
      "timestamp_ms": 0,
      "duration_ms": 45,
      "content": "Program(\n    Function(...)\n)",
      "diff": null
    },
    {
      "id": 1,
      "stage": "semantic_analysis",
      "pass": "variable_resolver",
      "ir_type": "C_AST",
      "timestamp_ms": 45,
      "duration_ms": 12,
      "content": "Program(...)",
      "diff": {
        "type": "line_diff",
        "changes": [
          {
            "line": 5,
            "type": "modified",
            "before": "Var(\"x\")",
            "after": "Var(\"x.0\")"
          }
        ]
      }
    }
  ],
  "index": {
    "by_stage": {
      "parse": [0],
      "semantic_analysis": [1, 2],
      "tacky_generation": [3],
      "assembly_generation": [4],
      "assembly_fixup": [5, 6]
    },
    "by_ir_type": {
      "C_AST": [0, 1, 2],
      "TACKY": [3],
      "ASM": [4, 5, 6]
    }
  }
}
```

**Campos removidos en V1 (se agregan en V2+):**
- `metadata` (details, stack_offset)
- `source_map`
- `diff.summary`

### 3. Tipos Rust (con serde)

**Ubicación:** `src/debug/snapshot.rs`

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotFile {
    pub version: String,
    pub compilation_metadata: CompilationMetadata,
    pub snapshots: Vec<Snapshot>,
    pub index: SnapshotIndex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilationMetadata {
    pub source_file: String,
    pub timestamp: String,
    pub total_duration_ms: u64,
    pub compiler_version: String,
}

/// V1: Sin metadata ni source_map
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    pub id: usize,
    pub stage: Stage,
    pub pass: String,
    pub ir_type: IRType,
    pub timestamp_ms: u64,
    pub duration_ms: u64,
    pub content: String,
    pub diff: Option<Diff>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Parse,
    SemanticAnalysis,
    TackyGeneration,
    AssemblyGeneration,
    AssemblyFixup,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum IRType {
    #[serde(rename = "C_AST")]
    CAST,
    #[serde(rename = "TACKY")]
    Tacky,
    #[serde(rename = "ASM")]
    Assembly,
}

/// V1: Sin summary (se agrega en V2)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Diff {
    #[serde(rename = "type")]
    pub diff_type: DiffType,
    pub changes: Vec<Change>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiffType {
    LineDiff,
    IrTransition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Change {
    pub line: usize,
    #[serde(rename = "type")]
    pub change_type: ChangeType,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotIndex {
    pub by_stage: HashMap<Stage, Vec<usize>>,
    pub by_ir_type: HashMap<IRType, Vec<usize>>,
}

// ============================================================
// TIPOS V2+ (NO IMPLEMENTAR EN V1)
// ============================================================

/*
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotMetadata {
    pub details: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_offset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceMapEntry {
    pub origin_snapshot: usize,
    pub origin_location: String,
    pub description: String,
}
*/
```

### 4. Puntos de Captura (7 snapshots)

En `src/driver.rs`, capturar en estos puntos exactos:

1. **Después de parsing** (línea ~167)
   - Stage: `Parse`
   - Pass: `"initial_ast"`
   - IR: `C_AST`

2. **Después de VariableResolver** (en `validate_semantics`)
   - Stage: `SemanticAnalysis`
   - Pass: `"variable_resolver"`
   - IR: `C_AST`

3. **Después de LoopLabeler**
   - Stage: `SemanticAnalysis`
   - Pass: `"loop_labeler"`
   - IR: `C_AST`

4. **Después de TACKY generation** (línea ~184)
   - Stage: `TackyGeneration`
   - Pass: `"tacky_from_ast"`
   - IR: `TACKY`

5. **Después de Assembly generation** (línea ~194)
   - Stage: `AssemblyGeneration`
   - Pass: `"tacky_to_asm"`
   - IR: `Assembly`

6. **Después de PseudoRegisterReplacer** (en `do_asm_passes`)
   - Stage: `AssemblyFixup`
   - Pass: `"pseudo_register_replacer"`
   - IR: `Assembly`
   - Metadata extra: `stack_offset` del replacer

7. **Después de InstructionFixer**
   - Stage: `AssemblyFixup`
   - Pass: `"instruction_fixer"`
   - IR: `Assembly`

### 5. Sistema SnapshotCapture (V1)

**Ubicación:** `src/debug/capture.rs`

```rust
use std::time::Instant;

pub struct SnapshotCapture {
    snapshots: Vec<Snapshot>,
    start_time: Instant,
    last_content: Option<String>,  // Para calcular diff
    enabled: bool,
    next_id: usize,
    source_file: String,
}

impl SnapshotCapture {
    /// NO-OP completo cuando snapshots desactivados
    pub fn disabled() -> Self {
        Self {
            snapshots: Vec::new(),
            start_time: Instant::now(),
            last_content: None,
            enabled: false,
            next_id: 0,
            source_file: String::new(),
        }
    }

    pub fn enabled(source_file: String) -> Self {
        Self {
            snapshots: Vec::new(),
            start_time: Instant::now(),
            last_content: None,
            enabled: true,
            next_id: 0,
            source_file,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Captura un snapshot. NO-OP si disabled.
    pub fn capture<T: SnapshotContent>(
        &mut self,
        stage: Stage,
        pass: &str,
        ir_type: IRType,
        content: &T,
    ) {
        if !self.enabled {
            return;
        }
        // Implementar: crear Snapshot, calcular diff, push a snapshots
    }

    pub fn save_to_file(&self, output_path: &str) -> Result<(), String> {
        // Serializar SnapshotFile a JSON
    }
}

/// Trait para tipos que pueden ser capturados como snapshot
pub trait SnapshotContent {
    fn to_snapshot_string(&self) -> String;
}
```

**Implementaciones de SnapshotContent:**

```rust
impl SnapshotContent for Program {
    fn to_snapshot_string(&self) -> String {
        format!("{}", self)  // Usa Display existente
    }
}

impl SnapshotContent for TackyProgram {
    fn to_snapshot_string(&self) -> String {
        self.pretty_print()  // Usa pretty_print existente
    }
}

impl SnapshotContent for AsmProgram {
    fn to_snapshot_string(&self) -> String {
        format!("{}", self)  // Usa Display (debug repr) - VER SECCIÓN 5.1
    }
}
```

### 5.1 Display para Assembly AST (REQUERIDO)

**Problema:** `AsmProgram::to_string_asm()` genera texto assembly final, pero para debugging queremos ver la **estructura del AST** (pseudo-registers, instrucciones sin expandir, etc.).

**Solución:** Crear `impl Display for AsmProgram` en `src/codegen/x64/display.rs`

**Ejemplo de output deseado:**

```
AsmProgram(
    AsmFunction(
        name = "main",
        instructions = [
            AllocateStack(16),
            Mov(Imm(42), Pseudo("tmp.0")),
            Mov(Pseudo("tmp.0"), Reg(AX)),
            Ret
        ]
    )
)
```

**Esto permite ver transformaciones entre fixer passes:**
- Antes de PseudoRegisterReplacer: `Pseudo("tmp.0")`
- Después: `Stack(-4)`
- Antes de InstructionFixer: `Mov(Stack(-4), Stack(-8))`
- Después: `Mov(Stack(-4), Reg(R10)), Mov(Reg(R10), Stack(-8))`

**Archivo a crear:** `src/codegen/x64/display.rs`

```rust
use std::fmt::{Display, Formatter, Result};
use super::ast::*;

impl Display for AsmProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "AsmProgram(")?;
        for func in &self.functions {
            write!(f, "{}", indent(&format!("{}", func), 4))?;
        }
        write!(f, ")")
    }
}

impl Display for AsmFunctionDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "AsmFunction(")?;
        writeln!(f, "    name = \"{}\",", self.name)?;
        writeln!(f, "    instructions = [")?;
        for ins in &self.instructions {
            writeln!(f, "        {},", ins)?;
        }
        writeln!(f, "    ]")?;
        write!(f, ")")
    }
}

impl Display for AsmInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Mov(src, dst) => write!(f, "Mov({}, {})", src, dst),
            Self::Ret => write!(f, "Ret"),
            Self::AllocateStack(n) => write!(f, "AllocateStack({})", n),
            Self::Binary(op, src, dst) => write!(f, "Binary({:?}, {}, {})", op, src, dst),
            // ... etc para todas las variantes
        }
    }
}

impl Display for AsmOperand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Imm(n) => write!(f, "Imm({})", n),
            Self::Reg(r) => write!(f, "Reg({:?})", r),
            Self::Pseudo(name) => write!(f, "Pseudo(\"{}\")", name),
            Self::Stack(offset) => write!(f, "Stack({})", offset),
        }
    }
}
```

### 6. Diff Computation

**Ubicación:** `src/debug/differ.rs`

```rust
pub fn compute_diff(before: &str, after: &str) -> Diff {
    // Line-by-line comparison
    // Retorna Diff con:
    // - changes: Vec<Change>
    // - summary: String descriptivo
}
```

### 7. Source IDs para Trazabilidad (V2+ - NO IMPLEMENTAR EN V1)

> **NOTA:** Esta sección describe funcionalidad para V2+. No implementar en V1.

**SourceIdGenerator** en `src/debug/capture.rs`:

```rust
pub struct SourceIdGenerator {
    counter: usize,
}

impl SourceIdGenerator {
    pub fn new() -> Self;
    pub fn next_expr_id(&mut self) -> String;  // "expr_0", "expr_1", ...
    pub fn next_stmt_id(&mut self) -> String;  // "stmt_0", "stmt_1", ...
}
```

**Propagación:**

En `src/tacky/from.rs` y `src/codegen/x64/from.rs`, agregar comentarios:

```rust
// TACKY
TackyInstruction::Comment("# source: expr_5")

// Assembly
AsmInstruction::Comment("# source: expr_5")
```

### 8. Integración en driver.rs

**Patrón de uso V1:**

```rust
fn compile(&self, preprocessed_file_name: &str) -> Result<String, String> {
    // 1. Inicializar capturador
    let mut snapshots = if self.save_snapshots {
        SnapshotCapture::enabled(preprocessed_file_name.to_string())
    } else {
        SnapshotCapture::disabled()
    };

    // 2. Después de cada pass (V1: sin metadata)
    let c_program = Program::try_from(tokens)?;
    snapshots.capture(Stage::Parse, "initial_ast", IRType::CAST, &c_program);

    // ... más capturas ...

    // 3. Al final, guardar JSON
    if snapshots.is_enabled() {
        let snapshot_file = preprocessed_file_name.replace(".i", ".snapshots.json");
        snapshots.save_to_file(&snapshot_file)?;
        info!("[driver] snapshots saved to {}", snapshot_file);
    }

    Ok(assembly_file_name)
}
```

### 8.1 Refactorización Requerida para Capturar Snapshots Intermedios

**Problema:** Las funciones `validate_semantics()` y `do_asm_passes()` encadenan múltiples passes internamente. Para capturar snapshots después de cada pass individual, necesitamos modificar estas funciones.

**Solución: Pasar SnapshotCapture como parámetro:**

```rust
// ANTES (líneas 265-269 de driver.rs):
pub fn validate_semantics(program: Program) -> Result<Program, String> {
    let program = VariableResolver::default().fold_prog(program)?;
    let program = LoopLabeler::default().fold_prog(program)?;
    Ok(program)
}

// DESPUÉS:
pub fn validate_semantics(
    program: Program,
    snapshots: &mut SnapshotCapture
) -> Result<Program, String> {
    let program = VariableResolver::default().fold_prog(program)?;
    snapshots.capture(Stage::SemanticAnalysis, "variable_resolver", IRType::CAST, &program);

    let program = LoopLabeler::default().fold_prog(program)?;
    snapshots.capture(Stage::SemanticAnalysis, "loop_labeler", IRType::CAST, &program);

    Ok(program)
}
```

```rust
// ANTES (líneas 226-234 de driver.rs):
fn do_asm_passes(&self, assembly_program: AsmProgram) -> Result<AsmProgram, String> {
    let mut replacer = PseudoRegisterReplacer::default();
    let assembly_program = replacer.fold_prog(assembly_program);
    let last_offset = replacer.last_offset();
    let assembly_program = InstructionFixer::new(last_offset).fold_prog(assembly_program);
    Ok(assembly_program)
}

// DESPUÉS:
fn do_asm_passes(
    &self,
    assembly_program: AsmProgram,
    snapshots: &mut SnapshotCapture
) -> Result<AsmProgram, String> {
    let mut replacer = PseudoRegisterReplacer::default();
    let assembly_program = replacer.fold_prog(assembly_program);
    snapshots.capture(Stage::AssemblyFixup, "pseudo_register_replacer", IRType::Assembly, &assembly_program);

    let last_offset = replacer.last_offset();
    let assembly_program = InstructionFixer::new(last_offset).fold_prog(assembly_program);
    snapshots.capture(Stage::AssemblyFixup, "instruction_fixer", IRType::Assembly, &assembly_program);

    Ok(assembly_program)
}
```

## Archivos a Crear/Modificar (V1)

**Crear:**
- `src/debug/mod.rs` - Módulo exports
- `src/debug/snapshot.rs` - Tipos con serde derives
- `src/debug/capture.rs` - SnapshotCapture struct
- `src/debug/differ.rs` - Diff computation line-by-line
- `src/codegen/x64/display.rs` - Display para Assembly AST (debug repr)

**Modificar:**
- `Cargo.toml` - Agregar `serde = { version = "1.0", features = ["derive"] }` y `serde_json = "1.0"`
- `src/lib.rs` - Agregar `pub mod debug;`
- `src/codegen/x64/mod.rs` - Agregar `pub mod display;`
- `src/driver.rs` - Agregar flag `--save-snapshots` + integración de snapshots (7 puntos de captura)

**NO modificar en V1:**
- `src/tacky/from.rs` - (Source IDs son V2+)
- `src/codegen/x64/from.rs` - (Source IDs son V2+)

## Principios NO INVASIVOS

✅ **DEBE cumplir:**
- Zero overhead cuando flag desactivado
- `SnapshotCapture::disabled()` debe ser NO-OP completo
- No modificar traits `Folder*` existentes
- Reutilizar Display/pretty_print existentes
- No cambiar comportamiento del compilador

❌ **NO debe:**
- Modificar lógica de passes (VariableResolver, LoopLabeler, etc.)
- Agregar dependencies al código productivo
- Cambiar interfaces públicas

## Testing

```bash
# Test básico
cargo run -- examples/test.c --save-snapshots

# Verificar JSON generado
ls -lh examples/test.snapshots.json
jq '.snapshots | length' examples/test.snapshots.json  # Debe ser 7
jq '.index.by_stage | keys' examples/test.snapshots.json

# Validar schema
jq '.' examples/test.snapshots.json > /dev/null && echo "Valid JSON"
```

---

# SPEC 2: TUI Debugger (para equipo de agentes implementadores)

## Responsabilidad

Crear binario independiente `fcc-debug` que lee el JSON de snapshots y provee una interface TUI interactiva tipo debugger para navegar y analizar transformaciones del compilador.

## Requisitos del Proyecto

### 1. Estructura del Proyecto

**Ubicación:** `/Users/void_rsk/Projects/fcc-debug/` (proyecto independiente, NO workspace)

```
fcc-debug/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── ui.rs
│   ├── snapshot_loader.rs
│   └── navigation.rs
└── README.md
```

**Cargo.toml:**

```toml
[package]
name = "fcc-debug"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "fcc-debug"
path = "src/main.rs"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ratatui = "0.28"
crossterm = "0.28"
anyhow = "1.0"
```

**NOTA:** Este es un proyecto completamente independiente de fcc. No comparte workspace ni dependencias.

### 2. Input: JSON Schema

El TUI debe leer el JSON generado por el Snapshot Generator (ver SPEC 1, sección 2).

**Estructuras Rust para deserialización (V1):**

```rust
// src/snapshot_loader.rs

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SnapshotFile {
    pub version: String,
    pub compilation_metadata: CompilationMetadata,
    pub snapshots: Vec<Snapshot>,
    pub index: SnapshotIndex,
}

#[derive(Debug, Deserialize)]
pub struct CompilationMetadata {
    pub source_file: String,
    pub timestamp: String,
    pub total_duration_ms: u64,
    pub compiler_version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Snapshot {
    pub id: usize,
    pub stage: Stage,
    pub pass: String,
    pub ir_type: IRType,
    pub timestamp_ms: u64,
    pub duration_ms: u64,
    pub content: String,
    pub diff: Option<Diff>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Parse,
    SemanticAnalysis,
    TackyGeneration,
    AssemblyGeneration,
    AssemblyFixup,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum IRType {
    #[serde(rename = "C_AST")]
    CAST,
    #[serde(rename = "TACKY")]
    Tacky,
    #[serde(rename = "ASM")]
    Assembly,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Diff {
    #[serde(rename = "type")]
    pub diff_type: DiffType,
    pub changes: Vec<Change>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DiffType {
    LineDiff,
    IrTransition,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Change {
    pub line: usize,
    #[serde(rename = "type")]
    pub change_type: ChangeType,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotIndex {
    pub by_stage: HashMap<Stage, Vec<usize>>,
    pub by_ir_type: HashMap<IRType, Vec<usize>>,
}

pub fn load_snapshot_file(path: &str) -> Result<SnapshotFile, anyhow::Error> {
    let content = std::fs::read_to_string(path)?;
    let snapshot_file: SnapshotFile = serde_json::from_str(&content)?;
    Ok(snapshot_file)
}
```

### 3. CLI Interface

```bash
# Uso básico
fcc-debug <snapshot-file.json>

# Ejemplo
fcc-debug examples/program.snapshots.json
```

### 4. UI Layout Requerido

#### Vista Principal: Split View (default)

```
┌────────────────────────────────────────────────────────────────┐
│ FCC Debug TUI | Snapshot 3/7 | Stage: semantic | Pass: var_res│ ← HEADER
├──────────────────────────────┬─────────────────────────────────┤
│ Previous: initial_ast        │ Current: variable_resolver      │
│                              │                                 │
│ Program(                     │ Program(                        │
│   Function(                  │   Function(                     │ ← CONTENT
│     name="main",             │     name="main",                │
│     body=Var("x")            │     body=Var("x.0") ◄ highlighted
│   )                          │   )                             │
│ )                            │ )                               │
│                              │                                 │
├──────────────────────────────┴─────────────────────────────────┤
│ q: Quit | ←/→: Prev/Next | ↑/↓: Scroll | v: View | t: Trace  │ ← FOOTER
└────────────────────────────────────────────────────────────────┘
```

### 5. Modos de Vista Requeridos

**A. Single Panel**
- Muestra solo snapshot actual
- Ocupa todo el espacio de contenido

**B. Split View** (default, recomendado)
- Panel izquierdo: snapshot anterior
- Panel derecho: snapshot actual
- 50/50 split

**C. Diff View**
- Muestra solo los cambios del diff
- Colores:
  - Verde: líneas agregadas (`+`)
  - Rojo: líneas removidas (`-`)
  - Amarillo: líneas modificadas (`~`)
- Formato:
```
Diff Summary: Renamed 3 variables

~ Line 5:
  - Var("x")
  + Var("x.0")
```

**D. Trace View** (V3 - NO IMPLEMENTAR EN V1)

> Esta vista requiere source IDs (V2). No implementar en V1.

- Se activa con tecla `t`
- Muestra journey completo de un source ID
- Layout:
```
┌─────────────────────────────────────────────┐
│ TRACE HISTORY: expr_5                       │
│ Binary(Add, Var(x), Var(y))                │
├─────────────────────────────────────────────┤
│                                             │
│ [0] Parse: initial_ast                     │
│   Expression::Binary(Add, Var("x"), ...)   │
│                                             │
│ [1] Semantic: variable_resolver            │
│   Expression::Binary(Add, Var("x.0"), ...) │
│                                             │
│ [3] TACKY: tacky_from_ast                  │
│   tmp.2 = tmp.0 + tmp.1  # source: expr_5 │
│                                             │
│ [6] Assembly: pseudo_register_replacer     │
│   addl -4(%rbp), -8(%rbp)  # source: expr_5│
│                                             │
│ Press 0-6 to jump | ESC to close           │
└─────────────────────────────────────────────┘
```

### 6. Comandos de Navegación Requeridos

#### V1 - Básicos (REQUERIDOS):
- `←` / `→` - Snapshot anterior/siguiente
- `↑` / `↓` - Scroll arriba/abajo en contenido
- `v` - Toggle view mode (Single → Split → Diff → Single)
- `q` - Quit
- `Home` / `End` - Ir al primer/último snapshot
- `1-7` - Saltar a snapshot por número

#### V3 - Avanzados (NO IMPLEMENTAR EN V1):
- `t` - **Trace**: Abrir trace view para source ID bajo cursor
  - Requiere source IDs (V2)

- `g` - **Goto**: Dialog para saltar a stage o pass específico

- `s` - **Search**: Dialog para buscar texto en snapshots

### 7. Estado de la Aplicación (V1)

```rust
// src/app.rs

pub struct App {
    pub snapshot_file: SnapshotFile,
    pub current_index: usize,
    pub view_mode: ViewMode,
    pub scroll_offset: usize,
    pub quit: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ViewMode {
    SinglePanel,
    #[default]
    SplitView,
    DiffView,
}

impl App {
    pub fn new(snapshot_file: SnapshotFile) -> Self {
        Self {
            snapshot_file,
            current_index: 0,
            view_mode: ViewMode::default(),
            scroll_offset: 0,
            quit: false,
        }
    }

    // Navegación
    pub fn next_snapshot(&mut self) {
        if self.current_index < self.snapshot_file.snapshots.len() - 1 {
            self.current_index += 1;
            self.scroll_offset = 0;
        }
    }

    pub fn prev_snapshot(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.scroll_offset = 0;
        }
    }

    pub fn goto_snapshot(&mut self, index: usize) {
        if index < self.snapshot_file.snapshots.len() {
            self.current_index = index;
            self.scroll_offset = 0;
        }
    }

    pub fn first_snapshot(&mut self) {
        self.goto_snapshot(0);
    }

    pub fn last_snapshot(&mut self) {
        self.goto_snapshot(self.snapshot_file.snapshots.len() - 1);
    }

    // Views
    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::SinglePanel => ViewMode::SplitView,
            ViewMode::SplitView => ViewMode::DiffView,
            ViewMode::DiffView => ViewMode::SinglePanel,
        };
    }

    pub fn current_snapshot(&self) -> &Snapshot {
        &self.snapshot_file.snapshots[self.current_index]
    }

    pub fn previous_snapshot(&self) -> Option<&Snapshot> {
        if self.current_index > 0 {
            Some(&self.snapshot_file.snapshots[self.current_index - 1])
        } else {
            None
        }
    }

    // Scroll
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }
}
```

**Campos removidos de V1 (V3):**
- `trace_view_active`
- `trace_history`
- Métodos `activate_trace()`, `close_trace()`, `jump_to_trace_entry()`

### 8. Renderizado con Ratatui (V1)

**Framework:** ratatui 0.28 + crossterm 0.28

**Componentes UI:**

```rust
// src/ui.rs

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, ViewMode};

pub fn draw(f: &mut Frame, app: &App) {
    // Layout: Header (3 lines) + Content (flex) + Footer (3 lines)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);

    match app.view_mode {
        ViewMode::SinglePanel => draw_single_panel(f, app, chunks[1]),
        ViewMode::SplitView => draw_split_view(f, app, chunks[1]),
        ViewMode::DiffView => draw_diff_view(f, app, chunks[1]),
    }

    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let snapshot = app.current_snapshot();
    let total = app.snapshot_file.snapshots.len();
    let title = format!(
        "FCC Debug TUI | Snapshot {}/{} | Stage: {:?} | Pass: {}",
        app.current_index + 1,
        total,
        snapshot.stage,
        snapshot.pass
    );
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(block, area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let help = "q: Quit | ←/→: Prev/Next | ↑/↓: Scroll | v: View | 1-7: Jump";
    let block = Block::default()
        .title(help)
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_single_panel(f: &mut Frame, app: &App, area: Rect) {
    let content = &app.current_snapshot().content;
    let paragraph = Paragraph::new(content.as_str())
        .block(Block::default().title("Current").borders(Borders::ALL))
        .scroll((app.scroll_offset as u16, 0));
    f.render_widget(paragraph, area);
}

fn draw_split_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Panel izquierdo: anterior
    if let Some(prev) = app.previous_snapshot() {
        let p = Paragraph::new(prev.content.as_str())
            .block(Block::default().title(format!("Previous: {}", prev.pass)).borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray))
            .scroll((app.scroll_offset as u16, 0));
        f.render_widget(p, chunks[0]);
    } else {
        let p = Paragraph::new("(No previous snapshot)")
            .block(Block::default().title("Previous").borders(Borders::ALL));
        f.render_widget(p, chunks[0]);
    }

    // Panel derecho: actual
    let curr = app.current_snapshot();
    let p = Paragraph::new(curr.content.as_str())
        .block(Block::default().title(format!("Current: {}", curr.pass)).borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .scroll((app.scroll_offset as u16, 0));
    f.render_widget(p, chunks[1]);
}

fn draw_diff_view(f: &mut Frame, app: &App, area: Rect) {
    let snapshot = app.current_snapshot();
    let content = if let Some(ref diff) = snapshot.diff {
        format_diff(diff)
    } else {
        "(No diff available - first snapshot)".to_string()
    };
    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Diff View").borders(Borders::ALL))
        .scroll((app.scroll_offset as u16, 0));
    f.render_widget(paragraph, area);
}

fn format_diff(diff: &Diff) -> String {
    let mut lines = Vec::new();
    for change in &diff.changes {
        match change.change_type {
            ChangeType::Added => {
                lines.push(format!("+ Line {}: {}", change.line, change.after));
            }
            ChangeType::Removed => {
                lines.push(format!("- Line {}: {}", change.line, change.before));
            }
            ChangeType::Modified => {
                lines.push(format!("~ Line {}:", change.line));
                lines.push(format!("  - {}", change.before));
                lines.push(format!("  + {}", change.after));
            }
        }
    }
    lines.join("\n")
}
```

### 9. Event Loop (V1)

```rust
// src/main.rs

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod snapshot_loader;
mod ui;

use app::App;
use snapshot_loader::load_snapshot_file;

fn main() -> anyhow::Result<()> {
    // Parse args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: fcc-debug <snapshot.json>");
        std::process::exit(1);
    }

    // Load snapshots
    let snapshot_file = load_snapshot_file(&args[1])?;
    if snapshot_file.snapshots.is_empty() {
        eprintln!("Error: No snapshots found in file");
        std::process::exit(1);
    }
    let mut app = App::new(snapshot_file);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Cleanup (always runs, even on error)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            // Solo procesar KeyPress (evita duplicados en Windows)
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => app.prev_snapshot(),
                KeyCode::Right => app.next_snapshot(),
                KeyCode::Up => app.scroll_up(),
                KeyCode::Down => app.scroll_down(),
                KeyCode::Char('v') => app.toggle_view_mode(),
                KeyCode::Home => app.first_snapshot(),
                KeyCode::End => app.last_snapshot(),
                // Jump to snapshot by number (1-7)
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(n) = c.to_digit(10) {
                        if n >= 1 && n <= 9 {
                            app.goto_snapshot((n - 1) as usize);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
```

### 10. Feature: Trace View (V3 - NO IMPLEMENTAR EN V1)

> **NOTA:** Esta sección describe funcionalidad para V3. Requiere source IDs (V2). No implementar en V1.

**Lógica de extracción de source ID:**

```rust
// src/trace_view.rs (V3)

pub fn extract_source_id(snapshot: &Snapshot, cursor_line: usize) -> Option<String> {
    // Parsear línea actual del snapshot.content
    // Buscar patrón: # source: <id>
    // Retornar Some(id) o None
}

pub fn build_trace_history(
    snapshots: &[Snapshot],
    source_id: &str
) -> Vec<TraceEntry> {
    // Buscar source_id en:
    // 1. snapshot.content (líneas con "# source: <id>")
    // 2. snapshot.source_map (HashMap<String, SourceMapEntry>)

    // Retornar lista de TraceEntry ordenados por snapshot_id
}

pub fn draw_trace_view(
    f: &mut Frame,
    app: &App,
    trace_history: &[TraceEntry]
) {
    // Modal centrado (80% width, 80% height)
    // Header: "TRACE HISTORY: {source_id}"
    // Content: Lista scrolleable de entries
    // Footer: "Press 0-9 to jump | ESC to close"
}
```

### 11. Estilos y Colores

**Paleta recomendada:**

```rust
use ratatui::style::{Color, Style, Modifier};

const HEADER_STYLE: Style = Style::default().fg(Color::Cyan);
const CURRENT_SNAPSHOT_STYLE: Style = Style::default().fg(Color::Green);
const PREV_SNAPSHOT_STYLE: Style = Style::default().fg(Color::Gray);

const DIFF_ADDED: Style = Style::default().fg(Color::Green);
const DIFF_REMOVED: Style = Style::default().fg(Color::Red);
const DIFF_MODIFIED: Style = Style::default().fg(Color::Yellow);

const TRACE_HIGHLIGHT: Style = Style::default()
    .fg(Color::Yellow)
    .add_modifier(Modifier::BOLD);
```

### 12. Testing del TUI (V1)

```bash
# Build
cd ~/Projects/fcc-debug
cargo build --release

# Test con snapshot real (requiere SPEC 1 implementado)
cd ~/Projects/fcc
cargo run -- examples/test.c --save-snapshots
~/Projects/fcc-debug/target/release/fcc-debug examples/test.snapshots.json

# Verificaciones V1:
# 1. Se abre TUI sin crashes
# 2. Header muestra info correcta (snapshot N/7, stage, pass)
# 3. → avanza snapshots (1→2→3→...→7)
# 4. ← retrocede snapshots
# 5. v cambia vistas (Single→Split→Diff→Single)
# 6. Split view muestra dos paneles lado a lado
# 7. Diff view muestra cambios (sin colores por ahora, solo texto)
# 8. ↑/↓ hacen scroll
# 9. Home/End van al primer/último snapshot
# 10. 1-7 saltan al snapshot correspondiente
# 11. q sale del TUI
```

## Deliverables (V1)

1. **Proyecto funcional** en `~/Projects/fcc-debug/`
2. **Binario compilado** `fcc-debug` que:
   - Lee JSON de snapshots
   - Muestra TUI interactivo
   - Permite navegación completa (←/→, ↑/↓, Home/End, 1-7)
   - Soporta 3 modos de vista (Single, Split, Diff)
3. **README.md** en `~/Projects/fcc-debug/` con:
   - Instrucciones de build
   - Instrucciones de uso
   - Lista de comandos

## Restricciones

❌ **NO debe:**
- Modificar código del compilador fcc (eso es SPEC 1)
- Generar snapshots (eso lo hace SPEC 1)
- Depender de fcc como librería
- Implementar trace view en V1 (es V3)

✅ **DEBE:**
- Ser binario completamente independiente
- Solo leer JSON como input
- Funcionar con cualquier snapshot JSON válido según schema V1

---

## Verificación End-to-End (V1)

```bash
# Paso 1: Generar snapshots (SPEC 1)
cd ~/Projects/fcc
cargo run -- examples/factorial.c --save-snapshots
# Output: examples/factorial.snapshots.json

# Paso 2: Validar JSON
jq '.snapshots | length' examples/factorial.snapshots.json
# Debe retornar: 7

jq '.snapshots[0].stage' examples/factorial.snapshots.json
# Debe retornar: "parse"

jq '.compilation_metadata.source_file' examples/factorial.snapshots.json
# Debe retornar el nombre del archivo

# Paso 3: Abrir TUI (SPEC 2)
cd ~/Projects/fcc-debug
cargo run -- ~/Projects/fcc/examples/factorial.snapshots.json

# Paso 4: Navegar en TUI (V1)
# - Presionar → 6 veces (ver todos los snapshots: 1→2→3→4→5→6→7)
# - Presionar v dos veces (Single→Split→Diff)
# - Presionar ↑/↓ para scroll
# - Presionar Home para ir al primer snapshot
# - Presionar 4 para saltar al snapshot 4
# - Presionar q para salir

# Paso 5: Verificar no hay crashes ni errores
echo "✓ Sistema V1 completo funcional"
```
