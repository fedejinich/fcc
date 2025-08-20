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
        trace!("Parsing <program>");
        debug!("Attempting to parse function definition");

        let mut iter = tokens.iter();
        let function_definition = FunctionDefinition::parse_fd(&mut iter)?;

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
                    token_eq(Token::Return, tokens)?;

                    debug!("Parsing return expression");
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

        trace!("<statement> parsing completed");

        Ok(statements)
    }
}

impl Expression {
    fn parse_fact(tokens: &mut Iter<Token>) -> ParseResult<Self> {
        trace!("Parsing <factor>");

        let next_token = tokens.clone().next().unwrap();
        match next_token {
            Token::Constant(n) => {
                trace!("Found integer constant: {}", n);
                let _ = tokens.next();

                Ok(Expression::Constant(n.parse::<i32>().unwrap()))
            }
            Token::Complement | Token::Negate => {
                trace!("Found unary operator: {:?}", next_token);
                let unary = UnaryOperator::parse_un(tokens)?;
                let exp = Expression::parse_fact(tokens)?;

                Ok(Expression::Unary(unary, Box::new(exp)))
            }
            Token::OpenParen => {
                trace!("Found open parenthesis");
                let _ = tokens.next();
                let exp = Expression::parse_exp(tokens, 0)?;
                token_eq(Token::CloseParen, tokens)?;

                Ok(exp)
            }
            _ => Err("could not parse expression".to_string()),
        }
    }

    fn parse_exp(tokens: &mut Iter<Token>, min_prec: i32) -> ParseResult<Self> {
        trace!("Parsing <exp>");
        let mut left = Expression::parse_fact(tokens)?;
        let mut next_token = tokens.clone().next().unwrap();

        let is_binary_op = matches!(
            next_token,
            Token::Plus | Token::Negate | Token::Multiply | Token::Divide | Token::Remainder
        );
        while is_binary_op && precedence(next_token) <= min_prec {
            let op = BinaryOperator::parse_bin(tokens)?;
            let right = Expression::parse_exp(tokens, precedence(next_token) + 1)?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
            next_token = tokens.clone().next().unwrap();
        }

        Ok(left)
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
    fn parse_bin(tokens: &mut Iter<Token>) -> Result<BinaryOperator, String> {
        trace!("Parsing <binop>");
        match tokens.next().unwrap() {
            Token::Plus => {
                trace!("Found binary operator: +");
                Ok(BinaryOperator::Add)
            }
            Token::Multiply => {
                trace!("Found binary operator: *");
                Ok(BinaryOperator::Multiply)
            }
            Token::Divide => {
                trace!("Found binary operator: /");
                Ok(BinaryOperator::Divide)
            }
            Token::Remainder => {
                trace!("Found binary operator: %");
                Ok(BinaryOperator::Remainder)
            }
            Token::Negate => {
                trace!("Found binary operator: -");
                Ok(BinaryOperator::Subtract)
            }
            _ => Err("could not parse binary operator".to_string()),
        }
    }
}

impl UnaryOperator {
    fn parse_un(tokens: &mut Iter<Token>) -> Result<UnaryOperator, String> {
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
