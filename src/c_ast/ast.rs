use crate::lexer::Token;

#[derive(Clone, Debug)]
pub struct Program(FunctionDefinition);

#[derive(Clone, Debug)]
pub struct FunctionDefinition(Identifier, Block);

#[derive(Clone, Debug)]
pub struct Identifier(String);

#[derive(Clone, Debug)]
pub struct Block(Vec<BlockItem>);

#[derive(Clone, Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

#[derive(Clone, Debug)]
pub struct Declaration(Identifier, Option<Expression>);

#[derive(Clone, Debug)]
pub enum ForInit {
    InitDecl(Box<Declaration>),
    InitExp(Option<Box<Expression>>),
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Statement {
    // Return(exp)
    Return(Expression),
    // Expression(exp)
    Expression(Expression),
    // If (exp condition, statement then, statement? else)
    If(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    // Compound(block)
    Compound(Box<Block>),
    // Break
    Break(Identifier),
    // Continue
    Continue(Identifier),
    // While (exp condition, statement body)
    While(Box<Expression>, Box<Statement>, Identifier),
    // DoWhile (statement body, exp condition)
    DoWhile(Box<Statement>, Box<Expression>, Identifier),
    // For (for_init init, exp? condition, exp? post, statement body)
    For(
        Box<ForInit>,
        Option<Box<Expression>>,
        Option<Box<Expression>>,
        Box<Statement>,
        Identifier,
    ),
    Null,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(i32),
    Var(Identifier),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>), // short circuit evaluation
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

impl Program {
    pub fn new(function_definition: FunctionDefinition) -> Self {
        Program(function_definition)
    }

    pub fn function_definition(&self) -> &FunctionDefinition {
        &self.0
    }
}

impl Block {
    pub fn new(block_items: Vec<BlockItem>) -> Self {
        Self(block_items)
    }

    // only used in tests
    #[allow(dead_code)]
    pub fn iter(&self) -> std::slice::Iter<'_, BlockItem> {
        self.0.iter()
    }

    pub fn block_items(&self) -> &Vec<BlockItem> {
        &self.0
    }
}

impl FunctionDefinition {
    pub fn new(name: Identifier, body: Block) -> Self {
        FunctionDefinition(name, body)
    }

    pub fn name(&self) -> &Identifier {
        &self.0
    }

    pub fn body(&self) -> &Block {
        &self.1
    }
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Identifier(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Declaration {
    pub fn new(name: Identifier, initializer: Option<Expression>) -> Self {
        Declaration(name, initializer)
    }

    pub fn name(&self) -> &Identifier {
        &self.0
    }

    pub fn initializer(&self) -> Option<&Expression> {
        self.1.as_ref()
    }

    pub fn is_declaration(token: Option<&&Token>) -> bool {
        // right now we only support only one declaration type which is the 'int' type declaration
        token == Some(&&Token::Int) // TODO: this is a workaround 
    }
}
