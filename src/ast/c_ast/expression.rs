use crate::ast::ast_item::ASTItem;

pub type Expression = ConstantExpression;

#[derive(Debug, PartialEq)]
pub struct ConstantExpression {
    constant: u32,
}

impl ConstantExpression {
    pub fn new(constant: u32) -> ConstantExpression {
        ConstantExpression { constant }
    }
}

impl ASTItem for ConstantExpression {
    fn generate_assembly(&self) -> String {
        self.constant.to_string()
    }

    fn pretty_print(&self) -> String {
        format!("Int<{}>", self.constant)
    }

    fn pretty_print_2(&self) -> String {
        format!("Const({})", self.constant)
    }
}
