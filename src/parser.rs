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
    Program::parse(&mut tokens.iter())
}

trait Parseable<T> {
    fn parse(tokens: &mut Iter<Token>) -> Result<T, String>;
}

impl Parseable<Program> for Program {
    fn parse(tokens: &mut Iter<Token>) -> Result<Program, String> {
        let function_definition = FunctionDefinition::parse(tokens)?;
        Ok(Program {
            function_definition,
        })
    }
}

impl Parseable<FunctionDefinition> for FunctionDefinition {
    fn parse(tokens: &mut Iter<Token>) -> Result<FunctionDefinition, String> {
        expect(Token::Constant(format!("int")), tokens);
        let identifier = Identifier::parse(tokens)?;
        expect(Token::OpenParen, tokens);
        expect(Token::Void, tokens);
        expect(Token::CloseParen, tokens);
        expect(Token::OpenBrace, tokens);
        let body = Statement::parse(tokens)?;
        expect(Token::CloseBrace, tokens);

        Ok(FunctionDefinition { identifier, body })
    }
}

impl Parseable<Identifier> for Identifier {
    fn parse(tokens: &mut Iter<Token>) -> Result<Identifier, String> {
        if let Some(Token::Identifier(n)) = tokens.next() {
            Ok(Identifier { name: n.clone() })
        } else {
            Err(String::from("expected identifier"))
        }
    }
}

impl Parseable<Statement> for Statement {
    fn parse(tokens: &mut Iter<Token>) -> Result<Statement, String> {
        expect(Token::Return, tokens);
        let expr = Expression::parse(tokens)?;
        expect(Token::Semicolon, tokens);

        Ok(Statement::Return(expr))
    }
}

impl Parseable<Expression> for Expression {
    fn parse(tokens: &mut Iter<Token>) -> Result<Expression, String> {
        if let Some(Token::Int(n)) = tokens.next() {
            Ok(Expression::Constant(ConstantType::Int(n.clone())))
        } else {
            Err(String::from("expected int"))
        }
    }
}

fn expect(expected: Token, tokens: &mut Iter<Token>) -> Result<(), String> {
    if let Some(t) = tokens.next() {
        if *t != expected {
            return Err(format!("expected {:?}, got {:?}", expected, t));
        }
        return Ok(());
    }

    Err(String::from("empty tokens"))
}
