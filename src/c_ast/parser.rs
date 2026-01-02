use std::{iter::Peekable, slice::Iter};

use log::{debug, error, trace};

use crate::{
    c_ast::ast::{
        BinaryOperator, Block, BlockItem, Declaration, Expression, ForInit, FunctionDefinition,
        Identifier, Program, Statement, UnaryOperator,
    },
    common::util::opt_box,
    lexer::{self, Token},
};

type ParseResult<T> = Result<T, String>;

impl TryFrom<Vec<Token>> for Program {
    type Error = String;

    fn try_from(tokens: Vec<Token>) -> ParseResult<Self> {
        trace!("[parser] <program>");
        let mut tokens_iter = tokens.iter().peekable();
        let function_definition = FunctionDefinition::parse_fd(&mut tokens_iter)?;
        let program_ast = Program::new(function_definition);

        if tokens_iter.next().is_some() {
            error!("[parser] unexpected tokens remaining");
            return Err(format!(
                "unexpected tokens remaining: {:?}",
                tokens_iter.collect::<Vec<_>>()
            ));
        }
        Ok(program_ast)
    }
}

impl FunctionDefinition {
    fn parse_fd(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("[parser] <function>");
        token_assert(Token::Int, tokens)?;
        let identifier = Identifier::parse_id(tokens)?;
        debug!("[parser] function: {}", identifier.value());
        token_assert(Token::OpenParen, tokens)?;
        token_assert(Token::Void, tokens)?;
        token_assert(Token::CloseParen, tokens)?;
        let block = Block::parse_block(tokens)?;
        Ok(FunctionDefinition::new(identifier, block))
    }
}

impl Block {
    fn parse_block(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("[parser] <block>");
        token_assert(Token::OpenBrace, tokens)?;
        let mut block_items = vec![];
        while let Some(next_token) = tokens.peek() {
            if *next_token == &Token::CloseBrace {
                break;
            }
            block_items.push(BlockItem::parse_bi(tokens)?);
        }
        token_assert(Token::CloseBrace, tokens)?;
        debug!("[parser] block with {} items", block_items.len());
        Ok(Block::new(block_items))
    }
}

impl BlockItem {
    fn parse_bi(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(next_token) = tokens.peek() else {
            error!("[parser] expected <block_item>");
            return Err("could not parse block item".to_string());
        };
        match next_token {
            Token::Int => Ok(BlockItem::D(Declaration::parse_decl(tokens)?)),
            _ => Ok(BlockItem::S(Statement::parse_st(tokens)?)),
        }
    }
}

impl Declaration {
    fn parse_decl(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("[parser] <declaration>");
        token_assert(Token::Int, tokens)?;
        let name = Identifier::parse_id(tokens)?;
        let mut initializer = None;
        if let Some(Token::Assignment) = tokens.peek() {
            let _ = tokens.next(); // consume '='
            initializer = Expression::parse_opt_exp(tokens, Token::Semicolon)?;
        }
        token_assert(Token::Semicolon, tokens)?;
        debug!("[parser] declaration: {}", name.value());
        Ok(Declaration::new(name, initializer))
    }
}

impl ForInit {
    fn parse_for_init(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("[parser] <for_init>");
        if Declaration::is_declaration(tokens.peek()) {
            return Ok(ForInit::InitDecl(Box::new(Declaration::parse_decl(
                tokens,
            )?)));
        }
        let opt_exp = Expression::parse_opt_exp(tokens, Token::Semicolon)?;
        token_assert(Token::Semicolon, tokens)?;
        Ok(ForInit::InitExp(Box::new(opt_exp)))
    }
}

