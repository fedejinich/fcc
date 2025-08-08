use log::{debug, info, trace};
use std::slice::Iter;

use crate::lexer::Token;

pub struct Program {
    function_definition: FunctionDefinition,
}

pub struct FunctionDefinition {
    identifier: Identifier,
    body: Statement,
}

pub struct Identifier {
    name: String,
}

pub enum Statement {
    Return(Expression),
}

pub enum Expression {
    Constant(ConstantType),
}

pub enum ConstantType {
    Int(String), // todo(fede) this should be an i32
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Program, String> {
    debug!("Starting parsing with {} tokens", tokens.len());
    trace!("Token stream: {:?}", tokens);

    let tokens_iter = &mut tokens.iter();
    let result = Program::parse(tokens_iter);

    if tokens_iter.len() > 0 {
        return Err(format!(
            "unexpected tokens remaining: {:?}",
            tokens_iter.collect::<Vec<_>>()
        ));
    }

    match &result {
        Ok(_) => debug!("Parsing completed successfully"),
        Err(e) => debug!("Parsing failed with error: {}", e),
    }

    result
}

trait Parseable<T> {
    fn parse(tokens: &mut Iter<Token>) -> Result<T, String>;
}

impl Parseable<Program> for Program {
    fn parse(tokens: &mut Iter<Token>) -> Result<Program, String> {
        trace!("Parsing Program");
        debug!("Attempting to parse function definition");

        let function_definition = FunctionDefinition::parse(tokens)?;

        debug!(
            "Successfully parsed function definition: {}",
            function_definition.identifier.name
        );
        trace!("Program parsing completed");

        let program = Program {
            function_definition,
        };

        if tokens.next().is_some() {
            return Err(String::from("unexpected tokens remaining"));
        }

        Ok(program)
    }
}

impl Parseable<FunctionDefinition> for FunctionDefinition {
    fn parse(tokens: &mut Iter<Token>) -> Result<FunctionDefinition, String> {
        trace!("Parsing FunctionDefinition");

        expect(Token::Int, tokens)?;

        debug!("Parsing function identifier");
        let identifier = Identifier::parse(tokens)?;
        debug!("Found function: {}", identifier.name);

        expect(Token::OpenParen, tokens)?;
        expect(Token::Void, tokens)?;
        expect(Token::CloseParen, tokens)?;
        expect(Token::OpenBrace, tokens)?;

        debug!("Parsing function body statement");
        let body = Statement::parse(tokens)?;

        expect(Token::CloseBrace, tokens)?;

        trace!("FunctionDefinition parsing completed successfully");
        Ok(FunctionDefinition { identifier, body })
    }
}

impl Parseable<Identifier> for Identifier {
    fn parse(tokens: &mut Iter<Token>) -> Result<Identifier, String> {
        trace!("Parsing Identifier");

        if let Some(Token::Identifier(n)) = tokens.next() {
            trace!("Found identifier: {}", n);
            Ok(Identifier { name: n.clone() })
        } else {
            debug!("Expected identifier but found none");
            Err(String::from("expected identifier"))
        }
    }
}

impl Parseable<Statement> for Statement {
    fn parse(tokens: &mut Iter<Token>) -> Result<Statement, String> {
        trace!("Parsing Statement");

        expect(Token::Return, tokens)?;

        debug!("Parsing return expression");
        let expr = Expression::parse(tokens)?;

        expect(Token::Semicolon, tokens)?;

        trace!("Statement parsing completed");
        Ok(Statement::Return(expr))
    }
}

impl Parseable<Expression> for Expression {
    fn parse(tokens: &mut Iter<Token>) -> Result<Expression, String> {
        trace!("Parsing Expression");

        if let Some(Token::Constant(n)) = tokens.next() {
            trace!("Found integer constant: {}", n);
            Ok(Expression::Constant(ConstantType::Int(n.clone())))
        } else {
            debug!("Expected integer constant but found none");
            Err(String::from("expected int"))
        }
    }
}

fn expect(expected: Token, tokens: &mut Iter<Token>) -> Result<(), String> {
    debug!("Expecting token: {:?}", expected);

    if let Some(t) = tokens.next() {
        trace!("Found token: {:?}", t);
        if *t != expected {
            debug!("Token mismatch - expected: {:?}, got: {:?}", expected, t);
            return Err(format!("expected {:?}, got {:?}", expected, t));
        }
        debug!("Token matched successfully");
        return Ok(());
    }

    debug!("No more tokens available when expecting: {:?}", expected);
    Err(String::from("empty tokens"))
}
