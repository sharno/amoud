use std::iter::Peekable;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,

    And,
    Or,

    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
}

#[derive(Debug)]
pub enum ASTNode {
    Number(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Variable(String),
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Vec<ASTNode>>,
    },
    BinaryOp {
        left: Box<ASTNode>,
        operator: Operator,
        right: Box<ASTNode>,
    },
    VariableDeclaration {
        variable: String,
        value: Box<ASTNode>,
    },
}

struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    fn new(tokens: I) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    fn parse(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut statements = Vec::new();
        while self.tokens.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.tokens.peek() {
            Some(Token::VariableKeyword) => self.parse_variable_declaration(),
            Some(Token::IfKeyword) => self.parse_if_statement(),
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Dot)?;
                Ok(expr)
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        self.tokens.next();
        if let Some(Token::Identifier(var_name)) = self.tokens.next() {
            self.expect(Token::Equals)?;
            let value = self.parse_expression()?;
            self.expect(Token::Dot)?;
            Ok(ASTNode::VariableDeclaration {
                variable: var_name,
                value: Box::new(value),
            })
        } else {
            Err("Expected identifier after 'متغير'".to_string())
        }
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_additive()?;

        loop {
            let op = match self.tokens.peek() {
                Some(Token::LT) => Operator::LT,
                Some(Token::GT) => Operator::GT,
                Some(Token::LTE) => Operator::LTE,
                Some(Token::GTE) => Operator::GTE,
                Some(Token::EQ) => Operator::EQ,
                Some(Token::NEQ) => Operator::NEQ,
                _ => break,
            };

            self.tokens.next(); // Consume the operator
            let right = self.parse_additive()?;
            expr = ASTNode::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_multiplicative()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = match self.tokens.next().unwrap() {
                        Token::Plus => Operator::Plus,
                        Token::Minus => Operator::Minus,
                        _ => unreachable!(),
                    };
                    let right = self.parse_multiplicative()?;
                    left = ASTNode::BinaryOp {
                        left: Box::new(left),
                        operator: op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Multiply | Token::Divide => {
                    let op = match self.tokens.next().unwrap() {
                        Token::Multiply => Operator::Multiply,
                        Token::Divide => Operator::Divide,
                        _ => unreachable!(),
                    };
                    let right = self.parse_primary()?;
                    left = ASTNode::BinaryOp {
                        left: Box::new(left),
                        operator: op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        match self.tokens.next() {
            Some(Token::Number(n)) => Ok(ASTNode::Number(n)),
            Some(Token::String(s)) => Ok(ASTNode::StringLiteral(s)),
            Some(Token::True) => Ok(ASTNode::BooleanLiteral(true)),
            Some(Token::False) => Ok(ASTNode::BooleanLiteral(false)),
            Some(Token::Identifier(name)) => Ok(ASTNode::Variable(name)),
            Some(Token::LeftParen) => {
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        self.tokens.next();
        let condition = self.parse_expression()?;
        self.expect(Token::ThenKeyword)?;

        let mut then_branch = Vec::new();
        while self.tokens.peek() != Some(&Token::ElseKeyword) && self.tokens.peek().is_some() {
            then_branch.push(self.parse_statement()?);
        }

        let else_branch = if self.tokens.peek() == Some(&Token::ElseKeyword) {
            self.tokens.next();
            let mut else_statements = Vec::new();
            while self.tokens.peek().is_some() {
                else_statements.push(self.parse_statement()?);
            }
            Some(else_statements)
        } else {
            None
        };

        Ok(ASTNode::IfStatement {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.tokens.next() == Some(expected.clone()) {
            Ok(())
        } else {
            Err(format!("Expected {:?}", expected))
        }
    }
}

pub fn run(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut parser = Parser::new(tokens.into_iter());
    match parser.parse() {
        Ok(ast) => ast,
        Err(e) => panic!("Error: {}", e),
    }
}
