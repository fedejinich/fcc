//! Snapshot types for serialization.
//!
//! These types represent the JSON structure for compilation snapshots.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root structure for the snapshot JSON file.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotFile {
    pub version: String,
    pub compilation_metadata: CompilationMetadata,
    pub snapshots: Vec<Snapshot>,
    pub index: SnapshotIndex,
}

/// Metadata about the compilation run.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompilationMetadata {
    pub source_file: String,
    pub timestamp: String,
    pub total_duration_ms: u64,
    pub compiler_version: String,
}

/// A single snapshot of the compilation state.
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

/// Compilation stage.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Parse,
    SemanticAnalysis,
    TackyGeneration,
    AssemblyGeneration,
    AssemblyFixup,
}

/// Type of intermediate representation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum IRType {
    #[serde(rename = "C_AST")]
    Cast,
    #[serde(rename = "TACKY")]
    Tacky,
    #[serde(rename = "ASM")]
    Assembly,
}

/// Diff between consecutive snapshots.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Diff {
    #[serde(rename = "type")]
    pub diff_type: DiffType,
    pub changes: Vec<Change>,
}

/// Type of diff computation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiffType {
    LineDiff,
    IrTransition,
}

/// A single change in the diff.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Change {
    pub line: usize,
    #[serde(rename = "type")]
    pub change_type: ChangeType,
    pub before: String,
    pub after: String,
}

/// Type of change (added, removed, or modified).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

/// Index for quick lookup of snapshots.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotIndex {
    pub by_stage: HashMap<Stage, Vec<usize>>,
    pub by_ir_type: HashMap<IRType, Vec<usize>>,
}

impl SnapshotIndex {
    pub fn new() -> Self {
        Self {
            by_stage: HashMap::new(),
            by_ir_type: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: usize, stage: Stage, ir_type: IRType) {
        self.by_stage.entry(stage).or_default().push(id);
        self.by_ir_type.entry(ir_type).or_default().push(id);
    }
}

impl Default for SnapshotIndex {
    fn default() -> Self {
        Self::new()
    }
}
