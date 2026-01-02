use std::sync::atomic::AtomicUsize;

use crate::{
    c_ast::ast::{Identifier, Statement},
    common::{folder::FolderC, util::temporary_name},
};

static LOOP_LABEL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct LoopLabeler(Identifier);

impl Default for LoopLabeler {
    fn default() -> Self {
        Self::new(Identifier::new("dummy_label".to_string()))
    }
}

impl LoopLabeler {
    pub fn new(current_label: Identifier) -> Self {
        Self(current_label)
    }

    fn _current_label(&self) -> &Identifier {
        &self.0
    }
}

impl FolderC for LoopLabeler {
    fn name(&self) -> &'static str {
        "loop_lab"
    }

    fn fold_st(&mut self, statement: crate::c_ast::ast::Statement) -> Result<Statement, String> {
        let res = match statement {
            Statement::Break(_) => {
                if self._current_label().is_dummy_label() {
                    return Err("statement outside of loop".to_string());
                }
                Statement::Break(self._current_label().clone())
            }

            Statement::Continue(_) => {
                if self._current_label().is_dummy_label() {
                    return Err("statement outside of loop".to_string());
                }
                Statement::Continue(self._current_label().clone())
            }
            Statement::While(cond, body, _) => {
                let label = _unique_loop_label();
                let mut new_labeler = Self::new(label);
                let lab_cond = new_labeler.fold_expr(*cond)?;
                let lab_body = new_labeler.fold_st(*body)?;
                Statement::While(
                    Box::new(lab_cond),
                    Box::new(lab_body),
                    new_labeler._current_label().clone(),
                )
            }
            Statement::DoWhile(body, cond, _) => {
                let label = _unique_loop_label();
                let mut new_labeler = Self::new(label);
                let lab_body = new_labeler.fold_st(*body)?;
                let lab_cond = new_labeler.fold_expr(*cond)?;
                Statement::DoWhile(
                    Box::new(lab_body),
                    Box::new(lab_cond),
                    new_labeler._current_label().clone(),
                )
            }
            Statement::For(for_init, cond, post, body, _) => {
                let label = _unique_loop_label();
                let mut new_labeler = Self::new(label);
                let lab_for_init = new_labeler.fold_for_init(*for_init)?;
                let lab_cond = new_labeler.fold_opt_expr(cond)?;
                let lab_post = new_labeler.fold_opt_expr(post)?;
                let lab_body = new_labeler.fold_st(*body)?;
                Statement::For(
                    Box::new(lab_for_init),
                    lab_cond,
                    lab_post,
                    Box::new(lab_body),
                    new_labeler._current_label().clone(),
                )
            }
            _ => self.default_fold_st(statement)?,
        };
        Ok(res)
    }
}

fn _unique_loop_label() -> Identifier {
    Identifier::new(temporary_name("loop_st", &LOOP_LABEL_COUNT))
}
