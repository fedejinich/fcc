//! Snapshot capture system.
//!
//! Provides zero-overhead capture when disabled, full snapshot capture when enabled.

use std::fs;
use std::time::Instant;

use chrono::Utc;

use super::differ::compute_diff;
use super::snapshot::{
    CompilationMetadata, Diff, DiffType, IRType, Snapshot, SnapshotFile, SnapshotIndex, Stage,
};
use crate::c_ast::ast::Program;
use crate::codegen::x64::ast::AsmProgram;
use crate::tacky::ast::TackyProgram;

/// Trait for types that can be captured as snapshots.
pub trait SnapshotContent {
    fn to_snapshot_string(&self) -> String;
}

impl SnapshotContent for Program {
    fn to_snapshot_string(&self) -> String {
        format!("{}", self)
    }
}

impl SnapshotContent for TackyProgram {
    fn to_snapshot_string(&self) -> String {
        self.pretty_print()
    }
}

impl SnapshotContent for AsmProgram {
    fn to_snapshot_string(&self) -> String {
        self.pretty_print()
    }
}

/// Captures compilation snapshots.
///
/// Zero overhead when disabled (all methods are no-ops).
pub struct SnapshotCapture {
    snapshots: Vec<Snapshot>,
    index: SnapshotIndex,
    start_time: Instant,
    last_timestamp_ms: u64,
    last_content: Option<String>,
    last_ir_type: Option<IRType>,
    enabled: bool,
    next_id: usize,
    source_file: String,
}

impl SnapshotCapture {
    /// Create a disabled capture (complete NO-OP).
    pub fn disabled() -> Self {
        Self {
            snapshots: Vec::new(),
            index: SnapshotIndex::new(),
            start_time: Instant::now(),
            last_timestamp_ms: 0,
            last_content: None,
            last_ir_type: None,
            enabled: false,
            next_id: 0,
            source_file: String::new(),
        }
    }

    /// Create an enabled capture that will record snapshots.
    pub fn enabled(source_file: String) -> Self {
        Self {
            snapshots: Vec::new(),
            index: SnapshotIndex::new(),
            start_time: Instant::now(),
            last_timestamp_ms: 0,
            last_content: None,
            last_ir_type: None,
            enabled: true,
            next_id: 0,
            source_file,
        }
    }

    /// Check if capture is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Capture a snapshot. NO-OP if disabled.
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

        let content_str = content.to_snapshot_string();
        let now = self.start_time.elapsed().as_millis() as u64;
        let duration_ms = now.saturating_sub(self.last_timestamp_ms);

        // Compute diff if we have previous content and same IR type
        let diff = self.compute_snapshot_diff(&content_str, &ir_type);

        let snapshot = Snapshot {
            id: self.next_id,
            stage: stage.clone(),
            pass: pass.to_string(),
            ir_type: ir_type.clone(),
            timestamp_ms: now,
            duration_ms,
            content: content_str.clone(),
            diff,
        };

        self.index.add(self.next_id, stage, ir_type.clone());
        self.snapshots.push(snapshot);

        self.next_id += 1;
        self.last_timestamp_ms = now;
        self.last_content = Some(content_str);
        self.last_ir_type = Some(ir_type);
    }

    fn compute_snapshot_diff(&self, new_content: &str, new_ir_type: &IRType) -> Option<Diff> {
        let Some(ref last_content) = self.last_content else {
            return None;
        };

        let Some(ref last_ir_type) = self.last_ir_type else {
            return None;
        };

        // If IR type changed, mark as IR transition
        if last_ir_type != new_ir_type {
            return Some(Diff {
                diff_type: DiffType::IrTransition,
                changes: Vec::new(),
            });
        }

        compute_diff(last_content, new_content)
    }

    /// Save snapshots to a JSON file.
    pub fn save_to_file(&self, output_path: &str) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let total_duration_ms = self.start_time.elapsed().as_millis() as u64;

        let snapshot_file = SnapshotFile {
            version: "1.0.0".to_string(),
            compilation_metadata: CompilationMetadata {
                source_file: self.source_file.clone(),
                timestamp: Utc::now().to_rfc3339(),
                total_duration_ms,
                compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            },
            snapshots: self.snapshots.clone(),
            index: SnapshotIndex {
                by_stage: self.index.by_stage.clone(),
                by_ir_type: self.index.by_ir_type.clone(),
            },
        };

        let json = serde_json::to_string_pretty(&snapshot_file)
            .map_err(|e| format!("Failed to serialize snapshots: {}", e))?;

        fs::write(output_path, json)
            .map_err(|e| format!("Failed to write snapshot file: {}", e))?;

        Ok(())
    }
}
