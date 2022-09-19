use std::collections::HashMap;

use super::{
	errors::ParseError,
	token::Const,
	types::{Expr, Function, Statement, Variable},
};

pub struct Env {
	pub global_variables: HashMap<String, Variable>,
}

pub struct VM {
	functions: HashMap<String, Function>,
}

impl VM {
	pub fn new(functions: HashMap<String, Function>) -> Self {
		VM { functions }
	}

	pub fn eval_expr(&self, expr: &Expr, env: &mut Env) -> i64 {
		match expr {
			Expr::Const(Const::Empty) => 0,
			Expr::Const(Const::Integer(ints)) => ints.parse().unwrap_or(0),
			Expr::Const(Const::Character(c)) => *c as i64,
			_ => 0,
		}
	}

	pub fn eval_stmt(&self, stmt: &Statement, env: &mut Env) {
		match stmt {
			Statement::Empty => (),
			Statement::CompoundStmt(stmts) => {
				for stmt in stmts {
					self.eval_stmt(stmt, env)
				}
			}
			Statement::ExprStmt(expr) => {
				self.eval_expr(expr, env);
			}
			Statement::IfStmt(cond, then, maybe_else) => {
				let c = self.eval_expr(cond, env);
				if c != 0 {
					self.eval_stmt(then, env)
				} else {
					match maybe_else {
						None => (),
						Some(es) => self.eval_stmt(es, env),
					}
				}
			}
			Statement::ReturnStmt(expr) => {
				let _r = self.eval_expr(expr, env);
			}
			Statement::ForStmt(m_init, cond, expr3, body) => {}
		}
	}

	pub fn eval_func(&self, func: &Function, env: &mut Env) {
		self.eval_stmt(&func.stmts, env)
	}

	pub fn eval(&self, env: &mut Env) -> Result<(), ParseError> {
		match self.functions.get("main") {
			Some(main) => Ok(self.eval_func(main, env)),
			None => Err(ParseError::General("main not found")),
		}
	}
}