impl Statement {
    fn parse_st(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(next_token) = tokens.peek() else {
            error!("[parser] expected <statement>");
            return Err("could not parse statement".to_string());
        };
        let statement = match next_token {
            Token::Semicolon => {
                trace!("[parser] <statement> null");
                let _ = tokens.next();
                Statement::Null
            }
            Token::Return => {
                trace!("[parser] <statement> return");
                token_assert(Token::Return, tokens)?;
                let expr = Expression::parse_exp(tokens, Token::Semicolon)?;
                token_assert(Token::Semicolon, tokens)?;
                Statement::Return(expr)
            }
            Token::If => {
                trace!("[parser] <statement> if");
                let _ = tokens.next(); // consume 'if'
                token_assert(Token::OpenParen, tokens)?;
                let expr = Expression::parse_exp(tokens, Token::CloseParen)?;
                token_assert(Token::CloseParen, tokens)?;
                let then = Statement::parse_st(tokens)?;
                let el = if let Some(Token::Else) = tokens.peek() {
                    debug!("[parser] found else branch");
                    let _ = tokens.next(); // consume 'else'
                    Some(Box::new(Statement::parse_st(tokens)?))
                } else {
                    None
                };
                Statement::If(Box::new(expr), Box::new(then), el)
            }
            Token::OpenBrace => {
                trace!("[parser] <statement> compound");
                Statement::Compound(Box::new(Block::parse_block(tokens)?))
            }
            Token::Break => {
                trace!("[parser] <statement> break");
                let _ = tokens.next(); // consume 'break'
                token_assert(Token::Semicolon, tokens)?;
                Statement::Break(Identifier::new("dummy".to_string()))
            }
            Token::Continue => {
                trace!("[parser] <statement> continue");
                token_assert(Token::Continue, tokens)?;
                token_assert(Token::Semicolon, tokens)?;
                Statement::Continue(Identifier::new("dummy".to_string()))
            }
            Token::While => {
                trace!("[parser] <statement> while");
                token_assert(Token::While, tokens)?;
                token_assert(Token::OpenParen, tokens)?;
                let cond = Expression::parse_exp(tokens, Token::CloseParen)?;
                token_assert(Token::CloseParen, tokens)?;
                let body = Statement::parse_st(tokens)?;
                Statement::While(
                    Box::new(cond),
                    Box::new(body),
                    Identifier::new("dummy".to_string()),
                )
            }
            Token::Do => {
                trace!("[parser] <statement> do-while");
                token_assert(Token::Do, tokens)?;
                let body = Statement::parse_st(tokens)?;
                token_assert(Token::While, tokens)?;
                token_assert(Token::OpenParen, tokens)?;
                let cond = Expression::parse_exp(tokens, Token::CloseParen)?;
                token_assert(Token::CloseParen, tokens)?;
                token_assert(Token::Semicolon, tokens)?;
                Statement::DoWhile(
                    Box::new(body),
                    Box::new(cond),
                    Identifier::new("dummy".to_string()),
                )
            }
            Token::For => {
                trace!("[parser] <statement> for");
                token_assert(Token::For, tokens)?;
                token_assert(Token::OpenParen, tokens)?;
                let for_init = ForInit::parse_for_init(tokens)?;
                let cond = Expression::parse_opt_exp(tokens, Token::Semicolon)?;
                token_assert(Token::Semicolon, tokens)?;
                let post = Expression::parse_opt_exp(tokens, Token::CloseParen)?;
                token_assert(Token::CloseParen, tokens)?;
                let body = Statement::parse_st(tokens)?;
                Statement::For(
                    Box::new(for_init),
                    opt_box(cond),
                    opt_box(post),
                    Box::new(body),
                    Identifier::new("dummy".to_string()),
                )
            }
            _ => {
                trace!("[parser] <statement> expression");
                let exp = Expression::parse_exp(tokens, Token::Semicolon)?;
                token_assert(Token::Semicolon, tokens)?;
                Statement::Expression(exp)
            }
        };
        Ok(statement)
    }
}

impl Expression {
    pub fn parse_exp(tokens: &mut Peekable<Iter<Token>>, until: Token) -> ParseResult<Self> {
        Self::parse_opt_exp(tokens, until)?.ok_or_else(|| {
            error!("[parser] expected <exp>");
            "expected expression".to_string()
        })
    }

    pub fn parse_opt_exp(
        tokens: &mut Peekable<Iter<Token>>,
        until: Token,
    ) -> ParseResult<Option<Self>> {
        let Some(next_token) = tokens.peek() else {
            error!("[parser] no tokens left for <exp>");
            return Err("no token left to parse".to_string());
        };
        if *next_token == &until {
            return Ok(None);
        }
        Ok(Some(Self::parse_exp_with_prec(tokens, 0)?))
    }

    fn parse_exp_with_prec(tokens: &mut Peekable<Iter<Token>>, min_prec: i32) -> ParseResult<Self> {
        trace!("[parser] <exp> prec={min_prec}");
        let mut left = Expression::parse_fact(tokens)?;
        let is_binary_op = |t: &Token| lexer::binary_operators().contains(t);

        while let Some(token) = tokens.peek().copied() {
            if !is_binary_op(token) || precedence(token) < min_prec {
                break;
            }
            left = match *token {
                Token::Assignment => {
                    trace!("[parser] <exp> assignment");
                    let _ = tokens.next(); // consume '='
                    let right = Expression::parse_exp_with_prec(tokens, precedence(token))?;
                    Expression::Assignment(Box::new(left), Box::new(right))
                }
                Token::QuestionMark => {
                    trace!("[parser] <exp> ternary");
                    let middle = Expression::parse_conditional_middle(tokens)?;
                    let right = Expression::parse_exp_with_prec(tokens, precedence(token))?;
                    Expression::Conditional(Box::new(left), Box::new(middle), Box::new(right))
                }
                _ => {
                    let op = BinaryOperator::parse_bin(tokens)?;
                    let right = Expression::parse_exp_with_prec(tokens, precedence(token) + 1)?;
                    Expression::Binary(op, Box::new(left), Box::new(right))
                }
            };
        }
        Ok(left)
    }

    fn parse_conditional_middle(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        token_assert(Token::QuestionMark, tokens)?;
        let middle = Expression::parse_exp(tokens, Token::DoubleDot)?;
        token_assert(Token::DoubleDot, tokens)?;
        Ok(middle)
    }

