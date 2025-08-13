use log::{debug, trace};
use std::{fmt, slice::Iter};

use crate::{lexer::Token, util::indent};

// todo(fede) disable warnings for 'variables can be used directly in the `format!` string'

pub struct CProgram {
    pub function_definition: CFunctionDefinition,
}

pub struct CFunctionDefinition {
    pub name: CIdentifier,
    pub body: Vec<CStatement>,
}

pub struct CIdentifier {
    pub value: String, //  todo(fede) this is still weird as fuck
}

#[derive(Clone)]
pub enum CStatement {
    Return(CExpression),
}

#[derive(Clone)]
pub enum CExpression {
    Constant(i32),
    Unary(CUnaryOperator, Box<CExpression>),
}

#[derive(Clone)]
pub enum CUnaryOperator {
    Complement,
    Negate,
}

// todo(fede) this should be 'from' trait
pub trait Parseable<T> {
    fn parse(tokens: &mut Iter<Token>) -> Result<T, String>;
}

impl Parseable<CProgram> for CProgram {
    fn parse(tokens: &mut Iter<Token>) -> Result<CProgram, String> {
        debug!("Starting parsing with {} tokens", tokens.len());
        trace!("Token stream: {:?}", tokens);
        trace!("Parsing Program");
        debug!("Attempting to parse function definition");

        let function_definition = CFunctionDefinition::parse(tokens)?;

        debug!(
            "Successfully parsed function definition: {}",
            function_definition.name.value
        );
        trace!("Program parsing completed");

        let program_ast = CProgram {
            function_definition,
        };

        if tokens.next().is_some() {
            return Err(String::from("unexpected tokens remaining"));
        }

        if tokens.len() > 0 {
            return Err(format!(
                "unexpected tokens remaining: {:?}",
                tokens.collect::<Vec<_>>()
            ));
        }

        debug!("Parsing completed successfully");

        Ok(program_ast)
    }
}

impl Parseable<CFunctionDefinition> for CFunctionDefinition {
    fn parse(tokens: &mut Iter<Token>) -> Result<CFunctionDefinition, String> {
        trace!("Parsing FunctionDefinition");

        token_eq(Token::Int, tokens)?;

        debug!("Parsing function identifier");
        let identifier = CIdentifier::parse(tokens)?;
        debug!("Found function: {}", identifier.value);

        token_eq(Token::OpenParen, tokens)?;
        token_eq(Token::Void, tokens)?;
        token_eq(Token::CloseParen, tokens)?;
        token_eq(Token::OpenBrace, tokens)?;

        debug!("Parsing function body statement");
        let body = CStatement::parse(tokens)?;

        token_eq(Token::CloseBrace, tokens)?;

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
            Ok(CIdentifier { value: n.clone() })
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
                    token_eq(Token::Return, tokens)?;

                    debug!("Parsing return expression");
                    let expr = CExpression::parse(tokens)?;

                    token_eq(Token::Semicolon, tokens)?;

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

        // todo(fede) i'm supposing that there's always a next token
        // analyze this and leave a comment
        let next_token = tokens.clone().next().unwrap();
        match next_token {
            Token::Constant(n) => {
                trace!("Found integer constant: {}", n);
                let _ = tokens.next();

                Ok(CExpression::Constant(n.parse::<i32>().unwrap()))
            }
            Token::Complement | Token::Negate => {
                trace!("Found unary operator: {:?}", next_token);
                let unary = CUnaryOperator::parse(tokens)?;
                let exp = CExpression::parse(tokens)?;

                Ok(CExpression::Unary(unary, Box::new(exp)))
            }
            Token::OpenParen => {
                trace!("Found open parenthesis");
                let _ = tokens.next();
                let exp = CExpression::parse(tokens)?;
                token_eq(Token::CloseParen, tokens)?;

                Ok(exp)
            }
            _ => Err("could not parse expression".to_string()),
        }
    }
}

impl Parseable<CUnaryOperator> for CUnaryOperator {
    fn parse(tokens: &mut Iter<Token>) -> Result<CUnaryOperator, String> {
        trace!("Parsing UnaryOperator");

        match tokens.next().unwrap() {
            Token::Complement => Ok(CUnaryOperator::Complement),
            Token::Negate => Ok(CUnaryOperator::Negate),
            _ => Err("could not parse unary operator".to_string()),
        }
    }
}

fn token_eq(expected: Token, tokens: &mut Iter<Token>) -> Result<(), String> {
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

impl fmt::Display for CProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program(")?;
        write!(f, "{}\n)", indent(&self.function_definition.to_string(), 4))
    }
}

impl fmt::Display for CFunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Function(")?;
        writeln!(
            f,
            "{}",
            indent(&format!("name=\"{}\",", self.name.value), 4)
        )?;
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
            CExpression::Unary(u, e) => todo!(),
        }
    }
}
