use std::fmt::Display;
use std::fmt::Write;

use console::style;

use super::{token::Const, types::*};

impl Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s: String;
		f.write_str(match self {
			Self::Void => "void",
			Self::Bool => "bool",
			Self::Char => "char",
			Self::Int => "int",
			Self::Ptr(Ptr { base_type }) => {
				if f.alternate() {
					s = format!("pointer to: < {:#} >", base_type);
				} else {
					s = format!("pointer to: ---> {}", base_type);
				}
				&s
			}
			Self::Array(Array { length, base_type, size_expr: _ }) => {
				if f.alternate() {
					s = format!("array of < {:#} > with size {}", base_type, length);
				} else {
					s = format!("array [size: {}] of ---> {}", length, base_type);
				}
				&s
			}
			Self::Func(Func { return_type, param_list, is_variadic: _ }) => {
				if f.alternate() {
					s = format!("function returning < {:#} > with parameters: (", return_type);
					if let Some((first, remaining)) = param_list.split_first() {
						write!(s, "{:#}", first)?;
						for p in remaining {
							write!(s, ", {:#}", p)?;
						}
					}
					s.push(')');
				} else {
					s = "function (".to_string();
					if let Some((first, remaining)) = param_list.split_first() {
						write!(s, "{}", first)?;
						for p in remaining {
							write!(s, ", {}", p)?;
						}
					}
					write!(s, ") returning ---> {}", return_type)?;
				}
				&s
			}
		})
	}
}

impl From<Vec<Declaration>> for DeclarationList {
	fn from(l: Vec<Declaration>) -> Self {
		DeclarationList { list: (l) }
	}
}

impl Display for Declarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.value {
			Const::Empty => write!(f, "{}", self.name),
			_ => write!(f, "{} = {}", self.name, self.value),
		}
	}
}

impl Display for Parameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl Display for FunctionDefinition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} (", self.name)?;
		if !self.params.is_empty() {
			write!(f, "{}", self.params[0])?;
		}
		for param in self.params.iter().skip(1) {
			write!(f, ", {}", param)?;
		}
		writeln!(f, ");")?;
		for stmt in self.stmts.iter() {
			write!(f, "{}", stmt)?;
		}
		writeln!(f)
	}
}

impl Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Empty => writeln!(f, ";"),
			// Self::ExprStmt(expr) => writeln!(f, "{};", expr),
			// Self::ReturnStmt(expr) => writeln!(f, "return\n{};", expr),
		}
	}
}

#[derive(PartialEq, Eq)]
pub enum NodePos {
	Init,
	Top,
	Middle,
	Bottom,
}

impl NodePos {
	fn node_prelude(&self) -> &'static str {
		match self {
			NodePos::Init => "────",
			NodePos::Top => "┌───",
			NodePos::Middle => "├───",
			NodePos::Bottom => "└───",
		}
	}

	fn is_init(&self) -> bool {
		*self == NodePos::Init
	}
	fn is_top(&self) -> bool {
		*self == NodePos::Top
	}
	fn is_middle(&self) -> bool {
		*self == NodePos::Middle
	}
}

fn print_op(s: &mut String, prev: &str, pos: &NodePos, v: impl Display) {
	s.push_str(&style(prev).dim().to_string());
	s.push_str(&style(pos.node_prelude()).dim().to_string());
	s.push_str(&style(v.to_string().as_str()).blue().to_string());
	s.push('\n');
}

fn print_leaf(s: &mut String, prev: &str, pos: &NodePos, v: impl Display) {
	// if prev.is_empty() { s.push_str(&style(v.to_string().as_str()).green().to_string()); } else {
	s.push_str(&style(prev).dim().to_string());
	s.push_str(&style(pos.node_prelude()).dim().to_string());
	s.push_str(&style(v.to_string().as_str()).green().to_string());
	s.push('\n');
	// }
}

