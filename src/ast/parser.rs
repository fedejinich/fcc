use std::slice::Iter;

use log::{debug, trace};

use crate::{
    ast::program::{
        BinaryOperator, Expression, FunctionDefinition, Identifier, Program, Statement,
        UnaryOperator,
    },
    lexer::Token,
};

type ParseResult<T> = Result<T, String>;

impl TryFrom<Vec<Token>> for Program {
    type Error = String;

    fn try_from(tokens: Vec<Token>) -> Result<Self, Self::Error> {
        debug!("Starting parsing with {} tokens", tokens.len());
        trace!("Token stream: {:?}", tokens);
        trace!("Parsing Program");
        debug!("Attempting to parse function definition");

        let mut iter = tokens.iter();
        let function_definition = FunctionDefinition::from(&mut iter)?;

        debug!(
            "Successfully parsed function definition: {}",
            function_definition.name.value
        );
        trace!("Program parsing completed");

        let program_ast = Program {
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

impl FunctionDefinition {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing FunctionDefinition");

        token_eq(Token::Int, tokens)?;

        debug!("Parsing function identifier");
        let identifier = Identifier::from(tokens)?;
        debug!("Found function: {}", identifier.value);

        token_eq(Token::OpenParen, tokens)?;
        token_eq(Token::Void, tokens)?;
        token_eq(Token::CloseParen, tokens)?;
        token_eq(Token::OpenBrace, tokens)?;

        debug!("Parsing function body statement");
        let body = Statement::from(tokens)?;

        token_eq(Token::CloseBrace, tokens)?;

        trace!("FunctionDefinition parsing completed successfully");
        Ok(FunctionDefinition {
            name: identifier,
            body,
        })
    }
}

impl Identifier {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing Identifier");

        if let Some(Token::Identifier(n)) = tokens.next() {
            trace!("Found identifier: {}", n);
            Ok(Identifier { value: n.clone() })
        } else {
            debug!("Expected identifier but found none");
            Err(String::from("expected identifier"))
        }
    }
}

impl Statement {
    fn from(tokens: &mut Iter<Token>) -> ParseResult<Vec<Self>> {
        trace!("Parsing Statement");

        let mut statements = Vec::new();
        while let Some(t) = tokens.clone().next() {
            match t {
                Token::Return => {
                    token_eq(Token::Return, tokens)?;

                    debug!("Parsing return expression");
                    // start with a minimum precedence of zero so
                    // the result includes operators at every precedence level
                    let expr = Expression::from(tokens, 0)?;

                    token_eq(Token::Semicolon, tokens)?;

                    statements.push(Statement::Return(expr));
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

impl Expression {
    // fn from(tokens: &mut Iter<Token>) -> ParseResult<Self> {
    //     trace!("Parsing Expression");
    //
    //     let next_token = tokens.clone().next().unwrap();
    //     match next_token {
    //         Token::Constant(n) => {
    //             trace!("Found integer constant: {}", n);
    //             let _ = tokens.next();
    //
    //             Ok(Expression::Constant(n.parse::<i32>().unwrap()))
    //         }
    //         Token::Complement | Token::Negate => {
    //             trace!("Found unary operator: {:?}", next_token);
    //             let unary = UnaryOperator::from(tokens)?;
    //             let exp = Expression::from(tokens)?;
    //
    //             Ok(Expression::Unary(unary, Box::new(exp)))
    //         }
    //         Token::OpenParen => {
    //             trace!("Found open parenthesis");
    //             let _ = tokens.next();
    //             let exp = Expression::from(tokens)?;
    //             token_eq(Token::CloseParen, tokens)?;
    //
    //             Ok(exp)
    //         }
    //         _ => Err("could not parse expression".to_string()),
    //     }
    // }
    fn from(tokens: &mut Iter<Token>, min_prec: i32) -> ParseResult<Self> {
        trace!("Parsing Expression");
        let mut left = Expression::from_factor(tokens)?;
        let mut next_token = tokens.clone().next().unwrap();

        let is_binary_op = match next_token {
            Token::Plus | Token::Negate | Token::Multiply | Token::Divide | Token::Remainder => {
                true
            }
            _ => false,
        };
        while is_binary_op && precedence(next_token) <= min_prec {
            let op = BinaryOperator::from(tokens)?;
            let right = Expression::from(tokens, precedence(next_token) + 1)?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
            next_token = tokens.clone().next().unwrap();
        }

        Ok(left)
    }

    fn from_factor(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        let next_token = tokens.clone().next().unwrap();
        match next_token {
            Token::Int => todo!(),
            Token::Complement | Token::Negate => {
                let op = UnaryOperator::from(tokens)?;
                let inner_exp = Expression::from_factor(tokens)?;

                Ok(Expression::Unary(op, Box::new(inner_exp)))
            }
            Token::OpenParen => {
                let _ = tokens.next();
                let inner_exp = Expression::from(tokens, 0)?;
                token_eq(Token::CloseParen, tokens)?;

                Ok(inner_exp)
            }
            _ => Err("malformed factor".to_string()),
        }
    }
}

fn precedence(token: &Token) -> i32 {
    match token {
        Token::Multiply | Token::Divide | Token::Remainder => 50,
        Token::Plus | Token::Negate => 45,
        _ => 0,
    }
}

impl BinaryOperator {
    fn from(tokens: &mut Iter<Token>) -> Result<BinaryOperator, String> {
        todo!()
    }
}

impl UnaryOperator {
    fn from(tokens: &mut Iter<Token>) -> Result<UnaryOperator, String> {
        trace!("Parsing UnaryOperator");

        match tokens.next().unwrap() {
            Token::Complement => Ok(UnaryOperator::Complement),
            Token::Negate => Ok(UnaryOperator::Negate),
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
