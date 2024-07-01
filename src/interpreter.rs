use std::collections::HashMap;

use crate::parser::{ASTNode, Operator};

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn interpret(&mut self, ast: &[ASTNode]) -> Result<(), String> {
        for node in ast {
            self.execute(node)?;
        }
        Ok(())
    }

    fn execute(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Number(n) => Ok(Value::Number(*n)),
            ASTNode::StringLiteral(s) => Ok(Value::String(s.to_string())),
            ASTNode::Variable(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            ASTNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left_val = self.execute(left)?;
                let right_val = self.execute(right)?;
                self.evaluate_binary_op(operator, left_val, right_val)
            }
            ASTNode::VariableDeclaration { variable, value } => {
                let val = self.execute(value)?;
                self.variables.insert(variable.clone(), val.clone());
                Ok(val)
            }
            ASTNode::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.execute(condition)?;
                match condition_value {
                    Value::Boolean(true) => {
                        for stmt in then_branch {
                            self.execute(stmt)?;
                        }
                    }
                    Value::Boolean(false) if else_branch.is_some() => {
                        for stmt in else_branch.as_ref().unwrap() {
                            self.execute(stmt)?;
                        }
                    }
                    Value::Boolean(false) => {}
                    _ => return Err("Condition must evaluate to a boolean".to_string()),
                }
                Ok(Value::Boolean(true)) // If statements always evaluate to true in this implementation
            }
            ASTNode::BooleanLiteral(b) => Ok(Value::Boolean(*b)),
        }
    }

    fn evaluate_binary_op(
        &self,
        operator: &Operator,
        left: Value,
        right: Value,
    ) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                return match operator {
                    Operator::Plus => Ok(Value::Number(l + r)),
                    Operator::Minus => Ok(Value::Number(l - r)),
                    Operator::Multiply => Ok(Value::Number(l * r)),
                    Operator::Divide => {
                        if r == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::Number(l / r))
                    }
                    Operator::LT => Ok(Value::Boolean(l < r)),
                    Operator::GT => Ok(Value::Boolean(l > r)),
                    Operator::LTE => Ok(Value::Boolean(l <= r)),
                    Operator::GTE => Ok(Value::Boolean(l >= r)),
                    Operator::EQ => Ok(Value::Boolean((l - r).abs() < f64::EPSILON)),
                    Operator::NEQ => Ok(Value::Boolean((l - r).abs() >= f64::EPSILON)),
                    _ => return Err(format!("Unknown operator for numbers: {:?}", operator)),
                };
            }
            (Value::Boolean(l), Value::Boolean(r)) => {
                let result = match operator {
                    Operator::And => l && r,
                    Operator::Or => l || r,
                    _ => return Err(format!("Unknown operator for booleans: {:?}", operator)),
                };
                Ok(Value::Boolean(result))
            }
            (Value::String(_), Value::String(_)) => todo!(),
            _ => Err("Type mismatch in binary operation".to_string()),
        }
    }

    fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

pub fn run(ast: Vec<ASTNode>) {
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(&ast) {
        Ok(()) => {
            println!("Interpretation successful.");
            println!("Variables: {:#?}", interpreter.variables);
        }
        Err(e) => println!("Error: {}", e),
    }
}