    fn parse_fact(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(next_token) = tokens.peek() else {
            error!("[parser] expected <factor>");
            return Err("could not parse factor".to_string());
        };
        match next_token {
            Token::Constant(n) => {
                let n = n.clone();
                let _ = tokens.next();
                n.parse::<i32>().map(Expression::Constant).map_err(|_| {
                    error!("[parser] invalid constant: {n}");
                    "could not parse constant".to_string()
                })
            }
            Token::Complement | Token::Negate | Token::Not => {
                let unary = UnaryOperator::parse_un(tokens)?;
                let exp = Expression::parse_fact(tokens)?;
                Ok(Expression::Unary(unary, Box::new(exp)))
            }
            Token::OpenParen => {
                let _ = tokens.next();
                let exp = Expression::parse_exp(tokens, Token::CloseParen)?;
                token_assert(Token::CloseParen, tokens)?;
                Ok(exp)
            }
            Token::Identifier(_) => Ok(Expression::Var(Identifier::parse_id(tokens)?)),
            _ => {
                error!("[parser] unexpected token in <factor>: {next_token:?}");
                let _ = tokens.next();
                Err("could not parse expression".to_string())
            }
        }
    }
}

// returns value representing precedence order
// operators are sorted according to the official spec
// https://en.cppreference.com/w/c/language/operator_precedence.html
fn precedence(token: &Token) -> i32 {
    match token {
        Token::Multiply | Token::Divide | Token::Remainder => 50,
        Token::Add | Token::Negate => 45,
        Token::LeftShift | Token::RightShift => 44,
        Token::LessThan
        | Token::LessThanOrEqual
        | Token::GreaterThan
        | Token::GreaterThanOrEqual => 35,
        Token::Equal => 30,
        Token::NotEqual => 30,
        Token::BitwiseAnd => 22,
        Token::BitwiseXor => 21,
        Token::BitwiseOr => 20,
        Token::And => 10,
        Token::Or => 5,
        Token::QuestionMark => 3,
        Token::Assignment => 1,
        _ => 0,
    }
}

impl BinaryOperator {
    fn parse_bin(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(token) = tokens.next() else {
            error!("[parser] expected <binop>");
            return Err("could not parse binary operator".to_string());
        };
        match token {
            Token::Add => Ok(BinaryOperator::Add),
            Token::Multiply => Ok(BinaryOperator::Multiply),
            Token::Divide => Ok(BinaryOperator::Divide),
            Token::Remainder => Ok(BinaryOperator::Remainder),
            Token::Negate => Ok(BinaryOperator::Subtract),
            Token::BitwiseAnd => Ok(BinaryOperator::BitwiseAnd),
            Token::BitwiseOr => Ok(BinaryOperator::BitwiseOr),
            Token::BitwiseXor => Ok(BinaryOperator::BitwiseXor),
            Token::LeftShift => Ok(BinaryOperator::LeftShift),
            Token::RightShift => Ok(BinaryOperator::RightShift),
            Token::And => Ok(BinaryOperator::And),
            Token::Or => Ok(BinaryOperator::Or),
            Token::Equal => Ok(BinaryOperator::Equal),
            Token::NotEqual => Ok(BinaryOperator::NotEqual),
            Token::GreaterThan => Ok(BinaryOperator::GreaterThan),
            Token::LessThan => Ok(BinaryOperator::LessThan),
            Token::GreaterThanOrEqual => Ok(BinaryOperator::GreaterThanOrEqual),
            Token::LessThanOrEqual => Ok(BinaryOperator::LessThanOrEqual),
            _ => {
                error!("[parser] invalid <binop>: {token:?}");
                Err("could not parse binary operator".to_string())
            }
        }
    }
}

impl UnaryOperator {
    fn parse_un(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(token) = tokens.next() else {
            error!("[parser] expected <unop>");
            return Err("could not parse unary operator".to_string());
        };
        match token {
            Token::Complement => Ok(UnaryOperator::Complement),
            Token::Negate => Ok(UnaryOperator::Negate),
            Token::Not => Ok(UnaryOperator::Not),
            _ => {
                error!("[parser] invalid <unop>: {token:?}");
                Err("could not parse unary operator".to_string())
            }
        }
    }
}

impl Identifier {
    fn parse_id(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(Token::Identifier(n)) = tokens.next() else {
            error!("[parser] expected <identifier>");
            return Err("could not parse identifier".to_string());
        };
        Ok(Identifier::new(n.clone()))
    }
}

fn token_assert(expected: Token, tokens: &mut Peekable<Iter<Token>>) -> Result<(), String> {
    let Some(t) = tokens.next() else {
        error!("[parser] unexpected end of tokens, expected {expected:?}");
        return Err(String::from("empty tokens"));
    };
    if *t != expected {
        error!("[parser] expected {expected:?}, got {t:?}");
        return Err(format!("expected {expected:?}, got {t:?}"));
    }
    Ok(())
}
