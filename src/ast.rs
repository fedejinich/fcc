use log::{debug, trace};
use std::{fmt, slice::Iter};

use crate::lexer::Token;

#[allow(dead_code)]
pub struct CProgram {
    pub function_definition: CFunctionDefinition,
}

#[allow(dead_code)]
pub struct CFunctionDefinition {
    pub name: CIdentifier,
    pub body: Vec<CStatement>,
}

pub struct CIdentifier {
    pub id: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum CStatement {
    Return(CExpression),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum CExpression {
    Constant(i32),
}

pub fn generate_ast(tokens: Vec<Token>) -> Result<CProgram, String> {
    debug!("Starting parsing with {} tokens", tokens.len());
    trace!("Token stream: {:?}", tokens);

    let tokens_iter = &mut tokens.iter();
    let program_ast = CProgram::parse(tokens_iter);

    if tokens_iter.len() > 0 {
        return Err(format!(
            "unexpected tokens remaining: {:?}",
            tokens_iter.collect::<Vec<_>>()
        ));
    }

    match &program_ast {
        Ok(_) => debug!("Parsing completed successfully"),
        Err(e) => debug!("Parsing failed with error: {}", e),
    }

    program_ast
}

trait Parseable<T> {
    fn parse(tokens: &mut Iter<Token>) -> Result<T, String>;
}

impl Parseable<CProgram> for CProgram {
    fn parse(tokens: &mut Iter<Token>) -> Result<CProgram, String> {
        trace!("Parsing Program");
        debug!("Attempting to parse function definition");

        let function_definition = CFunctionDefinition::parse(tokens)?;

        debug!(
            "Successfully parsed function definition: {}",
            function_definition.name.id
        );
        trace!("Program parsing completed");
        let program = CProgram {
            function_definition,
        };

        if tokens.next().is_some() {
            return Err(String::from("unexpected tokens remaining"));
        }

        Ok(program)
    }
}

impl Parseable<CFunctionDefinition> for CFunctionDefinition {
    fn parse(tokens: &mut Iter<Token>) -> Result<CFunctionDefinition, String> {
        trace!("Parsing FunctionDefinition");

        expect(Token::Int, tokens)?;

        debug!("Parsing function identifier");
        let identifier = CIdentifier::parse(tokens)?;
        debug!("Found function: {}", identifier.id);

        expect(Token::OpenParen, tokens)?;
        expect(Token::Void, tokens)?;
        expect(Token::CloseParen, tokens)?;
        expect(Token::OpenBrace, tokens)?;

        println!("{}", tokens.len());

        debug!("Parsing function body statement");
        let body = CStatement::parse(tokens)?;

        println!("{}", tokens.len());

        expect(Token::CloseBrace, tokens)?;

        trace!("FunctionDefinition parsing completed successfully");
        Ok(CFunctionDefinition {
            name: identifier,
            body,
        })
    }
}

impl Parseable<CIdentifier> for CIdentifier {
    fn parse(tokens: &mut Iter<Token>) -> Result<CIdentifier, String> {
        trace!("Parsing Identifier");

        if let Some(Token::Identifier(n)) = tokens.next() {
            trace!("Found identifier: {}", n);
            Ok(CIdentifier { id: n.clone() })
        } else {
            debug!("Expected identifier but found none");
            Err(String::from("expected identifier"))
        }
    }
}

impl Parseable<Vec<CStatement>> for CStatement {
    fn parse(tokens: &mut Iter<Token>) -> Result<Vec<CStatement>, String> {
        trace!("Parsing Statement");

        let mut statements = Vec::new();
        while let Some(t) = tokens.clone().next() {
            match t {
                Token::Return => {
                    expect(Token::Return, tokens)?;

                    debug!("Parsing return expression");
                    let expr = CExpression::parse(tokens)?;

                    expect(Token::Semicolon, tokens)?;

                    statements.push(CStatement::Return(expr));
                }
                // try to match any other token statement
                _ => break,
            };
        }

        if statements.is_empty() {
            return Err(String::from("could not parse any statement"));
        }

        trace!("Statement parsing completed");

        Ok(statements)
    }
}

impl Parseable<CExpression> for CExpression {
    fn parse(tokens: &mut Iter<Token>) -> Result<CExpression, String> {
        trace!("Parsing Expression");

        if let Some(Token::Constant(n)) = tokens.next() {
            trace!("Found integer constant: {}", n);
            Ok(CExpression::Constant(n.parse::<i32>().unwrap()))
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

fn indent(s: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    s.lines()
        .map(|line| format!("{}{}", pad, line))
        .collect::<Vec<_>>()
        .join("\n")
}

impl fmt::Display for CProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program(")?;
        write!(f, "{}\n)", indent(&self.function_definition.to_string(), 4))
    }
}

impl fmt::Display for CFunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Function(")?;
        writeln!(f, "{}", indent(&format!("name=\"{}\",", self.name.id), 4))?;
        write!(
            f,
            "{}",
            indent(
                &format!(
                    "body={}",
                    self.body
                        .clone()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                4
            )
        )?;
        Ok(())
    }
}

impl fmt::Display for CStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CStatement::Return(expr) => {
                writeln!(f, "Return(")?;
                write!(f, "{}\n)", indent(&expr.to_string(), 4))
            }
        }
    }
}

impl fmt::Display for CExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CExpression::Constant(c) => write!(f, "{}", c),
        }
    }
}
