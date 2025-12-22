use std::{iter::Peekable, slice::Iter};

use log::{debug, trace};

use crate::{
    c_ast::ast::{
        BinaryOperator, Block, BlockItem, Declaration, Expression, FunctionDefinition, Identifier,
        Program, Statement, UnaryOperator,
    },
    lexer::{self, Token},
};

// ques: should i do a trait for 'from(tokens: Vec<Token>) -> ParseResult<Self>'?

type ParseResult<T> = Result<T, String>;

impl TryFrom<Vec<Token>> for Program {
    type Error = String;

    fn try_from(tokens: Vec<Token>) -> Result<Self, Self::Error> {
        trace!("Parsing <program>");

        let mut iter = tokens.iter().peekable();
        let function_definition = FunctionDefinition::parse_fd(&mut iter)?;

        let program_ast = Program::new(function_definition);

        if iter.next().is_some() {
            return Err(format!(
                "unexpected tokens remaining: {:?}",
                iter.collect::<Vec<_>>()
            ));
        }

        trace!("<program> parsing completed successfully");

        debug!("Parsed C source code successfully");

        Ok(program_ast)
    }
}

impl FunctionDefinition {
    fn parse_fd(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("Parsing <function>");

        token_eq(Token::Int, tokens)?;

        let identifier = Identifier::parse_id(tokens)?;

        token_eq(Token::OpenParen, tokens)?;
        token_eq(Token::Void, tokens)?;
        token_eq(Token::CloseParen, tokens)?;
        token_eq(Token::OpenBrace, tokens)?;

        trace!("Parsing {{ <block_item> }}");

        let mut block_items = vec![];
        while let Some(next_token) = tokens.peek() {
            if *next_token == &Token::CloseBrace {
                break;
            }
            let block_item = BlockItem::parse_bi(tokens)?;
            block_items.push(block_item);
        }
        let block = Block::new(block_items);

        token_eq(Token::CloseBrace, tokens)?;

        trace!("<function> parsing completed successfully");

        Ok(FunctionDefinition::new(identifier, block))
    }
}

impl BlockItem {
    fn parse_bi(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("Parsing <block_item>");
        let Some(next_token) = tokens.peek() else {
            return Err("could not parse block item".to_string());
        };
        let block_item = match next_token {
            // TODO: this will turn into is_keyword
            Token::Int => {
                trace!("Parsing <block_item> ::= <declaration>");
                BlockItem::D(Declaration::parse_decl(tokens)?)
            }
            _ => {
                trace!("Parsing <block_item> ::= <statement>");
                BlockItem::S(Statement::parse_st(tokens)?)
            }
        };
        Ok(block_item)
    }
}

impl Declaration {
    fn parse_decl(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("Parsing <declaration> := \"int\" <identifier> [ \"=\" <exp> ] \";\"");

        token_eq(Token::Int, tokens)?;
        let name = Identifier::parse_id(tokens)?;

        let mut initializer = None;
        if let Some(Token::Assignment) = tokens.peek() {
            let _ = tokens.next(); // consume '='
            initializer = Some(Expression::parse_exp(tokens, 0)?);
        }

        token_eq(Token::Semicolon, tokens)?;

        let dec = Declaration { name, initializer };

        trace!("Parsed <declaration> {dec:?}");

        Ok(dec)
    }
}

impl Statement {
    fn parse_st(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(next_token) = tokens.peek() else {
            return Err("could not parse statement".to_string());
        };
        let statement = match next_token {
            Token::Semicolon => {
                trace!("Parsing <statement> ::= ;");

                let _ = tokens.next();

                Statement::Null
            }
            Token::Return => {
                trace!("Parsing <statement> ::= return <exp> ;");

                token_eq(Token::Return, tokens)?;
                // start with a minimum precedence of zero so
                // the result includes operators at every precedence level
                let expr = Expression::parse_exp(tokens, 0)?;
                token_eq(Token::Semicolon, tokens)?;

                Statement::Return(expr)
            }
            Token::If => {
                trace!(
                    "Parsing <statement> ::= if ( <exp> ) then <statement> [else <statement>] ;"
                );

                let _ = tokens.next(); // consume 'if'
                token_eq(Token::OpenParen, tokens)?;
                let expr = Expression::parse_exp(tokens, 0)?;
                token_eq(Token::CloseParen, tokens)?;
                let then = Statement::parse_st(tokens)?;
                let mut el = None;

                if let Some(Token::Else) = tokens.peek() {
                    let _ = tokens.next(); // consume 'else'
                    el = Some(Box::new(Statement::parse_st(tokens)?));
                }

                Statement::If(Box::new(expr), Box::new(then), el)
            }
            _ => {
                trace!("Parsing <statement> ::= <exp> ;");

                let exp = Expression::parse_exp(tokens, 0)?;

                token_eq(Token::Semicolon, tokens)?;

                Statement::Expression(exp)
            }
        };

        trace!("Parsed <statement>");

        Ok(statement)
    }
}

