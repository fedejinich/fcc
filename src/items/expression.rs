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
