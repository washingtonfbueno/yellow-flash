use std::fmt::Error;

use crate::{
    token::{DataType, TokenType},
    yf_error::{ErrorType, YFError},
};

use super::{
    data::Data,
    environment::Environment,
    expression::{BinaryOperator, Identifier, Literal, UnaryOperator},
    statement::{ExpressionStatement, PrintStatement, Statement, VariableDeclStatement},
};

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Statement>>) -> Result<(), YFError> {
        self.env.enter_scope();

        for stmt in statements.iter() {
            if let Err(error_type) = self.visit_statement(stmt) {
                return Err(YFError {
                    error_type,
                    line: 0,
                });
            }
        }

        self.env.exit_scope();

        return Ok(());
    }

    pub fn visit_statement(&mut self, stmt: &Box<dyn Statement>) -> Result<(), ErrorType> {
        stmt.accept_visitor(self)?;

        return Ok(());
    }

    pub fn visit_expr_statement(&mut self, stmt: &ExpressionStatement) -> Result<(), ErrorType> {
        stmt.value.accept_visitor(self)?;

        return Ok(());
    }

    pub fn visit_print_statement(&self, stmt: &PrintStatement) -> Result<(), ErrorType> {
        println!("{:?}", stmt.value.accept_visitor(self)?);

        return Ok(());
    }

    pub fn visit_variable_decl_statement(
        &mut self,
        stmt: &VariableDeclStatement,
    ) -> Result<(), ErrorType> {
        let data_type = &stmt.data_type;
        let data = stmt.data.accept_visitor(self)?;
        let identifier = stmt.identifier.clone();

        self.env.create_symbol(identifier, data)?;

        return Ok(());
    }

    pub fn visit_binary_operator(&self, node: &BinaryOperator) -> Result<Data, ErrorType> {
        let left = node.left.accept_visitor(self)?;
        let right = node.right.accept_visitor(self)?;
        let ty = &node.ty;

        use {Data as E, TokenType as T};
        match (ty, left, right) {
            (T::Plus, E::Number(l), E::Number(r)) => Ok(E::Number(l + r)),
            (T::Plus, E::String(l), E::String(r)) => Ok(E::String(format!("{l}{r}"))),
            (T::Minus, E::Number(l), E::Number(r)) => Ok(E::Number(l - r)),
            (T::Multiply, E::Number(l), E::Number(r)) => Ok(E::Number(l * r)),
            (T::Divide, E::Number(l), E::Number(r)) => Ok(E::Number(l / r)),
            (T::And, E::Boolean(l), E::Boolean(r)) => Ok(E::Boolean(l && r)),
            (T::Or, E::Boolean(l), E::Boolean(r)) => Ok(E::Boolean(l || r)),
            (T::Equal, E::Number(l), E::Number(r)) => Ok(E::Boolean(l == r)),
            (T::NotEqual, E::Number(l), E::Number(r)) => Ok(E::Boolean(l != r)),
            (T::Greater, E::Number(l), E::Number(r)) => Ok(E::Boolean(l > r)),
            (T::GreaterEqual, E::Number(l), E::Number(r)) => Ok(E::Boolean(l >= r)),
            (T::Less, E::Number(l), E::Number(r)) => Ok(E::Boolean(l < r)),
            (T::LessEqual, E::Number(l), E::Number(r)) => Ok(E::Boolean(l <= r)),
            _ => return Err(ErrorType::TypeError),
        }
    }

    pub fn visit_identifier(&self, node: &Identifier) -> Result<Data, ErrorType> {
        let Identifier { value, .. } = node;

        return self.env.get_symbol(&value);
    }

    pub fn visit_literal(&self, node: &Literal) -> Result<Data, ErrorType> {
        let ty = &node.ty;
        let value = &node.value;

        match ty {
            DataType::Number => Ok(Data::Number(value.parse::<f64>().unwrap())),
            DataType::String => Ok(Data::String(value.clone())),
            DataType::Boolean => Ok(Data::Boolean(value.parse::<bool>().unwrap())),
            _ => Err(ErrorType::TypeError),
        }
    }

    pub fn visit_unary_operator(&self, node: &UnaryOperator) -> Result<Data, ErrorType> {
        let ty = &node.ty;
        let right = node.right.accept_visitor(self)?;

        match (ty, right) {
            (TokenType::Minus, Data::Number(r)) => Ok(Data::Number(-r)),
            (TokenType::Not, Data::Boolean(r)) => Ok(Data::Boolean(!r)),
            _ => Err(ErrorType::TypeError),
        }
    }
}
