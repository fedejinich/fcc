pub struct Program {
    pub function_definition: FunctionDefinition,
}

pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Vec<BlockItem>,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub name: Identifier,
    pub initializer: Option<Expression>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    Null,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(i32),
    Var(Identifier),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    // bitwise operators are binary operators as well
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    // logical operators
    And,
    Or,

    // relational operators
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Identifier {
            value: value.to_string(),
        }
    }
}

impl Declaration {
    pub fn new(name: Identifier, initializer: Option<Expression>) -> Self {
        Declaration { name, initializer }
    }
}