impl Expression {
    // TODO: this function is too long
    fn parse_exp(tokens: &mut Peekable<Iter<Token>>, min_prec: i32) -> ParseResult<Self> {
        trace!("Parsing <exp> with min_prec: {min_prec}");
        let mut left = Expression::parse_fact(tokens)?;
        let mut next_token = tokens.peek().copied();

        // ques: isn't this too expensive?
        let is_binary_op = |t: &Token| lexer::binary_operators().contains(t);
        while let Some(token) = next_token {
            if !is_binary_op(token) || precedence(token) < min_prec {
                trace!("No bin op ({token:?})");
                break;
            }
            trace!("Parsing <exp> ::= <exp> <binop> <exp>");
            match *token {
                Token::Assignment => {
                    trace!("Parsing assignment");
                    let _ = tokens.next(); // consume '='
                    let right = Expression::parse_exp(tokens, precedence(token))?;
                    left = Expression::Assignment(Box::new(left), Box::new(right));
                }
                Token::QuestionMark => {
                    trace!("Parsing <exp> ::= <exp> ? <exp> : <exp>");
                    let middle = Expression::parse_conditional_middle(tokens)?;
                    let right = Expression::parse_exp(tokens, precedence(token))?;
                    left =
                        Expression::Conditional(Box::new(left), Box::new(middle), Box::new(right));
                }
                _ => {
                    let op = BinaryOperator::parse_bin(tokens)?;
                    trace!(
                        "Parsing binary operator <exp> with precedence {}",
                        precedence(token) + 1
                    );
                    let right = Expression::parse_exp(tokens, precedence(token) + 1)?;
                    left = Expression::Binary(op, Box::new(left), Box::new(right));
                }
            }
            next_token = tokens.peek().copied(); // TODO: reconsider copied()
        }

        trace!("Parsed <exp> {:?}", &left);

        Ok(left)
    }

    fn parse_conditional_middle(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("Parsing <conditional_middle>");
        token_eq(Token::QuestionMark, tokens)?; // consume '?'
        let middle = Expression::parse_exp(tokens, 0)?;
        token_eq(Token::DoubleDot, tokens)?;

        Ok(middle)
    }

    fn parse_fact(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        trace!("Parsing <factor>");
        let Some(next_token) = tokens.peek() else {
            return Err("could not parse factor".to_string());
        };
        let factor = match next_token {
            Token::Constant(n) => {
                trace!("Parsing <factor> ::= <int>");
                let _ = tokens.next();
                if let Ok(num) = n.parse::<i32>() {
                    Expression::Constant(num)
                } else {
                    return Err("could not parse constant".to_string());
                }
            }
            Token::Complement | Token::Negate | Token::Not => {
                trace!("Parsing <unop> <factor>");
                let unary = UnaryOperator::parse_un(tokens)?;
                let exp = Expression::parse_fact(tokens)?;
                Expression::Unary(unary, Box::new(exp))
            }
            Token::OpenParen => {
                trace!("Parsing \"(\" <exp> \")\"");
                let _ = tokens.next();
                let exp = Expression::parse_exp(tokens, 0)?;
                token_eq(Token::CloseParen, tokens)?;
                exp
            }
            Token::Identifier(_) => {
                let id = Identifier::parse_id(tokens)?;
                Expression::Var(id)
            }
            _ => {
                trace!("Exiting <factor> (error) {next_token:?}");
                let _ = tokens.next();
                return Err("could not parse expression".to_string());
            }
        };

        Ok(factor)
    }
}

// returns value representing precedence order
// operators are sorted according to the official spec
// https://en.cppreference.com/w/c/language/operator_precedence.html
fn precedence(token: &Token) -> i32 {
    debug!("<precedence>: {token:?}");
    match token {
        Token::Multiply | Token::Divide | Token::Remainder => 50,
        Token::Plus | Token::Negate => 45,
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
            return Err("could not parse binary operator".to_string());
        };
        let binop = match token {
            Token::Plus => BinaryOperator::Add,
            Token::Multiply => BinaryOperator::Multiply,
            Token::Divide => BinaryOperator::Divide,
            Token::Remainder => BinaryOperator::Remainder,
            Token::Negate => BinaryOperator::Subtract,
            // binary operators
            Token::BitwiseAnd => BinaryOperator::BitwiseAnd,
            Token::BitwiseOr => BinaryOperator::BitwiseOr,
            Token::BitwiseXor => BinaryOperator::BitwiseXor,
            Token::LeftShift => BinaryOperator::LeftShift,
            Token::RightShift => BinaryOperator::RightShift,
            // logical operators
            Token::And => BinaryOperator::And,
            Token::Or => BinaryOperator::Or,
            // relational operators
            Token::Equal => BinaryOperator::Equal,
            Token::NotEqual => BinaryOperator::NotEqual,
            Token::GreaterThan => BinaryOperator::GreaterThan,
            Token::LessThan => BinaryOperator::LessThan,
            Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
            Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
            _ => return Err("could not parse binary operator".to_string()),
        };

        trace!("Parsed <binop>: {binop:?}");

        Ok(binop)
    }
}

impl UnaryOperator {
    fn parse_un(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(token) = tokens.next() else {
            return Err("could not parse unary operator".to_string());
        };

        let unop = match token {
            Token::Complement => UnaryOperator::Complement,
            Token::Negate => UnaryOperator::Negate,
            Token::Not => UnaryOperator::Not,
            _ => return Err("could not parse unary operator".to_string()),
        };

        trace!("Parsed <unop>: {unop:?}");

        Ok(unop)
    }
}

impl Identifier {
    fn parse_id(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let Some(Token::Identifier(n)) = tokens.next() else {
            debug!("Expected <identifier> but found none");
            return Err("could not parse identifier".to_string());
        };
        trace!("Parsing <identifier>: {n}");
        Ok(Identifier::new(n.clone()))
    }
}

fn token_eq(expected: Token, tokens: &mut Peekable<Iter<Token>>) -> Result<(), String> {
    let Some(t) = tokens.next() else {
        debug!("No more tokens available when expecting: {expected:?}");
        return Err(String::from("empty tokens"));
    };
    if *t != expected {
        debug!("Token mismatch - expected: {expected:?}, got: {t:?}");
        return Err(format!("expected {expected:?}, got {t:?}"));
    }
    debug!("Successfully matched token: {expected:?}");

    Ok(())
}
