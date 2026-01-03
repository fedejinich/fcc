//! TackyBuilder: A thin builder for emitting TACKY instructions.
//!
//! This builder encapsulates:
//! - Name generation for temporaries and labels (replacing the global `static COUNTER`)
//! - The instruction buffer with ergonomic `emit_*` methods
//!
//! # Design Principles
//!
//! - **Thin**: Does not reorder or optimize; emits exactly what you ask for
//! - **Explicit**: Control flow patterns remain visible in the calling code
//! - **Scoped naming**: Each builder instance has its own counter, making names
//!   predictable within a function
//!
//! # Example
//!
//! ```ignore
//! let mut builder = TackyBuilder::new();
//!
//! let tmp = builder.fresh_temp("x");
//! builder.emit_copy(TackyValue::Constant(42), tmp.clone());
//! builder.emit_return(tmp);
//!
//! let instructions = builder.finish();
//! ```

use crate::tacky::ast::{TackyIdentifier, TackyInstruction, TackyValue};

/// Builder for constructing TACKY instruction sequences.
///
/// Manages name generation and instruction accumulation for lowering C AST to TACKY.
pub struct TackyBuilder {
    /// Accumulated instructions
    instructions: Vec<TackyInstruction>,
    /// Counter for generating unique names (temporaries and labels)
    counter: usize,
}

impl TackyBuilder {
    /// Creates a new builder with an empty instruction buffer.
    pub fn new() -> Self {
        TackyBuilder {
            instructions: Vec::new(),
            counter: 0,
        }
    }

    /// Generates a fresh temporary variable with a unique suffix.
    ///
    /// Example: `fresh_temp("tmp")` → `TackyValue::Var("tmp.0")`, then `"tmp.1"`, etc.
    pub fn fresh_temp(&mut self, name: &str) -> TackyValue {
        let id = self.counter;
        self.counter += 1;
        TackyValue::Var(TackyIdentifier {
            value: format!("{name}.{id}"),
        })
    }

    /// Generates a fresh label identifier with a unique suffix.
    ///
    /// Example: `fresh_label("end")` → `TackyIdentifier { value: "end.0" }`
    pub fn fresh_label(&mut self, name: &str) -> TackyIdentifier {
        let id = self.counter;
        self.counter += 1;
        TackyIdentifier {
            value: format!("{name}.{id}"),
        }
    }

    /// Creates a label identifier with a prefix and the loop's label value.
    /// Does NOT increment the counter (labels come from semantic analysis).
    ///
    /// Example: `label_with_prefix("break_", loop_label)` → `"break_loop.1"`
    pub fn label_with_prefix(&self, prefix: &str, label: &crate::c_ast::ast::Identifier) -> TackyIdentifier {
        TackyIdentifier {
            value: format!("{}{}", prefix, label.value()),
        }
    }

    // ========================================================================
    // Basic emit methods
    // ========================================================================

    /// Emits a single instruction.
    pub fn emit(&mut self, instruction: TackyInstruction) {
        self.instructions.push(instruction);
    }

    /// Emits multiple instructions.
    pub fn emit_all(&mut self, instructions: Vec<TackyInstruction>) {
        self.instructions.extend(instructions);
    }

    /// Emits a Label instruction.
    pub fn emit_label(&mut self, label: TackyIdentifier) {
        self.emit(TackyInstruction::Label(label));
    }

    /// Emits a Jump instruction.
    pub fn emit_jump(&mut self, target: TackyIdentifier) {
        self.emit(TackyInstruction::Jump(target));
    }

    /// Emits a JumpIfZero instruction.
    pub fn emit_jump_if_zero(&mut self, condition: TackyValue, target: TackyIdentifier) {
        self.emit(TackyInstruction::JumpIfZero(condition, target));
    }

    /// Emits a JumpIfNotZero instruction.
    pub fn emit_jump_if_not_zero(&mut self, condition: TackyValue, target: TackyIdentifier) {
        self.emit(TackyInstruction::JumpIfNotZero(condition, target));
    }

    /// Emits a Copy instruction.
    pub fn emit_copy(&mut self, src: TackyValue, dst: TackyValue) {
        self.emit(TackyInstruction::Copy(src, dst));
    }

    /// Emits a Return instruction.
    pub fn emit_return(&mut self, value: TackyValue) {
        self.emit(TackyInstruction::Return(value));
    }

    // ========================================================================
    // Finalization
    // ========================================================================

    /// Consumes the builder and returns the accumulated instructions.
    pub fn finish(self) -> Vec<TackyInstruction> {
        self.instructions
    }

    /// Returns the current number of instructions (useful for debugging).
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

impl Default for TackyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
