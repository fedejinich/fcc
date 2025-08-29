use std::slice::Iter;

use log::{debug, trace};

use crate::{
    ast::program::{
        BinaryOperator, Expression, FunctionDefinition, Identifier, Program, Statement,
        UnaryOperator,
    },
    lexer::Token,
};

// ques: should i do a trait for 'from(tokens: Vec<Token>) -> ParseResult<Self>'?

type ParseResult<T> = Result<T, String>;

impl TryFrom<Vec<Token>> for Program {
    type Error = String;

    fn try_from(tokens: Vec<Token>) -> Result<Self, Self::Error> {
        trace!("Entering <program>");
        debug!("Starting parsing with {} tokens", tokens.len());
        trace!("Token stream: {:?}", tokens);
        debug!("Attempting to parse function definition");

        let mut iter = tokens.iter();
        let function_definition = FunctionDefinition::parse_fd(&mut iter)?;

        let program_ast = Program {
            function_definition,
        };

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
    fn parse_fd(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing <function>");

        token_eq(Token::Int, tokens)?;

        let identifier = Identifier::parse_id(tokens)?;
        debug!("Found <function>: {}", identifier.value);

        token_eq(Token::OpenParen, tokens)?;
        token_eq(Token::Void, tokens)?;
        token_eq(Token::CloseParen, tokens)?;
        token_eq(Token::OpenBrace, tokens)?;

        let body = Statement::parse_st(tokens)?;

        token_eq(Token::CloseBrace, tokens)?;

        trace!("<function> parsing completed successfully");
        Ok(FunctionDefinition {
            name: identifier,
            body,
        })
    }
}

impl Identifier {
    fn parse_id(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing <identifier>");

        if let Some(Token::Identifier(n)) = tokens.next() {
            trace!("Found <identifier>: {}", n);
            Ok(Identifier { value: n.clone() })
        } else {
            debug!("Expected <identifier> but found none");
            Err("could not parse identifier".to_string())
        }
    }
}

impl Statement {
    fn parse_st(tokens: &mut Iter<Token>) -> ParseResult<Vec<Self>> {
        trace!("Parsing <statement>");

        let mut statements = Vec::new();
        while let Some(t) = tokens.clone().next() {
            match t {
                Token::Return => {
                    let _ = tokens.next();

                    trace!("Found <statement>: return");

                    // start with a minimum precedence of zero so
                    // the result includes operators at every precedence level
                    let expr = Expression::parse_exp(tokens, 0)?;

                    token_eq(Token::Semicolon, tokens)?;

                    statements.push(Statement::Return(expr));
                }
                _ => break,
            };
        }

        if statements.is_empty() {
            return Err(String::from("could not parse any statement"));
        }

        trace!("Parsed <statement>");

        Ok(statements)
    }
}

impl Expression {
    fn parse_exp(tokens: &mut Iter<Token>, min_prec: i32) -> ParseResult<Self> {
        trace!("Entering <exp> with min_prec: {}", min_prec);
        let mut left = Expression::parse_fact(tokens)?;
        let mut next_token = tokens.clone().next().unwrap();

        // ques: isn't this too expensive?
        let is_binary_op = |t: &Token| {
            matches!(
                t, // ques: remove clone?
                Token::Plus
                    | Token::Negate
                    | Token::Multiply
                    | Token::Divide
                    | Token::Remainder
                    // bitwise operators are binary operators as well
                    | Token::And
                    | Token::Or
                    | Token::Xor
                    | Token::LeftShift
                    | Token::RightShift
            )
        };
        while is_binary_op(next_token) && precedence(next_token) >= min_prec {
            debug!(
                "Found <binop>: {:?} with precedence {}",
                next_token,
                precedence(next_token)
            );
            trace!("Parsing <binop>");
            let op = BinaryOperator::parse_bin(tokens)?;
            trace!(
                "Parsing right <exp> with precedence {}",
                precedence(next_token) + 1
            );
            let right = Expression::parse_exp(tokens, precedence(next_token) + 1)?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
            trace!("Created binary <exp>");
            next_token = tokens.clone().next().unwrap();
        }

        trace!("Exiting <exp>");
        Ok(left)
    }

    fn parse_fact(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Entering <factor>");
        let next_token = tokens.clone().next().unwrap();
        match next_token {
            Token::Constant(n) => {
                trace!("Parsed <int>: {}", n);
                let _ = tokens.next();
                trace!("Exiting <factor> (int)");
                Ok(Expression::Constant(n.parse::<i32>().unwrap()))
            }
            Token::Complement | Token::Negate => {
                trace!("Found <unop>: {:?}", next_token);
                let unary = UnaryOperator::parse_un(tokens)?;
                let exp = Expression::parse_fact(tokens)?;
                trace!("Exiting <factor> (unop)");
                Ok(Expression::Unary(unary, Box::new(exp)))
            }
            Token::OpenParen => {
                trace!("Found \"(\" - parsing parenthesized <exp>");
                let _ = tokens.next();
                let exp = Expression::parse_exp(tokens, 0)?;
                token_eq(Token::CloseParen, tokens)?;
                trace!("Exiting <factor> (parenthesized)");
                Ok(exp)
            }
            _ => {
                trace!("Exiting <factor> (error)");
                Err("could not parse expression".to_string())
            }
        }
    }
}

// returns value representing precedence order
// operators are sorted according to the official spec
// https://en.cppreference.com/w/c/language/operator_precedence.html
fn precedence(token: &Token) -> i32 {
    debug!("<precedence>: {:?}", token);
    match token {
        Token::Multiply | Token::Divide | Token::Remainder => 50,
        Token::Plus | Token::Negate => 45,
        Token::LeftShift | Token::RightShift => 44,
        Token::And => 43,
        Token::Xor => 42,
        Token::Or => 40,
        _ => 0,
    }
}

impl BinaryOperator {
    fn parse_bin(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        let binop = match tokens.next().unwrap() {
            Token::Plus => BinaryOperator::Add,
            Token::Multiply => BinaryOperator::Multiply,
            Token::Divide => BinaryOperator::Divide,
            Token::Remainder => BinaryOperator::Remainder,
            Token::Negate => BinaryOperator::Subtract,
            // bitwise operators are binary operators as well
            Token::And => BinaryOperator::And,
            Token::Or => BinaryOperator::Or,
            Token::Xor => BinaryOperator::Xor,
            Token::LeftShift => BinaryOperator::LeftShift,
            Token::RightShift => BinaryOperator::RightShift,
            _ => return Err("could not parse binary operator".to_string()),
        };

        trace!("Parsed <binop>: {:?}", binop);

        Ok(binop)
    }
}

impl UnaryOperator {
    fn parse_un(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        let unop = match tokens.next().unwrap() {
            Token::Complement => UnaryOperator::Complement,
            Token::Negate => UnaryOperator::Negate,
            _ => return Err("could not parse unary operator".to_string()),
        };

        trace!("Parsed <unop>: {:?}", unop);

        Ok(unop)
    }
}

fn token_eq(expected: Token, tokens: &mut Iter<Token>) -> Result<(), String> {
    if let Some(t) = tokens.next() {
        if *t != expected {
            debug!("Token mismatch - expected: {:?}, got: {:?}", expected, t);
            return Err(format!("expected {:?}, got {:?}", expected, t));
        }
        trace!("Successfully matched token: {:?}", expected);
        return Ok(());
    }

    debug!("No more tokens available when expecting: {:?}", expected);
    Err(String::from("empty tokens"))
}
