use crate::ast::print::Printable;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(u32),
}

impl Printable for Expression {
    fn print(&self) -> String {
        match self {
            Expression::Constant(c) => format!("Const({})", c),
        }
    }
}
