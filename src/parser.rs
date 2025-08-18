use std::slice::Iter;

use log::{debug, trace};

use crate::{ast::{CExpression, CFunctionDefinition, CIdentifier, CProgram, CStatement, CUnaryOperator}, lexer::Token};

type ParseResult<T> = Result<T, String>;

// AST PARSER

impl TryFrom<Vec<Token>> for CProgram {
    type Error = String;

    fn try_from(tokens: Vec<Token>) -> Result<Self, Self::Error> {
        debug!("Starting parsing with {} tokens", tokens.len());
        trace!("Token stream: {:?}", tokens);
        trace!("Parsing Program");
        debug!("Attempting to parse function definition");

        let mut iter = tokens.iter();
        let function_definition = CFunctionDefinition::from(&mut iter)?;

        debug!(
            "Successfully parsed function definition: {}",
            function_definition.name.value
        );
        trace!("Program parsing completed");

        let program_ast = CProgram {
            function_definition,
        };

        if iter.next().is_some() {
            return Err(format!(
                "unexpected tokens remaining: {:?}",
                iter.collect::<Vec<_>>()
            ));
        }

        debug!("Parsing completed successfully");

        Ok(program_ast)
    }
}

impl CFunctionDefinition {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing FunctionDefinition");

        token_eq(Token::Int, tokens)?;

        debug!("Parsing function identifier");
        let identifier = CIdentifier::from(tokens)?;
        debug!("Found function: {}", identifier.value);

        token_eq(Token::OpenParen, tokens)?;
        token_eq(Token::Void, tokens)?;
        token_eq(Token::CloseParen, tokens)?;
        token_eq(Token::OpenBrace, tokens)?;

        debug!("Parsing function body statement");
        let body = CStatement::from(tokens)?;

        token_eq(Token::CloseBrace, tokens)?;

        trace!("FunctionDefinition parsing completed successfully");
        Ok(CFunctionDefinition {
            name: identifier,
            body,
        })
    }
}

impl CIdentifier {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Self> {
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

impl CStatement {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Vec<Self>> {
        trace!("Parsing Statement");

        let mut statements = Vec::new();
        while let Some(t) = tokens.clone().next() {
            match t {
                Token::Return => {
                    token_eq(Token::Return, tokens)?;

                    debug!("Parsing return expression");
                    let expr = CExpression::from(tokens)?;

                    token_eq(Token::Semicolon, tokens)?;

                    statements.push(CStatement::Return(expr));
                }
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

impl CExpression {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing Expression");

        let next_token = tokens.clone().next().unwrap();
        match next_token {
            Token::Constant(n) => {
                trace!("Found integer constant: {}", n);
                let _ = tokens.next();

                Ok(CExpression::Constant(n.parse::<i32>().unwrap()))
            }
            Token::Complement | Token::Negate => {
                trace!("Found unary operator: {:?}", next_token);
                let unary = CUnaryOperator::from(tokens)?;
                let exp = CExpression::from(tokens)?;

                Ok(CExpression::Unary(unary, Box::new(exp)))
            }
            Token::OpenParen => {
                trace!("Found open parenthesis");
                let _ = tokens.next();
                let exp = CExpression::from(tokens)?;
                token_eq(Token::CloseParen, tokens)?;

                Ok(exp)
            }
            _ => Err("could not parse expression".to_string()),
        }
    }
}

impl CUnaryOperator {
    fn from(tokens: &mut Iter<Token>) -> Result<CUnaryOperator, String> {
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