fn print_binary_node(s: &mut String, prev: &str, pos: &NodePos, v: impl Display, left: &Expr, right: &Expr) {
	let next_prefix_top = if pos.is_init() || pos.is_top() { "    " } else { "│   " };
	print_expr_tree(left, s, &(prev.to_owned() + next_prefix_top), &NodePos::Top);

	print_op(s, prev, pos, v);

	let next_prefix = if pos.is_top() || pos.is_middle() { "│   " } else { "    " };
	print_expr_tree(right, s, &(prev.to_owned() + next_prefix), &NodePos::Bottom);
}

fn print_func_args(s: &mut String, prev: &str, pos: &NodePos, expr: &[Expr]) {
	if let Some((last, elems)) = expr.split_last() {
		let next_prefix = prev.to_owned() + if pos.is_top() || pos.is_middle() { "│   " } else { "    " };
		for e in elems {
			print_expr_tree(e, s, &next_prefix, &NodePos::Middle);
		}
		print_expr_tree(last, s, &next_prefix, &NodePos::Bottom);
	}
}

fn print_expr_tree(this: &Expr, s: &mut String, prev: &str, pos: &NodePos) {
	match this {
		Expr::Const(v) => print_leaf(s, prev, pos, v),

		Expr::StringLiteral(v) => print_leaf(s, prev, pos, v),

		Expr::Id(v) => print_leaf(s, prev, pos, v),

		Expr::BinOp(BinOp { op, left, right }) => print_binary_node(s, prev, pos, op, left, right),

		Expr::AssignExpr(AssignExpr { left, assign, right }) => print_binary_node(s, prev, pos, assign, left, right),

		Expr::CommaExpr(CommaExpr { left, right }) => print_binary_node(s, prev, pos, ",", left, right),

		Expr::CondExpr(CondExpr { cond, left, right }) => {
			let prefix_str = if pos.is_top() || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(cond, s, &(prev.to_owned() + prefix_str), &NodePos::Top);

			print_op(s, prev, pos, "? :");

			let next_prefix = prev.to_owned() + if pos.is_top() || pos.is_middle() { "│   " } else { "    " };
			print_expr_tree(left, s, &next_prefix, &NodePos::Middle);
			print_expr_tree(right, s, &next_prefix, &NodePos::Bottom);
		}
		Expr::FunctionCall(expr, args) => {
			let prefix_str = if pos.is_top() || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(expr, s, &(prev.to_owned() + prefix_str), &NodePos::Top);
			print_op(s, prev, pos, "<fn>()");

			print_func_args(s, prev, pos, args)
		}
		Expr::Postfix(PostfixOP { op, expr }) => {
			print_op(s, prev, pos, op);

			let next_prefix = prev.to_owned() + if pos.is_top() { "│   " } else { "    " };
			print_expr_tree(expr, s, &next_prefix, &NodePos::Bottom);
		}
		Expr::UnaryOp(UnaryOp { op, expr }) => {
			print_op(s, prev, pos, op);
			let next_prefix = prev.to_owned() + if pos.is_top() { "│   " } else { "    " };
			print_expr_tree(expr, s, &next_prefix, &NodePos::Bottom);
		}
		Expr::MemberAccess(expr, field) => {
			let prefix_str = if pos.is_top() || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(expr, s, &(prev.to_owned() + prefix_str), &NodePos::Top);
			let op = ".".to_owned() + field;
			print_op(s, prev, pos, &op);
		}
		Expr::MemberAccessP(expr, field) => {
			let prefix_str = if pos.is_top() || prev.is_empty() { "    " } else { "│   " };
			print_expr_tree(expr, s, &(prev.to_owned() + prefix_str), &NodePos::Top);
			let op = "->".to_owned() + field;
			print_op(s, prev, pos, &op);
		}
	}
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut s = String::new();
		print_expr_tree(self, &mut s, "", &NodePos::Init);
		f.write_str(s.as_str())
	}
}
